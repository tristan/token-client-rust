extern crate rustc_serialize;
extern crate docopt;
extern crate rand;
extern crate protobuf;
extern crate rusqlite;

#[macro_use]
extern crate lazy_static;

mod LocalStorageProtocol;
mod curve;
mod keys;
mod eth;

use ::keys::{IdentityKeyPair, PreKeyRecord};
use rand::{OsRng, Rng};

use rusqlite::{Connection};
use docopt::Docopt;

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_username: String,
    arg_recipient: String,
    arg_message: String,
    cmd_message: bool
}

const USAGE: &'static str = "
Usage:
  token_client <username>
  token_client <username> message <recipient> <message>
";

struct User {
    username: String,
    address: String,
    registration_id: u32,
    ethsecretkey: eth::SecretKey,
    identitykeypair: IdentityKeyPair,
    last_pre_key_id: u32,
    last_signed_pre_key_id: u32
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    let mut rng = OsRng::new().ok().unwrap();

    let mut con = Connection::open("token.db").unwrap();
    con.execute("CREATE TABLE IF NOT EXISTS tokenids (
                 address TEXT PRIMARY KEY,
                 username TEXT UNIQUE,
                 registration_id INTEGER,
                 ethsecretkey BLOB,
                 identitykeypair BLOB,
                 last_pre_key_id INTEGER DEFAULT 0,
                 last_signed_pre_key_id INTEGER DEFAULT 0
                 );", &[]).unwrap();
    con.execute("CREATE TABLE IF NOT EXISTS signal_pre_keys (
                 address TEXT,
                 registration_id INTEGER,
                 key_id INTEGER,
                 keypair BLOB,
                 PRIMARY KEY (address, registration_id, key_id)
                 );", &[]).unwrap();
    con.execute("CREATE TABLE IF NOT EXISTS signal_signed_pre_keys (
                 address TEXT,
                 registration_id INTEGER,
                 key_id INTEGER,
                 keypair BLOB,
                 PRIMARY KEY (address, registration_id, key_id)
                 );", &[]).unwrap();

    let userresult = con.query_row(
        "SELECT username, address, registration_id, ethsecretkey, identitykeypair,
         last_pre_key_id, last_signed_pre_key_id
         FROM tokenids WHERE username = $1",
        &[&args.arg_username], |row| {
            let ethvec: Vec<u8> = row.get(3);
            let idvec: Vec<u8> = row.get(4);
            User {
                username: row.get(0),
                address: row.get(1),
                registration_id: row.get(2),
                ethsecretkey: eth::SecretKey::deserialize(&ethvec),
                identitykeypair: IdentityKeyPair::deserialize(&idvec),
                last_pre_key_id: row.get(5),
                last_signed_pre_key_id: row.get(6)
            }
        });
    let user = match userresult {
        Ok(user) => user,
        Err(_) => {
            println!("Creating new user...");
            // TODO: generate address
            let ethsecretkey = eth::generate_secret_key();
            let address = format!("{:x}", ethsecretkey.address());
            // TODO: register with id service (to verify username)
            let me = User {
                username: args.arg_username,
                address: address.to_string(),
                registration_id: rng.gen_range(1, 16381),
                ethsecretkey: ethsecretkey,
                identitykeypair: IdentityKeyPair::generate(),
                last_pre_key_id: 0,
                last_signed_pre_key_id: 0
            };
            con.execute("INSERT INTO tokenids (username, address, registration_id, ethsecretkey, identitykeypair)
                         VALUES (?1, ?2, ?3, ?4, ?5)",
                        &[&me.username, &me.address, &me.registration_id,
                          &me.ethsecretkey.serialize(),
                          &me.identitykeypair.serialize()]).unwrap();
            me
        }
    };

    let prekey_count: u32 =
        con.query_row("SELECT COUNT(*) FROM signal_pre_keys WHERE address = $1 and registration_id = $2",
                      &[&user.address, &user.registration_id], |row| {
                          row.get(0)
                      }).unwrap();
    if prekey_count < 10 {
        let tx = con.transaction().unwrap();
        for key in PreKeyRecord::generate_prekeys(user.last_pre_key_id, 100) {
            tx.execute("INSERT INTO signal_pre_keys (address, registration_id, key_id, keypair)
                        VALUES (?1, ?2, ?3, ?4)",
                       &[&user.address, &user.registration_id,
                         &key.get_id(), &key.serialize()]).unwrap();
        }
        tx.execute("UPDATE tokenids SET last_pre_key_id = $1 WHERE address = $2",
                   &[&(user.last_pre_key_id + 100), &user.address]).unwrap();
        tx.commit().unwrap();
    }
}
