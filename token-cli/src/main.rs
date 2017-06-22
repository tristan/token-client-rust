#![feature(plugin)]
#![plugin(mod_path)]

#[macro_use]
extern crate serde_derive;
extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate protobuf;
extern crate rusqlite;
extern crate rpassword;

extern crate eth;
#[macro_use]
extern crate signal;
extern crate token_services;

mod account;

mod storage;

//use signal::protocol::SignalProtocolStore;
use account::Account;
use account::AccountStore;
use storage::{SQLiteAccountStore};

use docopt::Docopt;

// alias token_servies
use token_services as service;
use signal::protocol::SignalProtocolStore;
use signal::sqlite_store::SQLiteProtocolStore;
use signal::message::TokenMessage;

#[derive(Debug, Deserialize)]
struct Args {
    arg_username: String,
    arg_recipient: String,
    arg_message: String,
    cmd_message: bool,
    cmd_review: bool,
    cmd_create: bool,
    cmd_info: bool,
    cmd_messages: bool,
    cmd_debug: bool,
    cmd_dump: bool,
    cmd_websocket: bool,
    arg_target: String,
    arg_rating: Option<f32>
}

const USAGE: &'static str = "
Usage:
  token_client <username> create
  token_client <username> message <recipient> <message>
  token_client <username> messages
  token_client <username> info <target>
  token_client <username> review <recipient> <rating> <message>
  token_client <username> websocket
  token_client <username> debug dump
";

const TOKEN_ID_SERVICE_URL: &'static str = "https://token-id-service-development.herokuapp.com";
const TOKEN_CHAT_SERVICE_URL: &'static str = "https://token-chat-service-development.herokuapp.com";
const TOKEN_REPUTATION_SERVICE_URL: &'static str = "https://token-rep-service-development.herokuapp.com";

const ACCOUNT_DB_NAME: &'static str = "token.db";
macro_rules! get_account_db_name{($username:expr) => (format!("user_{}.db", $username.as_str()))}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    let mut account_store = SQLiteAccountStore::new(ACCOUNT_DB_NAME);
    let user = match account_store.load_account(&args.arg_username) {
        Some(account) => {
            // TODO: refresh keys
            account
        },
        None => {
            if args.cmd_create {
                let account = Account::new(&args.arg_username);
                let db_name = get_account_db_name!(args.arg_username);
                if std::path::Path::new(&db_name).exists() {
                    println!("ERROR: {} already exists\n{}", db_name, USAGE);
                    std::process::exit(1);
                }
                let mut protocol_store = new_protocol_store!(
                    SQLiteProtocolStore::new(&db_name,
                                             account.get_identity_keypair(),
                                             account.get_registration_id()));
                account.initialize(&mut protocol_store, TOKEN_ID_SERVICE_URL, TOKEN_CHAT_SERVICE_URL)
                    .unwrap_or_else(|e| {
                        println!("ERROR: {}\n{}", e, USAGE);
                        std::process::exit(1);
                    });
                account_store.store_account(&account);

                // TODO: display account details
                std::process::exit(0);
            } else {
                println!("ERROR: User doesn't exist\n{}", USAGE);
                std::process::exit(1);
            }
        }
    };

    // COMMANDS

    // REVIEW

    if args.cmd_review {
        let rating = args.arg_rating.unwrap();
        if rating < 0.0 || rating > 5.0 {
            println!("Invalid rating: must be between 0 and 5\n{}", USAGE);
            std::process::exit(1);
        }

        let repservice = service::rep::ReputationService::new(
            TOKEN_REPUTATION_SERVICE_URL, &user.get_private_key());

        repservice.submit_review(&args.arg_recipient.as_str(),
                                 rating,
                                 &args.arg_message.as_str())
            .unwrap();
        println!("Review sumbitted!");
    }

    if args.cmd_info {
        match service::id::IdService::new(TOKEN_ID_SERVICE_URL, &user.get_private_key())
            .get_user_by_username(&args.arg_target) {
                Ok(data) => {
                    println!("{:#}", data);
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            };
    }

    if args.cmd_messages {
        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));
        let mut cs = service::chat::ChatService::new(
            &mut store,
            TOKEN_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_token_id(), &user.get_password());
        let result = cs.get_messages();
        match result {
            Ok(messages) => {
                if messages.len() == 0 {
                    println!("No new messages");
                }
                for message in messages {
                    println!("{}", message);
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    if args.cmd_message {
        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));
        let mut cs = service::chat::ChatService::new(
            &mut store,
            TOKEN_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_token_id(), &user.get_password());

        let token_id = match service::id::IdService::new(TOKEN_ID_SERVICE_URL, &user.get_private_key())
            .get_user_by_username(&args.arg_recipient) {
                Ok(data) => {
                    eth::Address::from_string(&data["token_id"].as_str().unwrap())
                },
                Err(_) => {
                    println!("ERROR: User '{}' doesn't exist\n{}", args.arg_recipient, USAGE);
                    std::process::exit(1);
                }
        };

        let sofa_message = format!("SOFA::Message:{{\"body\":\"{}\"}}", args.arg_message);
        println!("Sending message to: {}", token_id.to_string());

        match cs.send_message(&token_id, &sofa_message) {
            Ok(needs_sync) => {
                if needs_sync {
                    println!("Needs sync!");
                } else {
                    println!("Message sent successfully!");
                }
            },
            Err(e) => {
                println!("Error sending message: {:?}", e);
            }
        };
    }

    if args.cmd_debug && args.cmd_dump {
        println!("{}", user.get_token_id().to_string());
        println!("{}", user.get_registration_id());
        //println!("{:?}", user.get_signaling_key());
    }

    if args.cmd_websocket {
        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));
        if let Err(error) = service::chat::ChatService::new(
            &mut store,
            TOKEN_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_token_id(), &user.get_password()).websocket_connect(
            user.get_signaling_key(),
            |msg| {
                println!("{}", msg);
            }
        ) {

            println!("ERROR: {}", error);
        };
    }
}
