use super::keys::{IdentityKeyPair,ECPublicKey,ECKeyPair};
#[cfg(test)]
use super::keys::{ECPrivateKey};
use super::state::{PreKeyBundle,PreKeyRecord,SignedPreKeyRecord};
use super::ratchet::{SessionState,SessionRecord};
use super::message::{PreKeySignalMessage};
use super::curve::{curve25519_verify};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Eq;
use std::fmt;

pub enum SignalError {
    UntrustedIdentity,
    //InvalidKeyId,
    NoValidSessions,
    UninitializedSession,
    MessageVersionMismatch,
    DuplicateMessage,
    InvalidMessage,
    BadMac,
    InvalidKey,
    NoSession
}

impl fmt::Debug for SignalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            SignalError::UntrustedIdentity => "Untrusted Identity",
            //SignalError::InvalidKeyId => "Invalid Key ID",
            SignalError::NoValidSessions => "No Valid Sessions",
            SignalError::UninitializedSession => "Uninitialized Session",
            SignalError::MessageVersionMismatch => "Message Version Mismatch",
            SignalError::DuplicateMessage => "Duplicate Message",
            SignalError::InvalidMessage => "Invalid Message",
            SignalError::BadMac => "Bad Mac!",
            SignalError::InvalidKey => "Invalid Key",
            SignalError::NoSession => "No Session for given address"
        };
        write!(f, "Signal Error: {}", msg)
    }
}

#[derive(Hash,Clone,PartialEq)]
pub struct SignalProtocolAddress {
    address: String,
    device_id: u32
}

impl Eq for SignalProtocolAddress {}

impl SignalProtocolAddress {
    pub fn new(address: &str, device_id: u32) -> SignalProtocolAddress {
        SignalProtocolAddress {
            address: address.to_string(),
            device_id: device_id
        }
    }
    pub fn get_address(&self) -> &String {
        &self.address
    }
    pub fn get_device_id(&self) -> u32 {
        self.device_id
    }
}

pub trait IdentityKeyStore {
    fn get_identity_key_pair(&self) -> &IdentityKeyPair;
    fn get_local_registration_id(&self) -> u32;

    fn store_identity(&mut self, address: &SignalProtocolAddress, identity_key: &ECPublicKey);
    fn load_identity(&self, address: &SignalProtocolAddress) -> Option<ECPublicKey>;

    fn is_trusted_identity(&self, address: &SignalProtocolAddress, identity_key: &ECPublicKey) -> bool {
        match self.load_identity(address) {
            Some(id) => {
                &id == identity_key
            },
            None => true
        }
    }
}

pub trait PreKeyStore {
    fn load_prekey(&self, prekey_id: u32) -> Option<PreKeyRecord>;
    fn store_prekey(&mut self, prekey_id: u32, record: &PreKeyRecord);
    fn contains_prekey(&self, prekey_id: u32) -> bool;
    fn remove_prekey(&mut self, prekey_id: u32);
}

pub trait SessionStore {
    // no option as this should create a new record if one doesn't exist
    fn load_session(&self, address: &SignalProtocolAddress) -> SessionRecord;
    fn store_session(&mut self, address: &SignalProtocolAddress, record: &SessionRecord);
    fn contains_session(&self, address: &SignalProtocolAddress) -> bool;
    fn delete_session(&mut self, address: &SignalProtocolAddress);

    fn get_sub_device_sessions(&self, address: &String) -> Vec<u32>;
}

pub trait SignedPreKeyStore {
    fn load_signed_prekey(&self, id: u32) -> Option<SignedPreKeyRecord>;
    fn store_signed_prekey(&mut self, id: u32, record: &SignedPreKeyRecord);
    fn contains_signed_prekey(&self, id: u32) -> bool;
    fn remove_signed_prekey(&mut self, id: u32);
}

pub trait SignalProtocolStoreImpl : IdentityKeyStore + PreKeyStore + SessionStore + SignedPreKeyStore {

