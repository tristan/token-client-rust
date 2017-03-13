use super::rusqlite::{Connection, Error, ErrorCode};
use signal::protocol::{SignalProtocolAddress,SignalProtocolStore,IdentityKeyStore,
                       SessionStore,PreKeyStore,SignedPreKeyStore};
use signal::state::{PreKeyRecord,SignedPreKeyRecord};
use signal::ratchet::{SessionRecord};
use signal::keys::{ECPublicKey,IdentityKeyPair};

pub struct SQLiteStore {
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

impl PreKeyStore for SQLiteStore {
    fn load_pre_key(&self, pre_key_id: u32) -> Option<PreKeyRecord> {
        // match self.prekey_hash_map.get(&pre_key_id) {
        //     Some(record) => Some(record.clone()),
        //     None => None
        // }
        None
    }
    fn store_pre_key(&mut self, pre_key_id: u32, record: &PreKeyRecord) {
        //self.prekey_hash_map.insert(pre_key_id, record.clone());
    }

    fn contains_pre_key(&self, pre_key_id: u32) -> bool {
        //self.prekey_hash_map.contains_key(&pre_key_id)
        false
    }
    fn remove_pre_key(&mut self, pre_key_id: u32) {
        //self.prekey_hash_map.remove(&pre_key_id);
    }
}

impl SessionStore for SQLiteStore {
    fn load_session(&self, address: &SignalProtocolAddress) -> SessionRecord {
        // match self.session_hash_map.get(&address) {
        //     Some(record) => record.clone(),
        //     None => SessionRecord::new()
        // }
        SessionRecord::new()
    }
    fn store_session(&mut self, address: &SignalProtocolAddress, record: &SessionRecord) {
        //self.session_hash_map.insert(address.clone(), record.clone());
    }
    fn contains_session(&self, address: &SignalProtocolAddress) -> bool {
        //self.session_hash_map.contains_key(&address)
        false
    }
    fn delete_session(&mut self, address: &SignalProtocolAddress) {
        //self.session_hash_map.remove(&address);
    }
}

impl SignedPreKeyStore for SQLiteStore {
    fn load_signed_pre_key(&self, id: u32) -> Option<SignedPreKeyRecord> {
        // match self.signed_pre_key_store.get(&id) {
        //     Some(record) => Some(record.clone()),
        //     None => None
        // }
        None
    }
    fn store_signed_pre_key(&mut self, id: u32, record: &SignedPreKeyRecord) {
        //self.signed_pre_key_store.insert(id, record.clone());
    }
}

impl SignalProtocolStore for SQLiteStore {
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
