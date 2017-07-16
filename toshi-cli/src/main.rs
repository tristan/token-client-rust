#![feature(plugin)]
#![plugin(mod_path)]
#![feature(i128_type)]

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
extern crate toshi_services;

mod account;

mod storage;

//use signal::protocol::SignalProtocolStore;
use account::Account;
use account::AccountStore;
use storage::{SQLiteAccountStore};

use docopt::Docopt;

// alias toshi_servies
use toshi_services as service;
use signal::protocol::{SignalProtocolStore, SignalProtocolAddress};
use signal::sqlite_store::SQLiteProtocolStore;
use signal::message::ToshiMessage;

#[derive(Debug, Deserialize)]
struct Args {
    arg_username: String,
    arg_recipient: String,
    arg_message: String,
    cmd_message: bool,
    cmd_review: bool,
    cmd_create: bool,
    cmd_refresh_keys: bool,
    cmd_info: bool,
    cmd_messages: bool,
    cmd_debug: bool,
    cmd_dump: bool,
    cmd_websocket: bool,
    arg_target: String,
    arg_rating: Option<f32>,

    cmd_reprocess: bool,

    cmd_init: bool,
    cmd_command: bool,
    arg_value: String,

    cmd_pn_deregister: bool,
    arg_pn_service: String,

    cmd_pay: bool,
    cmd_fakepay: bool,
    arg_ether: Option<f64>,

    cmd_reset_session: bool,
    cmd_generate_mnemonic: bool
}

const USAGE: &'static str = "
Usage:
  toshi_client generate-mnemonic
  toshi_client <username> create
  toshi_client <username> refresh-keys
  toshi_client <username> reset-session <recipient>
  toshi_client <username> message <recipient> <message>
  toshi_client <username> messages
  toshi_client <username> info <target>
  toshi_client <username> review <recipient> <rating> <message>
  toshi_client <username> reprocess
  toshi_client <username> websocket
  toshi_client <username> pn-deregister <pn-service>
  toshi_client <username> debug dump
  toshi_client <username> init <recipient>
  toshi_client <username> command <recipient> <value>
  toshi_client <username> pay <recipient> <ether>
  toshi_client <username> fakepay <recipient> <ether>
";

const TOSHI_ID_SERVICE_URL: &'static str = "https://token-id-service-development.herokuapp.com";
const TOSHI_CHAT_SERVICE_URL: &'static str = "https://token-chat-service-development.herokuapp.com";
const TOSHI_REPUTATION_SERVICE_URL: &'static str = "https://token-rep-service-development.herokuapp.com";

//const TOSHI_ID_SERVICE_URL: &'static str = "https://identity.service.tokenbrowser.com";
//const TOSHI_CHAT_SERVICE_URL: &'static str = "https://chat.service.tokenbrowser.com";
//const TOSHI_REPUTATION_SERVICE_URL: &'static str = "https://reputation.service.tokenbrowser.com";

//const TOSHI_CHAT_SERVICE_URL: &'static str = "https://toshi-chat-service-staging.herokuapp.com";

// const TOSHI_CHAT_SERVICE_URL: &'static str = "http://localhost:5001";
// const TOSHI_ID_SERVICE_URL: &'static str = "http://localhost:5002";
// const TOSHI_REPUTATION_SERVICE_URL: &'static str = "http://localhost:5004";

