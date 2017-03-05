extern crate crypto;

use super::message::{CURRENT_VERSION, SignalMessage};
use super::protocol::SignalError;
use super::cipher;
use ::keys::{IdentityKeyPair, ECKeyPair, ECPrivateKey, ECPublicKey};
use super::LocalStorageProtocol;
use ::protobuf::{Message, RepeatedField};
use std::collections::LinkedList;
use self::crypto::hkdf::{hkdf_extract,hkdf_expand};
use self::crypto::mac::Mac;
use self::crypto::hmac::Hmac;
use self::crypto::sha2::Sha256;
use std::mem;

// derive_secrets
macro_rules! derive_secrets {
    ($ikm:expr, $info:expr, $outlen:expr) => {
        derive_secrets!($ikm, [0u8;32], $info, $outlen)
    };
    ($ikm:expr, $salt:expr, $info:expr, $outlen:expr) => {
        {
            let digest = Sha256::new();
            let mut prk: Vec<u8> = (0..).take(32).collect();
            let mut okm: Vec<u8> = (0..).take($outlen).collect();
            hkdf_extract(digest, &$salt, &$ikm, &mut prk);
            hkdf_expand(digest, &prk, &$info, &mut okm);
            okm
        }
    };
}

enum KeyTypes {
    AES,
    HmacSHA256,
}

struct MessageKeys {
    cipher_key: Vec<u8>,
    cipher_key_type: KeyTypes,
    mac_key: Vec<u8>,
    mac_key_type: KeyTypes,
    iv: Vec<u8>,
    counter: u32
}

struct ChainKey {
    kdf_version: u32,
    key: Vec<u8>,
    index: u32
}

static MESSAGE_KEY_SEED: [u8;1] = [0x01];
static CHAIN_KEY_SEED: [u8;1] = [0x02];

impl ChainKey {
    fn create(version: u32, key: &Vec<u8>, index: u32) -> ChainKey {
        println!("CHAINKEY: {:?}, {:?}", key, index);
        ChainKey {
            kdf_version: version,
            key: key.clone(),
            index: index
        }
    }

    fn get_base_material(&self, seed: &Vec<u8>) -> Vec<u8> {
        let mut mac = Hmac::new(Sha256::new(), &self.key);
        mac.input(&seed);
        mac.result().code().to_vec()
    }

    fn get_message_keys(&self) -> MessageKeys {
        let input_key_material = self.get_base_material(&MESSAGE_KEY_SEED.to_vec());
        let key_material_bytes = derive_secrets!(
            input_key_material,
            // "WhisperMessageKeys".as_bytes()
            [87, 104, 105, 115, 112, 101, 114, 77, 101, 115, 115, 97, 103, 101, 75, 101, 121, 115],
            80);
        let cipher_key = key_material_bytes[..32].to_vec();
        let mac_key = key_material_bytes[32..64].to_vec();
        let iv = key_material_bytes[64..80].to_vec();
        MessageKeys {
            cipher_key: cipher_key,
            cipher_key_type: KeyTypes::AES,
            mac_key: mac_key,
            mac_key_type: KeyTypes::HmacSHA256,
            iv: iv,
            counter: self.index
        }
    }

    fn get_next_chain_key(&self) -> ChainKey {
        ChainKey {
            kdf_version: self.kdf_version,
            key: self.get_base_material(&CHAIN_KEY_SEED.to_vec()),
            index: self.index + 1
        }
    }
}

fn calculate_derived_keys(master_secret: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let okm = derive_secrets!(
        master_secret,
        // "WhisperText".as_bytes()
        [87, 104, 105, 115, 112, 101, 114, 84, 101, 120, 116],
        64);
    let (r, c) = okm.split_at(32);
    (r.to_vec(), c.to_vec())
}

fn create_chain(root_key_bytes: &Vec<u8>, their_ratchet_key: &ECPublicKey, our_ratchet_key: &ECKeyPair) -> (Vec<u8>, Vec<u8>) {
    let shared_secret = our_ratchet_key.get_private_key().calculate_agreement(
        their_ratchet_key);
    let derived_secret_bytes = derive_secrets!(
        shared_secret, root_key_bytes,
        // "WhisperRatchet".as_bytes()
        [87, 104, 105, 115, 112, 101, 114, 82, 97, 116, 99, 104, 101, 116],
        64);
    let (new_root_key_bytes, new_chain_key_bytes) =
        derived_secret_bytes.split_at(32);
    (new_root_key_bytes.to_vec(), new_chain_key_bytes.to_vec())
}

