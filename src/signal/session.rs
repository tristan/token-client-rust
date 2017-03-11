use super::protocol::{SignalProtocolAddress, SignalProtocolStore, SignalError};
use super::message::{CipherTextMessage,SignalMessage,PreKeySignalMessage};
use super::state::{PreKeyBundle};
use super::ratchet::{SessionRecord, SessionState};
use super::keys::{ECKeyPair, ECPublicKey};
use super::curve::{curve25519_sign, curve25519_verify};
use super::LocalStorageProtocol::SessionStructure;

fn process_prekey_message(store: &mut SignalProtocolStore, remote_address: &SignalProtocolAddress,
                          session_record: &mut SessionRecord, message: &PreKeySignalMessage)
                          -> Result<Option<u32>, SignalError> {

    let their_identity_key = message.get_identity_key();
    if ! store.is_trusted_identity(remote_address, &their_identity_key) {
        return Err(SignalError::UntrustedIdentity)
    }

    // process v3
    if session_record.has_session_state(message.get_version(), &message.get_base_key().serialize()) {
        // We've already setup a session for this V3 message, letting bundled message fall through...
        return Ok(None)
    }

    let spkr = store.load_signed_pre_key(
        message.get_signed_pre_key_id()).unwrap();
    let our_signed_pre_key = spkr.get_key_pair();

    let one_time_pre_key = match message.get_pre_key_id() {
        Some(id) => {
            match store.load_pre_key(id) {
                Some(pre_key) => {
                    Some(pre_key.get_key_pair().clone())
                },
                None => None
            }
        },
        None => None
    };

    let mut session = SessionState::new();
    session.initialize_as_bob(
        &store.get_identity_key_pair(),
        &our_signed_pre_key,
        &our_signed_pre_key,
        one_time_pre_key,
        message.get_identity_key(),
        message.get_base_key()
    );

    session.set_local_registration_id(store.get_local_registration_id());
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

    store.store_identity(remote_address, their_identity_key);

    unsigned_pre_key_id
}

pub fn process_prekey_bundle(store: &mut SignalProtocolStore,
                             remote_address: &SignalProtocolAddress,
                             prekey: &PreKeyBundle)
                             -> Result<(), SignalError> {
    if ! store.is_trusted_identity(remote_address, prekey.get_identity_key()) {
        return Err(SignalError::UntrustedIdentity);
    }

    if ! curve25519_verify(prekey.get_identity_key().as_slice(),
                           &prekey.get_signed_prekey().serialize(),
                           &prekey.get_signed_prekey_signature_as_slice()) {
        return Err(SignalError::InvalidKey);
    }

    let mut session_record = store.load_session(remote_address);
    let our_base_key = ECKeyPair::generate();
    let their_signed_prekey = prekey.get_signed_prekey();

    let mut session = SessionState::new();
    session.initialize_as_alice(
        &store.get_identity_key_pair(),
        &our_base_key,
        prekey.get_identity_key(),
        &their_signed_prekey,
        &their_signed_prekey,
        prekey.get_prekey()
    );

    session.set_unacknowledged_prekey_message(prekey.get_prekey_id(), prekey.get_signed_prekey_id(), our_base_key.get_public_key());
    session.set_local_registration_id(store.get_local_registration_id());
    session.set_remote_registration_id(prekey.get_registration_id());
    session.set_alice_base_key(our_base_key.get_public_key().serialize().to_vec());

    session_record.push_state(session);

    store.store_session(remote_address, &session_record);
    store.store_identity(remote_address, prekey.get_identity_key());

    Ok(())
}

pub fn decrypt_prekey_message(store: &mut SignalProtocolStore,
                              remote_address: &SignalProtocolAddress,
                              ciphertext: &PreKeySignalMessage)
                              -> Result<Vec<u8>, SignalError> {
    // TODO: locking
    let mut record = store.load_session(&remote_address);
    let process_result = process_prekey_message(
        store, remote_address,
        &mut record, ciphertext);
    match process_result {
        Ok(id) => {
            // decrypt message
            let plaintext = record.decrypt(ciphertext.get_whisper_message());
            // store session
            store.store_session(
                &remote_address,
                &record);
            // remove pre key if a match was found
            match id {
                Some(unsigned_pre_key_id) => {
                    store.remove_pre_key(unsigned_pre_key_id)
                },
                None => {}
            };
            plaintext
        }
        Err(e) => Err(e)
    }
}

pub fn decrypt_whisper_message(store: &mut SignalProtocolStore,
                               remote_address: &SignalProtocolAddress,
                               ciphertext: &SignalMessage)
                               -> Result<Vec<u8>, SignalError> {
    if ! store.contains_session(&remote_address) {
        return Err(SignalError::NoSession);
    }
    let mut record = store.load_session(&remote_address);
    let plaintext = record.decrypt(&ciphertext);
    store.store_session(&remote_address, &record);

    plaintext
}

pub fn encrypt_message(store: &mut SignalProtocolStore,
                       remote_address: &SignalProtocolAddress,
                       plaintext: &Vec<u8>)
                       -> Result<Box<CipherTextMessage>, SignalError> {
    let mut session_record = store.load_session(&remote_address);

    session_record.encrypt(plaintext)
}

#[cfg(test)]
mod tests {


}
