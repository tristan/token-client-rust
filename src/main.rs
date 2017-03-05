extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate protobuf;
extern crate rusqlite;
extern crate rpassword;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate json;

mod curve;
#[macro_use]
mod keys;
mod eth;
mod services;
mod signal;
mod storage;

use ::keys::{IdentityKeyPair, PreKeyRecord, SignedPreKeyRecord};
use rand::{OsRng, Rng};

use rusqlite::{Connection, Error as SQLiteError};
use docopt::Docopt;

#[derive(Debug, RustcDecodable)]
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
  token_client <username> debug dump
";

const TOKEN_ID_SERVICE_URL: &'static str = "https://token-id-service-development.herokuapp.com";
const TOKEN_CHAT_SERVICE_URL: &'static str = "https://token-chat-service-development.herokuapp.com";
const TOKEN_REPUTATION_SERVICE_URL: &'static str = "https://token-rep-service-development.herokuapp.com";

struct User {
    username: String,
    address: String,
    registration_id: u32,
    ethsecretkey: eth::SecretKey,
    identitykeypair: IdentityKeyPair,
    signed_prekey: SignedPreKeyRecord,
    #[allow(dead_code)]
    device_id: u32,
    signaling_key: [u8;52],
    last_pre_key_id: u32,
    last_signed_pre_key_id: u32,
    password: String
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    let mut rng = OsRng::new().ok().unwrap();

    let mut con = Connection::open("token.db").unwrap();
    con.execute("CREATE TABLE IF NOT EXISTS tokenids (
                 address TEXT PRIMARY KEY,
                 username TEXT UNIQUE,
                 password TEXT,
                 registration_id INTEGER,
                 device_id INTEGER DEFAULT 1,
                 ethsecretkey BLOB,
                 signaling_key BLOB,
                 identitykeypair BLOB,
                 signed_prekey BLOB,
                 last_pre_key_id INTEGER DEFAULT 0,
                 last_signed_pre_key_id INTEGER DEFAULT 0
                 );", &[]).unwrap();
    con.execute("CREATE TABLE IF NOT EXISTS signal_pre_keys (
                 address TEXT,
                 key_id INTEGER,
                 keypair BLOB,
                 last_resort INTEGER DEFAULT 0,
                 PRIMARY KEY (address, key_id)
                 );", &[]).unwrap();