#[derive(Default,Clone)]
pub struct SessionState(LocalStorageProtocol::SessionStructure);

impl SessionState {

    pub fn initialize_as_alice(
        our_identity_key: &IdentityKeyPair,
        our_base_key: &ECKeyPair,
        their_identity_key: &ECPublicKey,
        their_signed_pre_key: &ECPublicKey,
        their_ratchet_key: &ECPublicKey,
        their_one_time_pre_key: Option<&ECPublicKey>
    ) -> SessionState {

        let mut session = LocalStorageProtocol::SessionStructure::new();
        let mut state = SessionState(session);
        state.0.set_sessionVersion(CURRENT_VERSION);
        state.0.set_remoteIdentityPublic(their_identity_key.serialize().to_vec());
        state.0.set_localIdentityPublic(our_identity_key.get_public_key().serialize().to_vec());

        let sending_ratchet_key = ECKeyPair::generate();

        let mut secrets: Vec<u8> = vec![0xff; 32];
        secrets.extend(our_identity_key.get_private_key().calculate_agreement(their_signed_pre_key));
        secrets.extend(our_base_key.get_private_key().calculate_agreement(their_identity_key));
        secrets.extend(our_base_key.get_private_key().calculate_agreement(their_signed_pre_key));
        match their_one_time_pre_key {
            Some(key) => {
                secrets.extend(our_base_key.get_private_key().calculate_agreement(key));
            },
            None => {}
        };

        let (root_key_bytes, chain_key_bytes) = calculate_derived_keys(&secrets);
        let (new_root_key_bytes, new_chain_key_bytes) =
            create_chain(&root_key_bytes, &their_ratchet_key, &sending_ratchet_key);

        // add_receiver_chain
        state.add_receiver_chain(&their_ratchet_key, &chain_key_bytes, 0);
        // set_sender_chain
        state.set_sender_chain(&sending_ratchet_key, &chain_key_bytes, 0);

        // set_root_key
        state.0.set_rootKey(new_root_key_bytes.to_vec());

        state
    }

    pub fn initialize_as_bob(
        our_identity_key: &IdentityKeyPair,
        our_signed_pre_key: &ECKeyPair,
        our_ratchet_key: &ECKeyPair,
        our_one_time_pre_key: Option<ECKeyPair>,
        their_identity_key: &ECPublicKey,
        their_base_key: &ECPublicKey
    ) -> SessionState {
        let session = LocalStorageProtocol::SessionStructure::new();
        let mut state = SessionState(session);
        state.0.set_sessionVersion(CURRENT_VERSION);
        state.0.set_remoteIdentityPublic(their_identity_key.serialize().to_vec());
        state.0.set_localIdentityPublic(our_identity_key.get_public_key().serialize().to_vec());

        let mut secrets: Vec<u8> = vec![0xff; 32];
        secrets.extend(our_signed_pre_key.get_private_key().calculate_agreement(their_identity_key));
        secrets.extend(our_identity_key.get_private_key().calculate_agreement(their_base_key));
        secrets.extend(our_signed_pre_key.get_private_key().calculate_agreement(their_base_key));
        match our_one_time_pre_key {
            Some(key) => {
                secrets.extend(key.get_private_key().calculate_agreement(their_base_key));
            },
            None => {}
        };

        let (root_key_bytes, chain_key_bytes) = calculate_derived_keys(&secrets);

        // set_sender_chain
        state.set_sender_chain(&our_ratchet_key, &chain_key_bytes, 0);

        // set_root_key
        state.0.set_rootKey(root_key_bytes.to_vec());

        state
    }

    pub fn set_local_registration_id(&mut self, registration_id: u32) {
        self.0.set_localRegistrationId(registration_id);
    }

    pub fn set_remote_registration_id(&mut self, registration_id: u32) {
        self.0.set_remoteRegistrationId(registration_id);
    }

