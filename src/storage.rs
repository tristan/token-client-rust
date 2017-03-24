use super::rusqlite::{Connection}; //, Error, ErrorCode};
use signal::protocol::{SignalProtocolAddress,
                       SignalProtocolStoreImpl,IdentityKeyStore,
                       SessionStore,PreKeyStore,SignedPreKeyStore};
use signal::state::{PreKeyRecord,SignedPreKeyRecord};
use signal::ratchet::{SessionRecord};
use signal::keys::{ECPublicKey,IdentityKeyPair};
use account::{Account,AccountStore};

pub struct SQLiteProtocolStore {
    database: String,
    identity_keypair: IdentityKeyPair,
    local_registration_id: u32
}

// Identity stuff

impl SQLiteProtocolStore {
    pub fn new(database: &str, identity_keypair: &IdentityKeyPair, local_registration_id: u32) -> SQLiteProtocolStore {
        let store = SQLiteProtocolStore {
            database: database.to_string(),
            identity_keypair: identity_keypair.clone(),
            local_registration_id: local_registration_id
        };
        store.create_tables();
        store
    }
    fn open(&self) -> Connection {
        Connection::open(self.database.clone()).unwrap()
    }

    fn create_tables(&self) {
        let con = self.open();
        // identity store
        con.execute(
            "CREATE TABLE IF NOT EXISTS identity_store (
                 address TEXT,
                 device_id INTEGER,
                 identity_key BLOB,
                 PRIMARY KEY (address, device_id)
                 );", &[]).unwrap();
        // prekey store
        con.execute(
            "CREATE TABLE IF NOT EXISTS prekey_store (
                 prekey_id INTEGER,
                 record BLOB,
                 PRIMARY KEY (prekey_id)
                 );", &[]).unwrap();
        // session store
        con.execute(
            "CREATE TABLE IF NOT EXISTS session_store (
                 address TEXT,
                 device_id INTEGER,
                 record BLOB,
                 PRIMARY KEY (address, device_id)
                 );", &[]).unwrap();
        // signed prekey store
        con.execute(
            "CREATE TABLE IF NOT EXISTS signed_prekey_store (
                 signed_prekey_id INTEGER,
                 record BLOB,
                 PRIMARY KEY (signed_prekey_id)
                 );", &[]).unwrap();
    }
}

impl IdentityKeyStore for SQLiteProtocolStore {

    fn get_identity_key_pair(&self) -> &IdentityKeyPair {
        &self.identity_keypair
    }

    fn get_local_registration_id(&self) -> u32 {
        self.local_registration_id
    }