    fn process_prekey_bundle(&mut self,
                             remote_address: &SignalProtocolAddress,
                             prekey: &PreKeyBundle)
                             -> Result<(), SignalError> {
        if ! self.is_trusted_identity(remote_address, prekey.get_identity_key()) {
            return Err(SignalError::UntrustedIdentity);
        }

        if ! curve25519_verify(prekey.get_identity_key().as_slice(),
                               &prekey.get_signed_prekey().serialize(),
                               &prekey.get_signed_prekey_signature_as_slice()) {
            return Err(SignalError::InvalidKey);
        }

        let mut session_record = self.load_session(remote_address);
        // to ensure tests always return the same results
        #[cfg(test)]
        let our_base_key = ECKeyPair::create(
            ECPrivateKey::deserialize(&vec![144, 193, 85, 136, 117, 98, 48, 253, 174, 72, 43, 253, 209, 73, 136, 162, 79, 26, 183, 215, 197, 55, 220, 101, 171, 253, 238, 238, 220, 53, 200, 112]),
            ECPublicKey::deserialize(&vec![5, 112, 106, 1, 234, 114, 124, 179, 151, 247, 226, 24, 184, 163, 15, 62, 55, 86, 63, 56, 16, 163, 51, 184, 208, 235, 190, 242, 133, 154, 152, 65, 36]));
        #[cfg(not(test))]
        let our_base_key = ECKeyPair::generate();
        let their_signed_prekey = prekey.get_signed_prekey();

        let mut session = SessionState::new();
        session.initialize_as_alice(
            &self.get_identity_key_pair(),
            &our_base_key,
            prekey.get_identity_key(),
            &their_signed_prekey,
            &their_signed_prekey,
            prekey.get_prekey()
        );

        session.set_unacknowledged_prekey_message(prekey.get_prekey_id(), prekey.get_signed_prekey_id(), our_base_key.get_public_key());
        session.set_local_registration_id(self.get_local_registration_id());
        session.set_remote_registration_id(prekey.get_registration_id());
        session.set_alice_base_key(our_base_key.get_public_key().serialize().to_vec());

        session_record.push_state(session);

        self.store_session(remote_address, &session_record);
        self.store_identity(remote_address, prekey.get_identity_key());

        Ok(())
    }

    fn process_prekey_message(&mut self, remote_address: &SignalProtocolAddress,
                              session_record: &mut SessionRecord, message: &PreKeySignalMessage)
                              -> Result<Option<u32>, SignalError> {

        let their_identity_key = message.get_identity_key();
        if ! self.is_trusted_identity(remote_address, &their_identity_key) {
            return Err(SignalError::UntrustedIdentity)
        }

        // process v3
        if session_record.has_session_state(message.get_version(), &message.get_base_key().serialize()) {
            // We've already setup a session for this V3 message, letting bundled message fall through...
            return Ok(None)
        }

        let spkr = self.load_signed_prekey(
            message.get_signed_prekey_id()).unwrap();
        let our_signed_prekey = spkr.get_key_pair();

        let one_time_prekey = match message.get_prekey_id() {
            Some(id) => {
                match self.load_prekey(id) {
                    Some(prekey) => {
                        Some(prekey.get_key_pair().clone())
                    },
                    None => None
                }
            },
            None => None
        };

        let mut session = SessionState::new();
        session.initialize_as_bob(
            &self.get_identity_key_pair(),
            &our_signed_prekey,
            &our_signed_prekey,
            one_time_prekey,
            message.get_identity_key(),
            message.get_base_key()
        );

        session.set_local_registration_id(self.get_local_registration_id());
        session.set_remote_registration_id(message.get_registration_id());
        session.set_alice_base_key(message.get_base_key().serialize().to_vec());

        session_record.push_state(session);

        let unsigned_prekey_id = match message.get_prekey_id() {
            Some(id) => {
                if id != 0xFFFFFF { // Medium.MAX_VALUE
                    Ok(Some(id))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None)
        };

        self.store_identity(remote_address, &their_identity_key);

        unsigned_prekey_id
    }

}

pub type SignalProtocolStoreRef = Rc<RefCell<Box<SignalProtocolStoreImpl>>>;

#[derive(Clone)]
pub struct SignalProtocolStore(SignalProtocolStoreRef);

impl SignalProtocolStore {
    pub fn new(store: Box<SignalProtocolStoreImpl>) -> SignalProtocolStore {
        SignalProtocolStore(Rc::new(RefCell::new(store)))
    }

    // identity
    pub fn get_identity_key_pair(&self) -> IdentityKeyPair {
        (self.0.borrow()).get_identity_key_pair().clone()
    }
    pub fn get_local_registration_id(&self) -> u32 {
        (self.0.borrow()).get_local_registration_id()
    }

    pub fn store_identity(&mut self, address: &SignalProtocolAddress, identity_key: &ECPublicKey) {
        (self.0.borrow_mut()).store_identity(address, identity_key)
    }
    pub fn load_identity(&self, address: &SignalProtocolAddress) -> Option<ECPublicKey> {
        (self.0.borrow_mut()).load_identity(address)
    }

    pub fn is_trusted_identity(&self, address: &SignalProtocolAddress, identity_key: &ECPublicKey) -> bool {
        (self.0.borrow_mut()).is_trusted_identity(address, identity_key)
    }

