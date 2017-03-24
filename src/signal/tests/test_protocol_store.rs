
use ::signal::keys::{IdentityKeyPair,ECPublicKey};
use ::signal::state::{PreKeyRecord, SignedPreKeyRecord};
use ::signal::protocol;
use ::signal::session;
use ::signal::ratchet::SessionRecord;

use std::collections::HashMap;

pub struct TestProtocolStore {
    // identity store
    identity_key_pair: IdentityKeyPair,
    local_registration_id: u32,
    identity_hash_map: HashMap<String, ECPublicKey>,

    // pre key store
    prekey_hash_map: HashMap<u32, PreKeyRecord>,

    // session store
    session_hash_map: HashMap<protocol::SignalProtocolAddress, SessionRecord>,

    // signed pre key store
    signed_prekey_store: HashMap<u32, SignedPreKeyRecord>,
}

impl TestProtocolStore {
    pub fn new(identity_key_pair: &IdentityKeyPair, local_registration_id: u32) -> TestProtocolStore {
        TestProtocolStore {
            identity_key_pair: identity_key_pair.clone(),
            local_registration_id: local_registration_id,
            identity_hash_map: HashMap::new(),
            prekey_hash_map: HashMap::new(),
            session_hash_map: HashMap::new(),
            signed_prekey_store: HashMap::new()
        }
    }
}

impl protocol::IdentityKeyStore for TestProtocolStore {
    fn get_identity_key_pair(&self) -> &IdentityKeyPair {
        &self.identity_key_pair
    }

    fn get_local_registration_id(&self) -> u32 {
        self.local_registration_id
    }

    fn store_identity(&mut self, address: &protocol::SignalProtocolAddress, identity_key: &ECPublicKey) {
        let hashkey = format!("{}:{}", address.get_address(), address.get_device_id());
        self.identity_hash_map.insert(hashkey, identity_key.clone());
    }

    fn load_identity(&self, address: &protocol::SignalProtocolAddress) -> Option<ECPublicKey> {
        let hashkey = format!("{}:{}", address.get_address(), address.get_device_id());
        match self.identity_hash_map.get(&hashkey) {
            Some(key) => Some(key.clone()),
            None => None
        }
    }
}

impl protocol::PreKeyStore for TestProtocolStore {
    fn load_prekey(&self, prekey_id: u32) -> Option<PreKeyRecord> {
        match self.prekey_hash_map.get(&prekey_id) {
            Some(record) => Some(record.clone()),
            None => None
        }
    }
    fn store_prekey(&mut self, prekey_id: u32, record: &PreKeyRecord) {
        self.prekey_hash_map.insert(prekey_id, record.clone());
    }

    fn contains_prekey(&self, prekey_id: u32) -> bool {
        self.prekey_hash_map.contains_key(&prekey_id)
    }
    fn remove_prekey(&mut self, prekey_id: u32) {
        self.prekey_hash_map.remove(&prekey_id);
    }
}

impl protocol::SessionStore for TestProtocolStore {
    fn load_session(&self, address: &protocol::SignalProtocolAddress) -> SessionRecord {
        match self.session_hash_map.get(&address) {
            Some(record) => record.clone(),
            None => SessionRecord::new()
        }
    }
    fn store_session(&mut self, address: &protocol::SignalProtocolAddress, record: &SessionRecord) {
        self.session_hash_map.insert(address.clone(), record.clone());
    }
    fn contains_session(&self, address: &protocol::SignalProtocolAddress) -> bool {
        self.session_hash_map.contains_key(&address)
    }
    fn delete_session(&mut self, address: &protocol::SignalProtocolAddress) {
        self.session_hash_map.remove(&address);
    }

    fn get_sub_device_sessions(&self, address: &String) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        for key in self.session_hash_map.keys() {
            if key.get_address() == address && key.get_device_id() != 1 {
                res.push(key.get_device_id());
            }
        }
        return res;
    }
}

impl protocol::SignedPreKeyStore for TestProtocolStore {
    fn load_signed_prekey(&self, id: u32) -> Option<SignedPreKeyRecord> {
        match self.signed_prekey_store.get(&id) {
            Some(record) => Some(record.clone()),
            None => None
        }
    }
    fn store_signed_prekey(&mut self, id: u32, record: &SignedPreKeyRecord) {
        self.signed_prekey_store.insert(id, record.clone());
    }

    fn contains_signed_prekey(&self, id: u32) -> bool {
        self.signed_prekey_store.contains_key(&id)
    }
    fn remove_signed_prekey(&mut self, id: u32) {
        self.signed_prekey_store.remove(&id);
    }
}

impl protocol::SignalProtocolStoreImpl for TestProtocolStore {
}