    let userresult = con.query_row(
        "SELECT username, address, password,
         registration_id, device_id, ethsecretkey,
         signaling_key, identitykeypair, signed_prekey,
         last_pre_key_id, last_signed_pre_key_id
         FROM tokenids WHERE username = $1",
        &[&args.arg_username], |row| {
            let ethvec: Vec<u8> = row.get(5);
            let sigvec: Vec<u8> = row.get(6);
            let idvec: Vec<u8> = row.get(7);
            let spkvec: Vec<u8> = row.get(8);
            let mut sigarr: [u8;52] = [0; 52];
            sigarr.copy_from_slice(&sigvec);
            User {
                username: row.get(0),
                address: row.get(1),
                password: row.get(2),
                registration_id: row.get(3),
                device_id: row.get(4),
                ethsecretkey: eth::SecretKey::deserialize(&ethvec),
                identitykeypair: IdentityKeyPair::deserialize(&idvec),
                signed_prekey: SignedPreKeyRecord::deserialize(&spkvec),
                last_pre_key_id: row.get(9),
                signaling_key: sigarr,
                last_signed_pre_key_id: row.get(10)
            }
        });
    let user = match userresult {
        Ok(user) => {
            if args.cmd_create {
                println!("ERROR: User already exists...\n{}", USAGE);
                std::process::exit(1);
            }
            user
        },
        Err(e) => {
            // make sure the error is something expected
            match e {
                SQLiteError::QueryReturnedNoRows => {},
                _ => {
                    println!("UNEXPECTED ERROR: {:?}", e);
                    std::process::exit(1);
                }
            };

            if args.cmd_create {
                println!("Enter in a password for the new user:");
                //let pass = rpassword::read_password().unwrap();
                let pass = "testing".to_string();
                // TODO: generate address
                let ethsecretkey = eth::generate_secret_key();
                let address = ethsecretkey.address();
                // TODO: register with id service (to verify username)
                let mut signaling_key = [0u8; 52];
                rng.fill_bytes(&mut signaling_key);
                let identitykeypair = IdentityKeyPair::generate();
                let me = User {
                    username: args.arg_username,
                    address: format!("0x{:x}", address).to_string(),
                    registration_id: rng.gen_range(1, 16381),
                    ethsecretkey: ethsecretkey,
                    password: pass,
                    identitykeypair: identitykeypair,
                    signed_prekey: SignedPreKeyRecord::generate(&identitykeypair, 0),
                    last_pre_key_id: 100,
                    signaling_key: signaling_key,
                    device_id: 1,
                    last_signed_pre_key_id: 0
                };
                match services::IdService::new(TOKEN_ID_SERVICE_URL, &me.ethsecretkey)
                    .create_user(&me.username, &address) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Unable to create user: {:?}", e);
                            std::process::exit(1);
                        }
                    }
                // generate keys for new user
                let pre_keys = PreKeyRecord::generate_prekeys(0, 100);
                // TODO: this seems to be what the java client always sets, figure out why
                let last_resort_key = PreKeyRecord::generate(16777215);

                match services::ChatService::new(TOKEN_CHAT_SERVICE_URL, &me.ethsecretkey, &me.address, &me.password)
                    .bootstrap_account(&me.identitykeypair,
                                       &last_resort_key,
                                       &pre_keys,
                                       &me.signed_prekey,
                                       me.registration_id,
                                       &me.signaling_key) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Unable to create user: {:?}", e);
                            std::process::exit(1);
                        }
                    };

                let tx = con.transaction().unwrap();
                tx.execute("INSERT INTO tokenids (username, address, password, registration_id, ethsecretkey,
                                                  signaling_key, identitykeypair, signed_prekey,
                                                  last_pre_key_id, last_signed_pre_key_id)
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                            &[&me.username, &me.address, &me.password, &me.registration_id,
                              &me.ethsecretkey.serialize(),
                              &me.signaling_key.to_vec(),
                              &me.identitykeypair.serialize(),
                              &me.signed_prekey.serialize(),
                              &me.last_pre_key_id, &me.last_signed_pre_key_id]).unwrap();
                for key in pre_keys {
                    tx.execute("INSERT INTO signal_pre_keys (address, key_id, keypair)
                                VALUES (?1, ?2, ?3)",
                               &[&me.address, &key.get_id(), &key.serialize()]).unwrap();
                }
                tx.execute("INSERT INTO signal_pre_keys (address, key_id, keypair, last_resort)
                            VALUES (?1, ?2, ?3, 1)",
                           &[&me.address, &last_resort_key.get_id(), &last_resort_key.serialize()]).unwrap();
                tx.commit().unwrap();

                me
            } else {
                println!("Could not find user named: \"{}\"\n{}", args.arg_username, USAGE);
                std::process::exit(1);
            }
        }
    };

    let prekey_count: u32 =
        con.query_row("SELECT COUNT(*) FROM signal_pre_keys WHERE address = $1",
                      &[&user.address], |row| {
                          row.get(0)
                      }).unwrap();
    if prekey_count < 10 {
        let pre_keys = PreKeyRecord::generate_prekeys(user.last_pre_key_id, 100);
        // TODO: push new keys to server
        let tx = con.transaction().unwrap();
        for key in pre_keys {
            tx.execute("INSERT INTO signal_pre_keys (address, key_id, keypair)
                        VALUES (?1, ?2, ?3)",
                       &[&user.address,
                         &key.get_id(), &key.serialize()]).unwrap();
        }
        tx.execute("UPDATE tokenids SET last_pre_key_id = $1 WHERE address = $2",
                   &[&(user.last_pre_key_id + 100), &user.address]).unwrap();
        tx.commit().unwrap();
    }

    // COMMANDS

    // REVIEW

    if args.cmd_review {
        let rating = args.arg_rating.unwrap();
        if rating < 0.0 || rating > 5.0 {
            println!("Invalid rating: must be between 0 and 5\n{}", USAGE);
            std::process::exit(1);
        }

        let repservice = services::ReputationService::new(
            TOKEN_REPUTATION_SERVICE_URL, &user.ethsecretkey);

        repservice.submit_review(&args.arg_recipient.as_str(),
                                 rating,
                                 &args.arg_message.as_str())
            .unwrap();
        println!("Review sumbitted!");
    }

    if args.cmd_info {
        match services::IdService::new(TOKEN_ID_SERVICE_URL, &user.ethsecretkey)
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
        let cs = services::ChatService::new(TOKEN_CHAT_SERVICE_URL, &user.ethsecretkey, &user.address, &user.password);
        let result = cs.get_messages();
        match result {
            Ok(data) => {
                println!("{:#}", data);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    if args.cmd_debug && args.cmd_dump {
        println!("{}", user.address);
        println!("{}", user.registration_id);
        //println!("identity_keypair: {:?}", user.identitykeypair.serialize());
        //println!("signed_pre_keypair: {:?}", user.signed_prekey.serialize());
        let mut stmt = con.prepare(
            "SELECT key_id, keypair FROM signal_pre_keys WHERE address = $1 ORDER BY key_id ASC LIMIT 10"
        ).unwrap();
        let rows = stmt.query_map(&[&user.address], |row| {
            let id: u32 = row.get(0);
            let keypair: Vec<u8> = row.get(1);
            (id, keypair)
        }).unwrap();
        for result in rows {
            let (id, keypair) = result.unwrap();
            println!("let pre_key = PreKeyRecord::deserialize(\n&vec!{:?});", keypair);
            println!("store.store_pre_key({}, &pre_key);", id);
        }
    }
}