    fn store_identity(&mut self, address: &SignalProtocolAddress, identity_key: &ECPublicKey) {
        let con = self.open();
        let res = con.execute(
            "INSERT OR REPLACE INTO identity_store (address, device_id, identity_key)
             VALUES ($1, $2, $3)",
            &[address.get_address(), &address.get_device_id(),
              &identity_key.serialize()]);
        match res {
            Ok(_) => {},
            Err(e) => {
                // // make sure the only error we count as ok is
                // // a constraint violation
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
    fn load_identity(&self, address: &SignalProtocolAddress) -> Option<ECPublicKey> {
        let con = self.open();
        let rval = con.query_row(
            "SELECT identity_key FROM identity_store WHERE address = $1 AND device_id = $2",
            &[address.get_address(), &address.get_device_id()],
            |row| {
                let id: Vec<u8> = row.get(0);
                ECPublicKey::deserialize(&id)
            });
        match rval {
            Ok(id) => {
                Some(id)
            },
            Err(_) => None
        }
    }

}

impl PreKeyStore for SQLiteProtocolStore {
    fn load_prekey(&self, prekey_id: u32) -> Option<PreKeyRecord> {
        let con = self.open();
        let rval = con.query_row(
            "SELECT record FROM prekey_store WHERE prekey_id = $1",
            &[&prekey_id],
            |row| {
                let record: Vec<u8> = row.get(0);
                PreKeyRecord::deserialize(&record)
            });
        match rval {
            Ok(record) => {
                Some(record)
            },
            Err(_) => None
        }
    }
    fn store_prekey(&mut self, prekey_id: u32, record: &PreKeyRecord) {
        let con = self.open();
        let res = con.execute(
            "INSERT INTO prekey_store (prekey_id, record)
             VALUES ($1, $2)",
            &[&prekey_id, &record.serialize()]);
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

    fn contains_prekey(&self, prekey_id: u32) -> bool {
        self.load_prekey(prekey_id).is_some()
    }
    fn remove_prekey(&mut self, prekey_id: u32) {
        let con = self.open();
        con.execute(
            "DELETE FROM prekey_store WHERE prekey_id = $1",
            &[&prekey_id]).unwrap();
    }
}

impl SessionStore for SQLiteProtocolStore {
    fn load_session(&self, address: &SignalProtocolAddress) -> SessionRecord {
        let con = self.open();
        let rval = con.query_row(
            "SELECT record FROM session_store WHERE address = $1 AND device_id = $2",
            &[address.get_address(), &address.get_device_id()],
            |row| {
                let record: Vec<u8> = row.get(0);
                SessionRecord::deserialize(&record)
            });
        match rval {
            Ok(record) => {
                record
            },
            Err(_) => SessionRecord::new()
        }

    }
    fn store_session(&mut self, address: &SignalProtocolAddress, record: &SessionRecord) {
        let con = self.open();
        let res = con.execute(
            "INSERT OR REPLACE INTO session_store (address, device_id, record)
             VALUES ($1, $2, $3)",
            &[address.get_address(), &address.get_device_id(),
              &record.serialize()]);
        match res {
            Ok(_) => {},
            Err(e) => panic!(e)
        }
    }
    fn contains_session(&self, address: &SignalProtocolAddress) -> bool {
        let con = self.open();
        let rval = con.query_row(
            "SELECT 1 FROM session_store WHERE address = $1 AND device_id = $2",
            &[address.get_address(), &address.get_device_id()],
            |_| {
                true
            });
        match rval {
            Ok(_) => true,
            Err(_) => false
        }
    }
    fn delete_session(&mut self, address: &SignalProtocolAddress) {
        let con = self.open();
        con.execute(
            "DELETE FROM session_store WHERE address = $1 AND device_id = $2",
            &[address.get_address(), &address.get_device_id()]).unwrap();
    }

    fn get_sub_device_sessions(&self, address: &String) -> Vec<u32> {
        let con = self.open();
        let mut stmt = con.prepare("SELECT device_id FROM session_store WHERE address = $1").unwrap();
        let rows = stmt.query_map(&[address], |row| {
            let val: u32 = row.get(0);
            val
        }).unwrap();
        let mut v = Vec::new();
        for val in rows {
            let val = val.unwrap();
            if val != 1 {
                v.push(val);
            }
        }
        v
    }
}

impl SignedPreKeyStore for SQLiteProtocolStore {
    fn load_signed_prekey(&self, id: u32) -> Option<SignedPreKeyRecord> {
        let con = self.open();
        let rval = con.query_row(
            "SELECT record FROM signed_prekey_store WHERE signed_prekey_id = $1",
            &[&id],
            |row| {
                let record: Vec<u8> = row.get(0);
                SignedPreKeyRecord::deserialize(&record)
            });
        match rval {
            Ok(record) => {
                Some(record)
            },
            Err(_) => None
        }
    }
    fn store_signed_prekey(&mut self, id: u32, record: &SignedPreKeyRecord) {
        let con = self.open();
        let res = con.execute(
            "INSERT INTO signed_prekey_store (signed_prekey_id, record)
             VALUES ($1, $2)",
            &[&id, &record.serialize()]);
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
    fn contains_signed_prekey(&self, id: u32) -> bool {
        self.load_signed_prekey(id).is_some()
    }
    fn remove_signed_prekey(&mut self, id: u32) {
        let con = self.open();
        con.execute(
            "DELETE FROM signed_prekey_store WHERE signed_prekey_id = $1",
            &[&id]).unwrap();
    }
}

impl SignalProtocolStoreImpl for SQLiteProtocolStore {
}

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

#[cfg(test)]
mod tests {

    use std::fs::remove_file;

    use super::*;
    use signal::keys::IdentityKeyPair;

    #[test]
    fn test_identity_store_trait() {

        // try remove the test file if it already exists
        remove_file("/tmp/token-rust-test-store.db").unwrap_or(());
        let mut idstore = SQLiteProtocolStore::new("/tmp/token-rust-test-store.db", &IdentityKeyPair::generate(), 0);

        let kp = IdentityKeyPair::generate();
        let kp2 = IdentityKeyPair::generate();
        // clone kp to test we're not just comparing references
        let kp3 = IdentityKeyPair::deserialize(&kp.serialize());
        let addr = SignalProtocolAddress::new("0x1234567890123456789012345678901234567890".to_string(), 1);
        let addr3 = SignalProtocolAddress::new("0x1234567890123456789012345678901234567890".to_string(), 1);
        assert_eq!(idstore.is_trusted_identity(&addr, &kp.get_public_key()), true);
        idstore.store_identity(&addr, &kp.get_public_key());
        assert_eq!(idstore.is_trusted_identity(&addr, &kp.get_public_key()), true);
        assert_eq!(idstore.is_trusted_identity(&addr3, &kp3.get_public_key()), true);
        assert_eq!(idstore.is_trusted_identity(&addr, &kp2.get_public_key()), false);
        // make sure this doesn't fail
        idstore.store_identity(&addr, &kp.get_public_key());
    }

}