const ACCOUNT_DB_NAME: &'static str = "toshi.db";
macro_rules! get_account_db_name{($username:expr) => (format!("user_{}.db", $username.as_str()))}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    if args.cmd_generate_mnemonic {
        let wallet = eth::wallet::HDWallet::new(&"");
        println!("{}", wallet.mnemonic());
        std::process::exit(0);
    }

    let mut account_store = SQLiteAccountStore::new(ACCOUNT_DB_NAME);
    let user = match account_store.load_account(&args.arg_username) {
        Some(account) => {
            // TODO: refresh keys
            if args.cmd_create {
                println!("ERROR: user already registered");
                std::process::exit(1);
            }
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
                // --- TODO make registration with services an option
                //account_store.store_account(&account);
                //std::process::exit(0);
                // ---
                let mut protocol_store = new_protocol_store!(
                    SQLiteProtocolStore::new(&db_name,
                                             account.get_identity_keypair(),
                                             account.get_registration_id()));
                account.initialize(&mut protocol_store, TOSHI_ID_SERVICE_URL, TOSHI_CHAT_SERVICE_URL)
                    .unwrap_or_else(|e| {
                        println!("ERROR: {}\n{}", e, USAGE);
                        std::process::exit(1);
                    });
                // account.create(&mut protocol_store, TOSHI_CHAT_SERVICE_URL).unwrap_or_else(|e| {
                //     println!("ERROR: {}\n{}", e, USAGE);
                //     std::process::exit(1);
                // });
                account_store.store_account(&account);

                println!("{}", account.get_toshi_id().to_string());
                println!("{}", account.get_registration_id());
                // TODO: display account details
                std::process::exit(0);
            } else {
                println!("ERROR: User doesn't exist\n{}", USAGE);
                std::process::exit(1);
            }
        }
    };

    // COMMANDS

    // REFRESH KEYS
    if args.cmd_refresh_keys {
        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));
        user.refresh_keys(&mut store, TOSHI_CHAT_SERVICE_URL)
            .unwrap_or_else(|e| {
                println!("ERROR: {}\n{}", e, USAGE);
                std::process::exit(1);
            });
    }

    // REVIEW

    if args.cmd_review {
        let rating = args.arg_rating.unwrap();
        if rating < 0.0 || rating > 5.0 {
            println!("Invalid rating: must be between 0 and 5\n{}", USAGE);
            std::process::exit(1);
        }

        let repservice = service::rep::ReputationService::new(
            TOSHI_REPUTATION_SERVICE_URL, &user.get_private_key());

        repservice.submit_review(&args.arg_recipient.as_str(),
                                 rating,
                                 &args.arg_message.as_str())
            .unwrap();
        println!("Review sumbitted!");
    }

    if args.cmd_reprocess {

        let repservice = service::rep::ReputationService::new(
            TOSHI_REPUTATION_SERVICE_URL, &user.get_private_key());

        repservice.reporcess_all_reviews().unwrap();
        println!("Reprocessing Started!");
    }

    if args.cmd_info {
        match service::id::IdService::new(TOSHI_ID_SERVICE_URL, &user.get_private_key())
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
            TOSHI_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_toshi_id(), &user.get_password());
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

    if args.cmd_message || args.cmd_init || args.cmd_command || args.cmd_fakepay {
        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));
        let mut cs = service::chat::ChatService::new(
            &mut store,
            TOSHI_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_toshi_id(), &user.get_password());

        let (toshi_id, payment_address) = match service::id::IdService::new(TOSHI_ID_SERVICE_URL, &user.get_private_key())
            .get_user_by_username(&args.arg_recipient) {
                Ok(data) => {
                    (eth::Address::from_string(&data["toshi_id"].as_str().unwrap()),
                     eth::Address::from_string(&data["payment_address"].as_str().unwrap()))
                },
                Err(_) => {
                    println!("ERROR: User '{}' doesn't exist\n{}", args.arg_recipient, USAGE);
                    std::process::exit(1);
                }
            };

        let sofa_message = if args.cmd_message {
            format!("SOFA::Message:{{\"body\":\"{}\"}}", args.arg_message)
        } else if args.cmd_init {
            format!("SOFA::Init:{{\"language\":\"en\",\"paymentAddress\":\"{}\"}}",
                    user.get_private_key().address().to_string())
        } else if args.cmd_command {
            format!("SOFA::Command:{{\"body\":\"\",\"value\":\"{}\"}}", args.arg_value)
        } else if args.cmd_fakepay {
            let tx_hash = "64b33f6874d011502f06a51b1e1f0d79dbb84386d09b0a2ac49e41b7d2e014fb";
            let value = (args.arg_ether.unwrap() * 1_000000000000000000.0) as u128;

            format!("SOFA::Payment:{{\"txHash\":\"0x{}\",\"fromAddress\":\"{}\",\"toAddress\":\"{}\",\"value\":\"0x{:x}\"}}",
                    tx_hash, user.get_private_key().address().to_string(), payment_address.to_string(), value)

        } else {
            panic!("Unreachable!");
        };

        println!("Sending '{}' to: {}", sofa_message, toshi_id.to_string());

        match cs.send_message(&toshi_id, &sofa_message) {
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

    // reset the session for a user
    if args.cmd_reset_session {
        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));
        let mut cs = service::chat::ChatService::new(
            &mut store,
            TOSHI_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_toshi_id(), &user.get_password());

        let (toshi_id, _) = match service::id::IdService::new(TOSHI_ID_SERVICE_URL, &user.get_private_key())
            .get_user_by_username(&args.arg_recipient) {
                Ok(data) => {
                    (eth::Address::from_string(&data["toshi_id"].as_str().unwrap()),
                     eth::Address::from_string(&data["payment_address"].as_str().unwrap()))
                },
                Err(_) => {
                    println!("ERROR: User '{}' doesn't exist\n{}", args.arg_recipient, USAGE);
                    std::process::exit(1);
                }
            };

        let address = SignalProtocolAddress::new(&toshi_id.to_string(), 1);
        if store.contains_session(&address) {
            store.delete_session(&address);
        }
    }

    if args.cmd_pn_deregister {
        let servicetype = if args.arg_pn_service == "apn" {
            service::chat::PushServiceType::APN
        } else if args.arg_pn_service == "gcm" {
            service::chat::PushServiceType::GCM
        } else {
            println!("Invalid PN service type");
            std::process::exit(1);
        };

        let mut store = new_protocol_store!(
            SQLiteProtocolStore::new(&get_account_db_name!(user.get_username()),
                                     user.get_identity_keypair(),
                                     user.get_registration_id()));

        let mut cs = service::chat::ChatService::new(
            &mut store,
            TOSHI_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_toshi_id(), &user.get_password());
        match cs.deregister_push_notifications(servicetype) {
            Ok(()) => { println!("OK!"); },
            Err(e) => {
                println!("Error deregistering: {:?}", e);
            }
        }
    }

    if args.cmd_debug && args.cmd_dump {
        println!("{}", user.get_toshi_id().to_string());
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
            TOSHI_CHAT_SERVICE_URL, &user.get_private_key(),
            user.get_toshi_id(), &user.get_password()).websocket_connect(
            user.get_signaling_key(),
            |msg| {
                println!("{}", msg);
            }
        ) {

            println!("ERROR: {}", error);
        };
    }
}
