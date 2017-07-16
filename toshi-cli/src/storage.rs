use rusqlite::{Connection}; //, Error, ErrorCode};
use signal::protocol::{SignalProtocolAddress,
                       SignalProtocolStoreImpl,IdentityKeyStore,
                       SessionStore,PreKeyStore,SignedPreKeyStore};
use signal::state::{PreKeyRecord,SignedPreKeyRecord};
use signal::ratchet::{SessionRecord};
use signal::keys::{ECPublicKey,IdentityKeyPair};
use account::{Account,AccountStore};

pub struct SQLiteAccountStore(String);

impl SQLiteAccountStore {
    pub fn new(database: &str) -> SQLiteAccountStore {
        let store = SQLiteAccountStore(database.to_string());
        store.create_tables();
        store
    }
    fn create_tables(&self) {
        let con = self.open();
        // identity store
        con.execute(
            "CREATE TABLE IF NOT EXISTS account_store (
                 username TEXT,
                 account BLOB,
                 PRIMARY KEY (username)
                 );", &[]).unwrap();
    }
    fn open(&self) -> Connection {
        Connection::open(self.0.clone()).unwrap()
    }
}

impl AccountStore for SQLiteAccountStore {
    fn load_account(&self, username: &str) -> Option<Account> {
        let con = self.open();
        let rval = con.query_row(
            "SELECT account FROM account_store WHERE username = $1",
            &[&username],
            |row| {
                let record: Vec<u8> = row.get(0);
                Account::deserialize(&record)
            });
        match rval {
            Ok(record) => {
                Some(record)
            },
            Err(_) => None
        }
    }
    fn store_account(&mut self, account: &Account) {
        let con = self.open();
        let res = con.execute(
            "INSERT INTO account_store (username, account)
             VALUES ($1, $2)",
            &[account.get_username(), &account.serialize()]);
        match res {
            Ok(_) => {},
            Err(e) => {
                // make sure the only error we count as ok is
                // a constraint violation
                // match e {
                //     Error::SqliteFailure(f, _) => {
                //         match f.code {
                //             ErrorCode::ConstraintViolation => {},
                //             _ => panic!(e)
                //         }
                //     },
                //     _ => panic!(e)
                // }
                panic!(e)
            }
        }
    }
    fn contains_account(&self, username: &str) -> bool {
        self.load_account(username).is_some()
    }
    fn remove_account(&mut self, username: &str) {
        let con = self.open();
        con.execute(
            "DELETE FROM account_store WHERE username = $1",
            &[&username]).unwrap();
    }
}