    pub fn set_alice_base_key(&mut self, base_key: Vec<u8>) {
        self.0.set_aliceBaseKey(base_key);
    }

    pub fn decrypt(&mut self, ciphertext: &SignalMessage) -> Result<Vec<u8>, SignalError> {
        if ! self.0.has_senderChain() {
            return Err(SignalError::UninitializedSession);
        }

        let message_version = ciphertext.get_version();
        if message_version != self.0.get_sessionVersion() {
            return Err(SignalError::MessageVersionMismatch);
        }

        let their_ephemeral = ciphertext.get_sender_ratchet_key();
        let counter = ciphertext.get_counter();

        // get or create chain key
        let chainkey = {
            // try get
            match self.get_receiver_chain_key(&their_ephemeral) {
                Some((key, index)) => ChainKey::create(3, &key, index),
                None => {
                    // create
                    let our_ephemeral = {
                        // TODO: figure out why i need to put the self.0.get calls
                        // inside a different scope
                        let sender_chain = self.0.get_senderChain();
                        ECKeyPair::create(
                            ECPrivateKey::deserialize(
                                &sender_chain.get_senderRatchetKeyPrivate().to_vec()),
                            ECPublicKey::deserialize(
                                &sender_chain.get_senderRatchetKey().to_vec())
                        )
                    };
                    let (receiver_root_key_bytes, receiver_chain_key_bytes) =
                        create_chain(&self.0.get_rootKey().to_vec(), &their_ephemeral, &our_ephemeral);
                    let our_new_ephemeral = ECKeyPair::generate();
                    let (sender_root_key_bytes, sender_chain_key_bytes) =
                        create_chain(&receiver_root_key_bytes, &their_ephemeral, &our_new_ephemeral);

                    self.0.set_rootKey(sender_root_key_bytes);
                    self.add_receiver_chain(&their_ephemeral, &receiver_chain_key_bytes, 0);
                    let new_previous_counter = {
                        let idx = self.0.get_senderChain().get_chainKey().get_index();
                        if idx == 0 {
                            0
                        } else {
                            idx - 1
                        }
                    };
                    self.0.set_previousCounter(new_previous_counter);
                    self.set_sender_chain(&our_new_ephemeral, &sender_chain_key_bytes, 0);

                    ChainKey::create(3, &receiver_chain_key_bytes, 0)
                }
            }
        };

        // get or create message keys
        let message_keys = {
            if chainkey.index > counter {
                match self.remove_message_keys(&their_ephemeral, counter) {
                    Some(message_keys) => message_keys,
                    None => {
                        return Err(SignalError::DuplicateMessage);
                    }
                }
            } else if counter - chainkey.index > 2000 {
                // TODO: msg: over 2000 messages into the future!
                return Err(SignalError::InvalidMessage);
            } else {
                let mut chainkey = chainkey;
                while chainkey.index < counter {
                    let message_keys = chainkey.get_message_keys();
                    self.set_message_keys(&their_ephemeral, &message_keys);
                    chainkey = chainkey.get_next_chain_key();
                }
                self.set_receiver_chain_key(&their_ephemeral, &chainkey.get_next_chain_key());
                chainkey.get_message_keys()
            }
        };

        if ! ciphertext.verify_mac(
            message_version,
            &ECPublicKey::deserialize(&self.0.get_remoteIdentityPublic().to_vec()),
            &ECPublicKey::deserialize(&self.0.get_localIdentityPublic().to_vec()),
            &message_keys.mac_key) {
            return Err(SignalError::BadMac);
        }

        // get plaintext
        let plaintext = {
            if message_version >= 3 {
                cipher::decrypt_cbc(
                    &ciphertext.get_body(),
                    &message_keys.cipher_key,
                    &message_keys.iv)
            } else {
                cipher::decrypt_ctr(
                    &ciphertext.get_body(),
                    &message_keys.cipher_key,
                    counter)
            }
        };
        let plaintext = match plaintext {
            Ok(plaintext) => plaintext,
            Err(_) => {
                // TODO: be more specific
                return Err(SignalError::InvalidMessage);
            }
        };

        self.0.clear_pendingPreKey();

        Ok(plaintext)
    }

