extern crate crypto;

use super::message::{CURRENT_VERSION, CipherTextMessage,
                     SignalMessage, PreKeySignalMessage};
use super::protocol::SignalError;
use super::cipher;
use super::keys::{IdentityKeyPair, ECKeyPair, ECPrivateKey, ECPublicKey};
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

    pub fn get_message_keys(&self) -> MessageKeys {
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

    pub fn new() -> SessionState {
        SessionState(LocalStorageProtocol::SessionStructure::new())
    }

    pub fn initialize_as_alice(
        &mut self,
        our_identity_key: &IdentityKeyPair,
        our_base_key: &ECKeyPair,
        their_identity_key: &ECPublicKey,
        their_signed_prekey: &ECPublicKey,
        their_ratchet_key: &ECPublicKey,
        their_one_time_prekey: Option<&ECPublicKey>
    ) {

        self.0.set_sessionVersion(CURRENT_VERSION);
        self.0.set_remoteIdentityPublic(their_identity_key.serialize().to_vec());
        self.0.set_localIdentityPublic(our_identity_key.get_public_key().serialize().to_vec());

        #[cfg(test)] // for test_initialise_as_alice to keep ratchet key specific
        let sending_ratchet_key = ECKeyPair::new(
            &vec![0, 79, 166, 98, 173, 168, 60, 31, 47, 51, 107, 63, 210, 86, 183, 120, 226, 144, 204, 194, 208, 200, 22, 203, 36, 15, 106, 189, 224, 155, 188, 80],
            &vec![5, 51, 47, 84, 82, 41, 161, 168, 53, 167, 112, 159, 75, 178, 40, 65, 107, 87, 89, 18, 142, 238, 224, 124, 189, 218, 222, 201, 73, 110, 126, 44, 24]
            );
        #[cfg(not(test))]
        let sending_ratchet_key = ECKeyPair::generate();

        let mut secrets: Vec<u8> = vec![0xff; 32];
        secrets.extend(our_identity_key.get_private_key().calculate_agreement(their_signed_prekey));
        secrets.extend(our_base_key.get_private_key().calculate_agreement(their_identity_key));
        secrets.extend(our_base_key.get_private_key().calculate_agreement(their_signed_prekey));
        match their_one_time_prekey {
            Some(key) => {
                secrets.extend(our_base_key.get_private_key().calculate_agreement(key));
            },
            None => {}
        };

        let (root_key_bytes, chain_key_bytes) = calculate_derived_keys(&secrets);
        let (new_root_key_bytes, sending_chain_key_bytes) =
            create_chain(&root_key_bytes, &their_ratchet_key, &sending_ratchet_key);

        // add_receiver_chain
        self.add_receiver_chain(&their_ratchet_key, &chain_key_bytes, 0);
        // set_sender_chain
        self.set_sender_chain(&sending_ratchet_key, &sending_chain_key_bytes, 0);

        // set_root_key
        self.0.set_rootKey(new_root_key_bytes.to_vec());
    }

    pub fn initialize_as_bob(
        &mut self,
        our_identity_key: &IdentityKeyPair,
        our_signed_prekey: &ECKeyPair,
        our_ratchet_key: &ECKeyPair,
        our_one_time_prekey: Option<ECKeyPair>,
        their_identity_key: &ECPublicKey,
        their_base_key: &ECPublicKey
    ) {
        self.0.set_sessionVersion(CURRENT_VERSION);
        self.0.set_remoteIdentityPublic(their_identity_key.serialize().to_vec());
        self.0.set_localIdentityPublic(our_identity_key.get_public_key().serialize().to_vec());

        let mut secrets: Vec<u8> = vec![0xff; 32];
        secrets.extend(our_signed_prekey.get_private_key().calculate_agreement(their_identity_key));
        secrets.extend(our_identity_key.get_private_key().calculate_agreement(their_base_key));
        secrets.extend(our_signed_prekey.get_private_key().calculate_agreement(their_base_key));
        match our_one_time_prekey {
            Some(key) => {
                secrets.extend(key.get_private_key().calculate_agreement(their_base_key));
            },
            None => {}
        };

        let (root_key_bytes, chain_key_bytes) = calculate_derived_keys(&secrets);

        // set_sender_chain
        self.set_sender_chain(&our_ratchet_key, &chain_key_bytes, 0);

        // set_root_key
        self.0.set_rootKey(root_key_bytes.to_vec());
    }

    pub fn get_session_version(&self) -> u32 {
        self.0.get_sessionVersion()
    }

    pub fn set_local_registration_id(&mut self, registration_id: u32) {
        self.0.set_localRegistrationId(registration_id);
    }

    pub fn set_remote_registration_id(&mut self, registration_id: u32) {
        self.0.set_remoteRegistrationId(registration_id);
    }

    pub fn get_remote_registration_id(&self) -> u32 {
        self.0.get_remoteRegistrationId()
    }

    pub fn set_alice_base_key(&mut self, base_key: Vec<u8>) {
        self.0.set_aliceBaseKey(base_key);
    }

    pub fn get_alice_base_key(&self) -> Option<Vec<u8>> {
        if self.0.has_aliceBaseKey() {
            Some(self.0.get_aliceBaseKey().to_vec())
        } else {
            None
        }
    }

    pub fn get_previous_counter(&self) -> u32 {
        self.0.get_previousCounter()
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
            println!("BadMac @ {:?}", ciphertext.serialize());
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
            Some((chain, _)) => {
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

    pub fn get_sender_chain_key(&self) -> ChainKey {
        let chainkey = self.0.get_senderChain().get_chainKey();
        ChainKey::create(self.get_session_version(),
                         &chainkey.get_key().to_vec(),
                         chainkey.get_index())
    }

    pub fn set_sender_chain_key(&mut self, chainkey: &ChainKey) {
        let mut cks = LocalStorageProtocol::SessionStructure_Chain_ChainKey::new();
        cks.set_key(chainkey.key.to_vec());
        cks.set_index(chainkey.index);

        self.0.mut_senderChain().set_chainKey(cks);
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
            Some((chain, _)) => {
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

    pub fn set_unacknowledged_prekey_message(&mut self, prekey_id: Option<u32>, signed_prekey_id: u32,
                                         base_key: &ECPublicKey) {
        let mut pending = LocalStorageProtocol::SessionStructure_PendingPreKey::new();
        pending.set_signedPreKeyId(signed_prekey_id as i32);
        pending.set_baseKey(base_key.serialize());
        match prekey_id {
            Some(id) => {
                pending.set_preKeyId(id);
            },
            None => {}
        };
        self.0.set_pendingPreKey(pending);
    }

    pub fn get_sender_ratchet_key(&self) -> ECPublicKey {
        ECPublicKey::deserialize(&self.0.get_senderChain().get_senderRatchetKey().to_vec())
    }
}

// TODO: move this to state.rs
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

    pub fn encrypt(&mut self, plaintext: &Vec<u8>) -> Result<Box<CipherTextMessage>, SignalError> {
        let chainkey = self.state.get_sender_chain_key();
        let messagekeys = chainkey.get_message_keys();
        let sender_ephemeral = self.state.get_sender_ratchet_key();
        let previous_counter = self.state.get_previous_counter();
        let session_version = self.state.get_session_version();

        // get_cipher_text
        let ciphertextbody = match {
            if session_version >= 3 {
                cipher::encrypt_cbc(
                    &plaintext,
                    &messagekeys.cipher_key,
                    &messagekeys.iv)
            } else {
                cipher::encrypt_ctr(
                    &plaintext,
                    &messagekeys.cipher_key,
                    messagekeys.counter)
            }
        } {
            Ok(body) => body,
            Err(_) => {
                // TODO: be more specific
                return Err(SignalError::InvalidMessage);
            }
        };

        let ciphertextmsg = SignalMessage::new(
            session_version, &messagekeys.mac_key,
            &sender_ephemeral, chainkey.index,
            previous_counter, &ciphertextbody,
            &ECPublicKey::deserialize(
                &self.state.0.get_localIdentityPublic().to_vec()),
            &ECPublicKey::deserialize(
                &self.state.0.get_remoteIdentityPublic().to_vec()));

        let res: Box<CipherTextMessage> = if self.state.0.has_pendingPreKey() {
            let pending_prekey = self.state.0.get_pendingPreKey();
            let prekey_id = if pending_prekey.has_preKeyId() {
                Some(pending_prekey.get_preKeyId())
            } else {
                None
            };
            Box::new(PreKeySignalMessage::new(
                session_version, self.state.0.get_localRegistrationId(),
                prekey_id, Some(pending_prekey.get_signedPreKeyId() as u32),
                &ECPublicKey::deserialize(
                    &pending_prekey.get_baseKey().to_vec()),
                &ECPublicKey::deserialize(
                    &self.state.0.get_localIdentityPublic().to_vec()),
                ciphertextmsg))

        } else {
            Box::new(ciphertextmsg)
        };

        self.state.set_sender_chain_key(&chainkey.get_next_chain_key());

        Ok(res)
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

    pub fn get_session_state(&self) -> &SessionState {
        &self.state
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_chain_key_message_key_generation() {
        let chainkey = ChainKey {
            kdf_version: 3,
            key: vec![0xf1, 0xbf, 0xce, 0x96, 0x29, 0xfe, 0xf7, 0x1f, 0x66, 0x6b, 0x1a, 0xbf, 0x70, 0x29, 0x78, 0xd3,
                      0x69, 0x27, 0x10, 0xb6, 0xb1, 0x3e, 0x66, 0xa4, 0x09, 0x9a, 0x1d, 0x29, 0x3e, 0xaa, 0x4b, 0x17],
            index: 0
        };

        let messagekeys = chainkey.get_message_keys();

        assert_eq!(messagekeys.cipher_key, vec![0x8d, 0x24, 0x3d, 0xe6, 0x95, 0x3e, 0xa1, 0x00, 0x65, 0x12, 0x76, 0x18, 0x16, 0x2b, 0xef, 0x1b,
                                                0x8c, 0x9c, 0x5b, 0xea, 0xca, 0xcd, 0xbd, 0xb7, 0x5d, 0x27, 0x2a, 0x75, 0x0d, 0x56, 0x70, 0x51]);
        assert_eq!(messagekeys.mac_key, vec![0xb5, 0xb9, 0xc7, 0xf3, 0xbb, 0x02, 0x12, 0x28, 0xeb, 0x6c, 0x5e, 0xb4, 0xd7, 0x50, 0x10, 0x00,
                                             0x44, 0x01, 0x7c, 0x34, 0x06, 0x16, 0x52, 0x59, 0xd6, 0x4b, 0xe8, 0x8d, 0x53, 0xb2, 0x17, 0x7b]);
        assert_eq!(messagekeys.iv, vec![0xa1, 0x84, 0x23, 0xd0, 0x0f, 0xd6, 0xd0, 0x2e, 0x9e, 0x6c, 0x27, 0x32, 0xf1, 0xa5, 0x15, 0x9a]);
        assert_eq!(messagekeys.counter, 0);
    }

    use protobuf;
    use super::*;
    use ::signal::state::*;

    #[test]
    fn test_initialise_as_alice() {

        let prekey = PreKeyBundle::new(
            8462, 1, Some(31337),
            Some(&ECPublicKey::deserialize(
                &vec![0x05, 0x5a, 0x75, 0x64, 0xad, 0xcf, 0xa8, 0x34, 0xc7, 0x02, 0xd3, 0x7b, 0x31, 0x32, 0x27, 0xe0,
                      0x95, 0x4a, 0xb2, 0x0a, 0x04, 0xf3, 0x6a, 0x01, 0x1e, 0xd7, 0xa1, 0xed, 0xd2, 0xf4, 0x91, 0xfe,
                      0x2a])),
            22,
            &ECPublicKey::deserialize(
                &vec![0x05, 0xeb, 0x84, 0xe4, 0x46, 0x98, 0x71, 0xec, 0xf3, 0x57, 0x55, 0xf3, 0x3b, 0x60, 0x31, 0xe8,
                      0x21, 0x20, 0xc3, 0x59, 0x75, 0x01, 0xa5, 0x64, 0x60, 0x15, 0xf1, 0x17, 0x2e, 0xa7, 0xe1, 0x56,
                      0x3d]),
            &vec![0x50, 0x0b, 0xf7, 0x40, 0xc5, 0x9b, 0x13, 0x8b, 0xe8, 0x54, 0x1a, 0x80, 0xdd, 0xdd, 0x3a, 0x20,
                  0xc5, 0xcb, 0x21, 0xd7, 0x59, 0x8e, 0xfb, 0xc2, 0xbe, 0xbe, 0xa5, 0x22, 0x72, 0x2a, 0xd6, 0xf5,
                  0xc5, 0xb4, 0xee, 0x84, 0xb6, 0xd4, 0xd6, 0x53, 0x89, 0x7f, 0x47, 0xca, 0xf2, 0x52, 0xcd, 0x55,
                  0x42, 0x40, 0xcb, 0x0a, 0x51, 0xcb, 0x2b, 0xca, 0x4c, 0x97, 0x51, 0x47, 0xc8, 0x68, 0x30, 0x8f],
            &ECPublicKey::deserialize(
                &vec![0x05, 0x72, 0x7e, 0x15, 0x7b, 0xe2, 0x8b, 0x78, 0xd9, 0xfe, 0x0d, 0x58, 0x89, 0xb2, 0x6b, 0xb6,
                      0xff, 0x55, 0xdd, 0xf8, 0x07, 0x6a, 0x79, 0xdc, 0xe1, 0x34, 0xb4, 0x43, 0xa9, 0x55, 0xbb, 0x1c,
                      0x60]));
        let our_base_key = ECKeyPair::new(
            &vec![0x50, 0xc4, 0x82, 0x1b, 0x56, 0x97, 0xfd, 0x43, 0x9e, 0x8e, 0xf3, 0x5e, 0x2a, 0x71, 0x42, 0xd1,
                  0x81, 0x78, 0x71, 0x8e, 0x32, 0x89, 0xb7, 0x50, 0x87, 0x98, 0x8e, 0x05, 0x59, 0x62, 0xa1, 0x4c],
            &vec![0x05, 0xb0, 0x25, 0x11, 0x76, 0x20, 0xaa, 0x34, 0x95, 0x07, 0x60, 0xc3, 0xf4, 0xca, 0x39, 0xa3,
                  0x16, 0x10, 0x82, 0x94, 0x2d, 0xda, 0xae, 0x39, 0xf5, 0xfc, 0x99, 0x8e, 0x2d, 0xaf, 0x65, 0x8c,
                  0x25]);

        // data generated using the java library
        let expected_structure_bytes = vec![
            0x08, 0x03, 0x12, 0x21, 0x05, 0xb9, 0x26, 0x14, 0x8a, 0x13, 0x36, 0x25, 0x6a, 0x3c, 0xfb, 0xcd,
            0x59, 0x0f, 0x89, 0x4c, 0x3e, 0xc3, 0x82, 0x30, 0xc8, 0xa0, 0xd5, 0x82, 0x36, 0x05, 0x08, 0xe2,
            0xcc, 0x2f, 0xc7, 0xde, 0x18, 0x1a, 0x21, 0x05, 0x72, 0x7e, 0x15, 0x7b, 0xe2, 0x8b, 0x78, 0xd9,
            0xfe, 0x0d, 0x58, 0x89, 0xb2, 0x6b, 0xb6, 0xff, 0x55, 0xdd, 0xf8, 0x07, 0x6a, 0x79, 0xdc, 0xe1,
            0x34, 0xb4, 0x43, 0xa9, 0x55, 0xbb, 0x1c, 0x60, 0x22, 0x20, 0xfc, 0x82, 0xe1, 0x40, 0x75, 0x4b,
            0x27, 0x45, 0x3f, 0x1a, 0xb4, 0x99, 0x72, 0x64, 0x86, 0xfa, 0xad, 0x7c, 0x75, 0x09, 0x09, 0x8b,
            0x1c, 0x82, 0x25, 0x05, 0xcd, 0x12, 0x59, 0x8f, 0x92, 0xda, 0x32, 0x6b, 0x0a, 0x21, 0x05, 0x33,
            0x2f, 0x54, 0x52, 0x29, 0xa1, 0xa8, 0x35, 0xa7, 0x70, 0x9f, 0x4b, 0xb2, 0x28, 0x41, 0x6b, 0x57,
            0x59, 0x12, 0x8e, 0xee, 0xe0, 0x7c, 0xbd, 0xda, 0xde, 0xc9, 0x49, 0x6e, 0x7e, 0x2c, 0x18, 0x12,
            0x20, 0x00, 0x4f, 0xa6, 0x62, 0xad, 0xa8, 0x3c, 0x1f, 0x2f, 0x33, 0x6b, 0x3f, 0xd2, 0x56, 0xb7,
            0x78, 0xe2, 0x90, 0xcc, 0xc2, 0xd0, 0xc8, 0x16, 0xcb, 0x24, 0x0f, 0x6a, 0xbd, 0xe0, 0x9b, 0xbc,
            0x50, 0x1a, 0x24, 0x08, 0x00, 0x12, 0x20, 0x41, 0x3a, 0xca, 0x55, 0x74, 0x52, 0x0c, 0x99, 0xcf,
            0x44, 0xef, 0xb3, 0x6a, 0xaf, 0x60, 0x50, 0x86, 0xe7, 0x89, 0xa9, 0xa3, 0xc5, 0xb9, 0x3e, 0xe7,
            0x07, 0xca, 0x77, 0xff, 0x0d, 0x83, 0xb3, 0x3a, 0x49, 0x0a, 0x21, 0x05, 0xeb, 0x84, 0xe4, 0x46,
            0x98, 0x71, 0xec, 0xf3, 0x57, 0x55, 0xf3, 0x3b, 0x60, 0x31, 0xe8, 0x21, 0x20, 0xc3, 0x59, 0x75,
            0x01, 0xa5, 0x64, 0x60, 0x15, 0xf1, 0x17, 0x2e, 0xa7, 0xe1, 0x56, 0x3d, 0x1a, 0x24, 0x08, 0x00,
            0x12, 0x20, 0xe0, 0x5e, 0x65, 0x4a, 0x25, 0x3b, 0x00, 0x2d, 0x3f, 0xc8, 0x43, 0xdb, 0x59, 0x3d,
            0xe5, 0x27, 0x0c, 0xaa, 0x32, 0xa2, 0x21, 0xda, 0x5b, 0xc6, 0xed, 0x5e, 0xcb, 0x7c, 0xc4, 0x69,
            0x96, 0x64, 0x4a, 0x29, 0x08, 0xe9, 0xf4, 0x01, 0x12, 0x21, 0x05, 0xb0, 0x25, 0x11, 0x76, 0x20,
            0xaa, 0x34, 0x95, 0x07, 0x60, 0xc3, 0xf4, 0xca, 0x39, 0xa3, 0x16, 0x10, 0x82, 0x94, 0x2d, 0xda,
            0xae, 0x39, 0xf5, 0xfc, 0x99, 0x8e, 0x2d, 0xaf, 0x65, 0x8c, 0x25, 0x18, 0x16, 0x50, 0x8e, 0x42,
            0x58, 0x80, 0x61, 0x6a, 0x21, 0x05, 0xb0, 0x25, 0x11, 0x76, 0x20, 0xaa, 0x34, 0x95, 0x07, 0x60,
            0xc3, 0xf4, 0xca, 0x39, 0xa3, 0x16, 0x10, 0x82, 0x94, 0x2d, 0xda, 0xae, 0x39, 0xf5, 0xfc, 0x99,
            0x8e, 0x2d, 0xaf, 0x65, 0x8c, 0x25];

        let their_signed_prekey = prekey.get_signed_prekey();

        let id_keypair = IdentityKeyPair::new(
            &vec![0x78, 0x96, 0xe0, 0xbb, 0xdc, 0xcb, 0x3c, 0x19, 0x3c, 0x05, 0x55, 0xff, 0x4e, 0xec, 0x9e, 0xb3,
                  0xad, 0x25, 0x8a, 0xf2, 0xaa, 0x5e, 0x10, 0x79, 0xb7, 0x7f, 0x37, 0x6a, 0xec, 0x40, 0xb4, 0x46],
            &vec![0x05, 0xb9, 0x26, 0x14, 0x8a, 0x13, 0x36, 0x25, 0x6a, 0x3c, 0xfb, 0xcd, 0x59, 0x0f, 0x89, 0x4c,
                  0x3e, 0xc3, 0x82, 0x30, 0xc8, 0xa0, 0xd5, 0x82, 0x36, 0x05, 0x08, 0xe2, 0xcc, 0x2f, 0xc7, 0xde,
                  0x18]);

        let mut session = SessionState::new();
        session.initialize_as_alice(
            &id_keypair,
            &our_base_key,
            prekey.get_identity_key(),
            &their_signed_prekey,
            &their_signed_prekey,
            prekey.get_prekey()
        );

        session.set_unacknowledged_prekey_message(prekey.get_prekey_id(), prekey.get_signed_prekey_id(), our_base_key.get_public_key());
        session.set_local_registration_id(12416);
        session.set_remote_registration_id(prekey.get_registration_id());
        session.set_alice_base_key(our_base_key.get_public_key().serialize().to_vec());

        let expected_structure: LocalStorageProtocol::SessionStructure = protobuf::core::parse_from_bytes(&expected_structure_bytes).unwrap();

        assert_eq!(expected_structure.get_sessionVersion(), session.0.get_sessionVersion());
        assert_eq!(expected_structure.get_localIdentityPublic(), session.0.get_localIdentityPublic());
        assert_eq!(expected_structure.get_remoteIdentityPublic(), session.0.get_remoteIdentityPublic());
        assert_eq!(expected_structure.get_rootKey(), session.0.get_rootKey());
        assert_eq!(expected_structure.get_previousCounter(), session.0.get_previousCounter());

        assert_eq!(expected_structure.get_senderChain().get_senderRatchetKey(), session.0.get_senderChain().get_senderRatchetKey());
        assert_eq!(expected_structure.get_senderChain().get_senderRatchetKeyPrivate(), session.0.get_senderChain().get_senderRatchetKeyPrivate());
        assert_eq!(expected_structure.get_senderChain().get_chainKey().get_key(),
                   session.0.get_senderChain().get_chainKey().get_key());
        assert_eq!(expected_structure.get_senderChain().get_chainKey().get_index(),
                   session.0.get_senderChain().get_chainKey().get_index());
        let ex_mks = expected_structure.get_senderChain().get_messageKeys();
        let mks = session.0.get_senderChain().get_messageKeys();
        assert_eq!(ex_mks.len(), mks.len());
        for (ex_mk, mk) in ex_mks.iter().zip(mks.iter()) {
            assert_eq!(ex_mk.get_index(), mk.get_index());
            assert_eq!(ex_mk.get_cipherKey(), mk.get_cipherKey());
            assert_eq!(ex_mk.get_macKey(), mk.get_macKey());
            assert_eq!(ex_mk.get_iv(), mk.get_iv());
            assert_eq!(ex_mk.get_unknown_fields(), mk.get_unknown_fields());
            assert_eq!(ex_mk.get_cached_size(), mk.get_cached_size());
        }

        let excl = expected_structure.get_receiverChains();
        let cl = session.0.get_receiverChains();
        assert_eq!(excl.len(), cl.len());
        for (ex_chain, chain) in excl.iter().zip(cl.iter()) {
            assert_ne!(ex_chain as *const LocalStorageProtocol::SessionStructure_Chain, chain as *const LocalStorageProtocol::SessionStructure_Chain);
            assert_eq!(ex_chain.get_senderRatchetKey(), chain.get_senderRatchetKey());
            assert_eq!(ex_chain.get_senderRatchetKeyPrivate(), chain.get_senderRatchetKeyPrivate());
            let ex_ck = ex_chain.get_chainKey();
            let ck = chain.get_chainKey();
            assert_eq!(ex_ck.get_key(), ck.get_key());
            assert_eq!(ex_ck.get_index(), ck.get_index());
            assert_eq!(ex_ck.get_unknown_fields(), ck.get_unknown_fields());
            assert_eq!(ex_ck.get_cached_size(), ck.get_cached_size());
            let ex_mks = ex_chain.get_messageKeys();
            let mks = chain.get_messageKeys();
            assert_eq!(ex_mks.len(), mks.len());
            for (ex_mk, mk) in ex_mks.iter().zip(mks.iter()) {
                assert_eq!(ex_mk.get_index(), mk.get_index());
                assert_eq!(ex_mk.get_cipherKey(), mk.get_cipherKey());
                assert_eq!(ex_mk.get_macKey(), mk.get_macKey());
                assert_eq!(ex_mk.get_iv(), mk.get_iv());
                assert_eq!(ex_mk.get_unknown_fields(), mk.get_unknown_fields());
                assert_eq!(ex_mk.get_cached_size(), mk.get_cached_size());
            }
        }

        let ex_pke = expected_structure.get_pendingKeyExchange();
        let pke = session.0.get_pendingKeyExchange();
        assert_eq!(ex_pke.get_sequence(), pke.get_sequence());
        assert_eq!(ex_pke.get_localBaseKey(), pke.get_localBaseKey());
        assert_eq!(ex_pke.get_localBaseKeyPrivate(), pke.get_localBaseKeyPrivate());
        assert_eq!(ex_pke.get_localRatchetKey(), pke.get_localRatchetKey());
        assert_eq!(ex_pke.get_localRatchetKeyPrivate(), pke.get_localRatchetKeyPrivate());
        assert_eq!(ex_pke.get_localIdentityKey(), pke.get_localIdentityKey());
        assert_eq!(ex_pke.get_localIdentityKeyPrivate(), pke.get_localIdentityKeyPrivate());
        assert_eq!(ex_pke.get_unknown_fields(), pke.get_unknown_fields());
        assert_eq!(ex_pke.get_cached_size(), pke.get_cached_size());

        let ex_ppk = expected_structure.get_pendingPreKey();
        let ppk = session.0.get_pendingPreKey();
        assert_eq!(ex_ppk.get_preKeyId(), ppk.get_preKeyId());
        assert_eq!(ex_ppk.get_signedPreKeyId(), ppk.get_signedPreKeyId());
        assert_eq!(ex_ppk.get_baseKey(), ppk.get_baseKey());
        assert_eq!(ex_ppk.get_unknown_fields(), ppk.get_unknown_fields());
        assert_eq!(ex_ppk.get_cached_size(), ppk.get_cached_size());


        assert_eq!(expected_structure.get_remoteRegistrationId(), session.0.get_remoteRegistrationId());
        assert_eq!(expected_structure.get_localRegistrationId(), session.0.get_localRegistrationId());
        assert_eq!(expected_structure.get_needsRefresh(), session.0.get_needsRefresh());
        assert_eq!(expected_structure.get_aliceBaseKey(), session.0.get_aliceBaseKey());

        assert_eq!(expected_structure.get_unknown_fields(), session.0.get_unknown_fields());
        assert_eq!(expected_structure.get_cached_size(), session.0.get_cached_size());

        let serialized = session.0.write_to_bytes().unwrap();
        let reserialized = expected_structure.write_to_bytes().unwrap();
        assert_eq!(serialized, reserialized);

        // TODO: figure out why this doesn't work!
        // there seems to be some differences in the basic structure
        // of the protobuf message that is ignored when parsed
        // and ignored again when serialized back out.
        //assert_eq!(serialized, expected_structure_bytes);
    }
}