    // prekey
    pub fn load_prekey(&self, prekey_id: u32) -> Option<PreKeyRecord> {
        (self.0.borrow()).load_prekey(prekey_id)
    }
    pub fn store_prekey(&mut self, prekey_id: u32, record: &PreKeyRecord) {
        (self.0.borrow_mut()).store_prekey(prekey_id, record)
    }
    pub fn contains_prekey(&self, prekey_id: u32) -> bool {
        (self.0.borrow()).contains_prekey(prekey_id)
    }
    pub fn remove_prekey(&mut self, prekey_id: u32) {
        (self.0.borrow_mut()).remove_prekey(prekey_id)
    }

    // session
    pub fn load_session(&self, address: &SignalProtocolAddress) -> SessionRecord {
        (self.0.borrow()).load_session(address)
    }
    pub fn store_session(&mut self, address: &SignalProtocolAddress, record: &SessionRecord) {
        (self.0.borrow_mut()).store_session(address, record)
    }
    pub fn contains_session(&self, address: &SignalProtocolAddress) -> bool {
        (self.0.borrow()).contains_session(address)
    }
    pub fn delete_session(&mut self, address: &SignalProtocolAddress) {
        (self.0.borrow_mut()).delete_session(address)
    }

    pub fn get_sub_device_sessions(&self, address: &String) -> Vec<u32> {
        (self.0.borrow()).get_sub_device_sessions(address)
    }

    // signed prekey
    pub fn load_signed_prekey(&self, id: u32) -> Option<SignedPreKeyRecord> {
        (self.0.borrow()).load_signed_prekey(id)
    }
    pub fn store_signed_prekey(&mut self, id: u32, record: &SignedPreKeyRecord) {
        (self.0.borrow_mut()).store_signed_prekey(id, record)
    }
    pub fn contains_signed_prekey(&self, id: u32) -> bool {
        (self.0.borrow()).contains_signed_prekey(id)
    }
    pub fn remove_signed_prekey(&mut self, id: u32) {
        (self.0.borrow_mut()).remove_signed_prekey(id)
    }

    // misc

    pub fn process_prekey_bundle(&mut self,
                                 remote_address: &SignalProtocolAddress,
                                 prekey: &PreKeyBundle)
                                 -> Result<(), SignalError> {
        (self.0.borrow_mut()).process_prekey_bundle(remote_address, prekey)
    }
    pub fn process_prekey_message(&mut self, remote_address: &SignalProtocolAddress,
                                  session_record: &mut SessionRecord, message: &PreKeySignalMessage)
                                  -> Result<Option<u32>, SignalError> {
        (self.0.borrow_mut()).process_prekey_message(remote_address, session_record, message)
    }
}

macro_rules! new_protocol_store { ($store:expr) => (SignalProtocolStore::new(Box::new($store))) }

#[cfg(test)]
mod tests {

    use super::*;
    use ::signal::keys::IdentityKeyPair;

    #[test]
    fn test_identity_store_trait() {

        struct IdStore<'a> {
            addr: &'a SignalProtocolAddress,
            identity_key: &'a ECPublicKey,
            id_keypair: IdentityKeyPair,
            saved: bool
        }
        impl<'a> IdentityKeyStore for IdStore<'a> {
            fn get_identity_key_pair(&self) -> &IdentityKeyPair {
                &self.id_keypair
            }
            fn get_local_registration_id(&self) -> u32 {
                0
            }
            #[allow(unused_variables)]
            fn store_identity(&mut self, address: &SignalProtocolAddress, identity_key: &ECPublicKey) {
                self.saved = true;
            }
            fn load_identity(&self, address: &SignalProtocolAddress) -> Option<ECPublicKey> {
                if self.saved && address == self.addr {
                    Some(self.identity_key.clone())
                } else {
                    None
                }
            }
        }

        let kp = IdentityKeyPair::generate();
        let kp2 = IdentityKeyPair::generate();
        let kp3 = IdentityKeyPair::deserialize(&kp.serialize());
        let addr = SignalProtocolAddress::new("0x1234567890123456789012345678901234567890", 1);
        let addr2 = SignalProtocolAddress::new("0x1234567890123456789012345678901234567890", 1);
        let mut idstore = IdStore { addr: &addr, identity_key: kp.get_public_key(), id_keypair: IdentityKeyPair::generate(), saved: false };
        assert_eq!(idstore.is_trusted_identity(&addr, &kp.get_public_key()), true);
        assert_eq!(idstore.is_trusted_identity(&addr, &kp2.get_public_key()), true);
        idstore.store_identity(&addr, &kp.get_public_key());
        assert_eq!(idstore.is_trusted_identity(&addr, &kp.get_public_key()), true);
        assert_eq!(idstore.is_trusted_identity(&addr, &kp2.get_public_key()), false);
        assert_eq!(idstore.is_trusted_identity(&addr2, &kp3.get_public_key()), true);

    }

}
