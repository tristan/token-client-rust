use super::rusqlite::{Connection, Error, ErrorCode};
use signal::protocol::{SignalProtocolAddress, IdentityKeyStore};
use ::keys::{ECPublicKey,IdentityKeyPair};

struct SQLiteStore {
    database: &'static str
}

// Identity stuff

impl SQLiteStore {
    pub fn new(database: &'static str) -> SQLiteStore {
        let store = SQLiteStore { database: database };
        store.create_id_tables();
        store
    }
    fn open(&self) -> Connection {
        Connection::open(self.database).unwrap()
    }

    fn create_id_tables(&self) {
        let con = self.open();
        con.execute(
            "CREATE TABLE IF NOT EXISTS identity_store (
                 address TEXT,
                 device_id INTEGER,
                 identity_key BLOB,
                 PRIMARY KEY (address, device_id)
                 );", &[]).unwrap();
    }
}

impl IdentityKeyStore for SQLiteStore {

    fn get_identity_key_pair(&self) -> IdentityKeyPair {
        // TODO:
        IdentityKeyPair::generate()
    }

    fn get_local_registration_id(&self) -> u32 {
        // TODO:
        0
    }

    fn store_identity(&mut self, address: &SignalProtocolAddress, identity_key: &ECPublicKey) {
        let con = self.open();
        let res = con.execute(
            "INSERT INTO identity_store (address, device_id, identity_key)
             VALUES ($1, $2, $3)",
            &[address.get_address(), address.get_device_id(),
              &identity_key.serialize()]);
        match res {
            Ok(_) => {},
            Err(e) => {
                // make sure the only error we count as ok is
                // a constraint violation
                match e {
                    Error::SqliteFailure(f, _) => {
                        match f.code {
                            ErrorCode::ConstraintViolation => {},
                            _ => panic!(e)
                        }
                    },
                    _ => panic!(e)
                }
            }
        }
    }
    fn load_identity(&self, address: &SignalProtocolAddress) -> Option<ECPublicKey> {
        let con = self.open();
        let rval = con.query_row(
            "SELECT identity_key FROM identity_store WHERE address = $1 AND device_id = $2",
            &[address.get_address(), address.get_device_id()],
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


#[cfg(test)]
mod tests {

    use std::fs::remove_file;

    use super::*;
    use ::keys::IdentityKeyPair;

    #[test]
    fn test_identity_store_trait() {

        // try remove the test file if it already exists
        remove_file("/tmp/token-rust-test-store.db").unwrap_or(());
        let mut idstore = SQLiteStore::new("/tmp/token-rust-test-store.db");

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
