
use super::protocol::{SignalProtocolAddress, SignalProtocolStore, SignalError};
use super::message::{PreKeySignalMessage};
use super::ratchet::{SessionRecord, SessionState};
use ::keys::{ECPublicKey};
use super::LocalStorageProtocol::SessionStructure;

pub struct SessionBuilder<'a> {
    remote_address: &'a SignalProtocolAddress,
    store: &'a mut SignalProtocolStore
}

impl<'a> SessionBuilder<'a> {
    pub fn new(store: &'a mut SignalProtocolStore, remote_address: &'a SignalProtocolAddress) -> SessionBuilder<'a> {
        SessionBuilder {
            remote_address: remote_address,
            store: store
        }
    }

    fn process(&mut self, session_record: &mut SessionRecord, message: &PreKeySignalMessage) -> Result<Option<u32>, SignalError> {
        let their_identity_key = message.get_identity_key();
        if ! self.store.is_trusted_identity(self.remote_address, &their_identity_key) {
            return Err(SignalError::UntrustedIdentity)
        }

        // process v3
        if session_record.has_session_state(message.get_version(), &message.get_base_key().serialize()) {
            // We've already setup a session for this V3 message, letting bundled message fall through...
            return Ok(None)
        }

        let spkr = self.store.load_signed_pre_key(
            message.get_signed_pre_key_id()).unwrap();
        let our_signed_pre_key = spkr.get_key_pair();

        let one_time_pre_key = match message.get_pre_key_id() {
            Some(id) => {
                match self.store.load_pre_key(id) {
                    Some(pre_key) => {
                        Some(pre_key.get_key_pair().clone())
                    },
                    None => None
                }
            },
            None => None
        };
        let mut session = SessionState::initialize_as_bob(
            &self.store.get_identity_key_pair(),
            &our_signed_pre_key,
            &our_signed_pre_key,
            one_time_pre_key,
            message.get_identity_key(),
            message.get_base_key()
        );

        session.set_local_registration_id(self.store.get_local_registration_id());
        session.set_remote_registration_id(message.get_registration_id());
        session.set_alice_base_key(message.get_base_key().serialize().to_vec());

        session_record.push_state(session);

        let unsigned_pre_key_id = match message.get_pre_key_id() {
            Some(id) => {
                if id != 0xFFFFFF { // Medium.MAX_VALUE
                    Ok(Some(id))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None)
        };

        self.store.store_identity(self.remote_address, their_identity_key);

        unsigned_pre_key_id
    }
}

pub struct SessionCipher<'a> {
    //remote_address: &'a SignalProtocolAddress,
    //store: &'a mut SignalProtocolStore,
    builder: SessionBuilder<'a>
}

impl<'a> SessionCipher<'a> {

    pub fn new(store: &'a mut SignalProtocolStore, remote_address: &'a SignalProtocolAddress) -> SessionCipher<'a> {
        let mut builder = SessionBuilder::new(store, remote_address);
        SessionCipher {
            builder: builder
        }
    }

    pub fn decrypt(&mut self, ciphertext: &PreKeySignalMessage) -> Result<Vec<u8>, SignalError> {
        // TODO: locking
        let mut record = self.builder.store.load_session(self.builder.remote_address);
        let process_result = self.builder.process(&mut record, ciphertext);
        match process_result {
            Ok(id) => {
                // decrypt message
                let plaintext = record.decrypt(ciphertext.get_whisper_message());
                // store session
                self.builder.store.store_session(
                    self.builder.remote_address,
                    &record);
                // remove pre key if a match was found
                match id {
                    Some(unsigned_pre_key_id) => {
                        self.builder.store.remove_pre_key(unsigned_pre_key_id)
                    },
                    None => {}
                };
                plaintext
            }
            Err(e) => Err(e)
        }
    }
}
