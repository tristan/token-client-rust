use super::keys::{IdentityKeyPair,ECPublicKey,ECKeyPair};
use super::state::{PreKeyBundle,PreKeyRecord,SignedPreKeyRecord};
use super::ratchet::{SessionState,SessionRecord};
use super::curve::{curve25519_sign, curve25519_verify};
use std::cmp::Eq;
use std::fmt;

pub enum SignalError {
    UntrustedIdentity,
    InvalidKeyId,
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
            SignalError::InvalidKeyId => "Invalid Key ID",
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
    pub fn new(address: String, device_id: u32) -> SignalProtocolAddress {
        SignalProtocolAddress {
            address: address,
            device_id: device_id
        }
    }
    pub fn get_address(&self) -> &String {
        &self.address
    }
    pub fn get_device_id(&self) -> &u32 {
        &self.device_id
    }
}

pub trait IdentityKeyStore {
    fn get_identity_key_pair(&self) -> IdentityKeyPair;
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
    fn load_pre_key(&self, pre_key_id: u32) -> Option<PreKeyRecord>;
    fn store_pre_key(&mut self, pre_key_id: u32, record: &PreKeyRecord);
    fn contains_pre_key(&self, pre_key_id: u32) -> bool;
    fn remove_pre_key(&mut self, pre_key_id: u32);
}

pub trait SessionStore {
    // no option as this should create a new record if one doesn't exist
    fn load_session(&self, address: &SignalProtocolAddress) -> SessionRecord;
    fn store_session(&mut self, address: &SignalProtocolAddress, record: &SessionRecord);
    fn contains_session(&self, address: &SignalProtocolAddress) -> bool;
    fn delete_session(&mut self, address: &SignalProtocolAddress);
}

pub trait SignedPreKeyStore {
    fn load_signed_pre_key(&self, id: u32) -> Option<SignedPreKeyRecord>;
    fn store_signed_pre_key(&mut self, id: u32, record: &SignedPreKeyRecord);
}

pub trait SignalProtocolStore : IdentityKeyStore + PreKeyStore + SessionStore + SignedPreKeyStore {

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

}

#[cfg(test)]
mod tests {

    use super::*;
    use ::signal::keys::IdentityKeyPair;

    #[test]
    fn test_identity_store_trait() {

        struct IdStore<'a> {
            addr: &'a SignalProtocolAddress,
            identity_key: &'a ECPublicKey,
            saved: bool
        }
        impl<'a> IdentityKeyStore for IdStore<'a> {
            fn get_identity_key_pair(&self) -> IdentityKeyPair {
                IdentityKeyPair::generate() // lol
            }
            fn get_local_registration_id(&self) -> u32 {
                0
            }
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
        let addr = SignalProtocolAddress::new("0x1234567890123456789012345678901234567890".to_string(), 1);
        let addr2 = SignalProtocolAddress::new("0x1234567890123456789012345678901234567890".to_string(), 1);
        let mut idstore = IdStore { addr: &addr, identity_key: kp.get_public_key(), saved: false };
        assert_eq!(idstore.is_trusted_identity(&addr, &kp.get_public_key()), true);
        assert_eq!(idstore.is_trusted_identity(&addr, &kp2.get_public_key()), true);
        idstore.store_identity(&addr, &kp.get_public_key());
        assert_eq!(idstore.is_trusted_identity(&addr, &kp.get_public_key()), true);
        assert_eq!(idstore.is_trusted_identity(&addr, &kp2.get_public_key()), false);
        assert_eq!(idstore.is_trusted_identity(&addr2, &kp3.get_public_key()), true);

    }

}