    fn get_receiver_chain(&self, their_ephemeral: &ECPublicKey)
                          -> Option<(&LocalStorageProtocol::SessionStructure_Chain, u32)> {
        let mut index = 0;
        let receiver_chains = self.0.get_receiverChains();
        for chain in receiver_chains {
            let chain_sender_ratchet_key = ECPublicKey::deserialize(
                &chain.get_senderRatchetKey().to_vec());
            if &chain_sender_ratchet_key == their_ephemeral {
                return Some((chain, index));
            }
            index += 1;
        }
        return None;
    }

    fn mut_receiver_chain(&mut self, their_ephemeral: &ECPublicKey)
                          -> Option<(&mut LocalStorageProtocol::SessionStructure_Chain, u32)> {
        let mut index = 0;
        let mut receiver_chains = self.0.mut_receiverChains();
        for chain in receiver_chains.iter_mut() {
            let chain_sender_ratchet_key = ECPublicKey::deserialize(
                &chain.get_senderRatchetKey().to_vec());
            if &chain_sender_ratchet_key == their_ephemeral {
                return Some((chain, index));
            }
            index += 1;
        }
        return None;
    }

    fn get_receiver_chain_key(&self, their_ephemeral: &ECPublicKey) -> Option<(Vec<u8>, u32)> {
        match self.get_receiver_chain(their_ephemeral) {
            Some((chain, _)) => {
                let chainkey = chain.get_chainKey();
                Some((chainkey.get_key().to_vec(), chainkey.get_index()))
            },
            None => None
        }
    }

    fn set_receiver_chain_key(&mut self, sender_ephemeral: &ECPublicKey, chain_key: &ChainKey) {
        //let chain_key_bytes: &Vec<u8>, chain_key_index: u32
        match self.mut_receiver_chain(sender_ephemeral) {
            Some((chain, idx)) => {
                let mut chain = chain;
                let mut chainkey_structure = LocalStorageProtocol::SessionStructure_Chain_ChainKey::new();
                chainkey_structure.set_key(chain_key.key.clone());
                chainkey_structure.set_index(chain_key.index);

                chain.set_chainKey(chainkey_structure);
            }
            None => panic!("TODO: Not sure what to do here yet")
        };
    }

    fn add_receiver_chain(&mut self, sender_ratchet_key: &ECPublicKey, chain_key_bytes: &Vec<u8>, chain_key_index: u32) {
        let mut chainkey = LocalStorageProtocol::SessionStructure_Chain_ChainKey::new();
        chainkey.set_key(chain_key_bytes.to_vec());
        chainkey.set_index(chain_key_index);

        let mut chain = LocalStorageProtocol::SessionStructure_Chain::new();
        chain.set_chainKey(chainkey);
        chain.set_senderRatchetKey(sender_ratchet_key.serialize().to_vec());

        self.0.mut_receiverChains().push(chain);
        if self.0.get_receiverChains().len() > 5 {
            self.0.mut_receiverChains().remove(0);
        }
    }

    fn set_sender_chain(&mut self, sender_ratchet_key_pair: &ECKeyPair, chain_key_bytes: &Vec<u8>, chain_key_index: u32 ) {
        let mut chainkey = LocalStorageProtocol::SessionStructure_Chain_ChainKey::new();
        chainkey.set_key(chain_key_bytes.to_vec());
        chainkey.set_index(chain_key_index);

        let mut senderchain = LocalStorageProtocol::SessionStructure_Chain::new();
        senderchain.set_senderRatchetKey(sender_ratchet_key_pair.get_public_key().serialize().to_vec());
        senderchain.set_senderRatchetKeyPrivate(sender_ratchet_key_pair.get_private_key().serialize().to_vec());
        senderchain.set_chainKey(chainkey);

        self.0.set_senderChain(senderchain);
    }

    fn remove_message_keys(&mut self, their_ephemeral: &ECPublicKey, counter: u32) -> Option<MessageKeys> {
        let (receiver_chain, _) = match self.mut_receiver_chain(&their_ephemeral) {
            Some((chain, chain_index)) => (chain, chain_index),
            None => {
                return None;
            }
        };
        // has_message_keys + remove_message_keys
        let mut message_key_list = receiver_chain.mut_messageKeys();
        let pos = message_key_list.iter().position(|message_key| {
            message_key.get_index() == counter
        });

        match pos {
            Some(index) => {
                let message_key = message_key_list.remove(index);
                return Some(MessageKeys {
                    cipher_key: message_key.get_cipherKey().to_vec(),
                    cipher_key_type: KeyTypes::AES,
                    mac_key: message_key.get_macKey().to_vec(),
                    mac_key_type: KeyTypes::HmacSHA256,
                    iv: message_key.get_iv().to_vec(),
                    counter: message_key.get_index()
                });
            },
            None => {
                return None;
            }
        }
    }

    fn set_message_keys(&mut self, sender_ephemeral: &ECPublicKey, message_keys: &MessageKeys) {
        match self.mut_receiver_chain(sender_ephemeral) {
            Some((chain, idx)) => {
                let mut messagekey_structure = LocalStorageProtocol::SessionStructure_Chain_MessageKey::new();
                messagekey_structure.set_cipherKey(message_keys.cipher_key.clone());
                messagekey_structure.set_macKey(message_keys.mac_key.clone());
                messagekey_structure.set_iv(message_keys.iv.clone());
                messagekey_structure.set_index(message_keys.counter);

                chain.mut_messageKeys().push(messagekey_structure);
                if chain.get_messageKeys().len() > 5 {
                    chain.mut_messageKeys().remove(0);
                }
            }
            None => panic!("TODO: Not sure what to do here yet")
        };
    }

}

#[derive(Clone)]
pub struct SessionRecord {
    state: SessionState,
    previous: LinkedList<SessionState>,
    fresh: bool
}

const ARCHIVED_STATES_MAX_LENGTH: usize = 40;

impl SessionRecord {
    pub fn new() -> SessionRecord {
        SessionRecord {
            state: SessionState::default(),
            previous: LinkedList::new(),
            fresh: true
        }
    }

    pub fn deserialize(serialized: &Vec<u8>) -> SessionRecord {
        let mut record = LocalStorageProtocol::RecordStructure::new();
        record.merge_from_bytes(serialized).unwrap();
        let mut previous: LinkedList<SessionState> = LinkedList::new();
        for state in record.take_previousSessions().iter() {
            previous.push_back(SessionState(state.clone()));
        }
        SessionRecord {
            state: SessionState(record.take_currentSession()),
            previous: previous,
            fresh: false
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut record = LocalStorageProtocol::RecordStructure::new();
        let mut previous = RepeatedField::new();

        for state in self.previous.iter() {
            previous.push(state.0.clone());
        }

        record.set_currentSession(self.state.0.clone());
        record.set_previousSessions(previous);
        let serialized = record.write_to_bytes().unwrap();
        serialized
    }

    pub fn decrypt(&mut self, ciphertext: &SignalMessage) -> Result<Vec<u8>, SignalError> {
        macro_rules! try_decrypt {
            ($state:expr, $ciphertext:expr) => {
                let res = $state.decrypt($ciphertext);
                match res {
                    Ok(plaintext) => { return Ok(plaintext); },
                    Err(e) => {
                        // TODO: capture and combine errors some how
                        println!("{:?}", e);
                    }
                }
            }
        }

        try_decrypt!(self.state, ciphertext);
        for state in self.previous.iter_mut() {
            try_decrypt!(state, ciphertext);
        }
        Err(SignalError::NoValidSessions)
    }

    pub fn has_session_state(&self, version: u32, alice_base_key: &Vec<u8>) -> bool {
        if self.state.0.get_sessionVersion() == version && self.state.0.get_aliceBaseKey() == &alice_base_key[..] {
            return true;
        }

        for state in self.previous.iter() {
            if state.0.get_sessionVersion() == version && state.0.get_aliceBaseKey() == &alice_base_key[..] {
                return true;
            }
        }
        false
    }

    pub fn push_state(&mut self, state: SessionState) {
        if ! self.fresh {
            self.promote_state(state);
        } else {
            self.state = state;
        }
    }

    fn promote_state(&mut self, promoted_state: SessionState) {
        let old_state = mem::replace(&mut self.state, promoted_state);
        self.previous.push_front(old_state);
        if self.previous.len() > ARCHIVED_STATES_MAX_LENGTH {
            self.previous.pop_back();
        }
    }
}
