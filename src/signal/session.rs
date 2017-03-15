use super::protocol::{SignalProtocolAddress, SignalProtocolStore, SignalError};
use super::message::{CipherTextMessage,SignalMessage,PreKeySignalMessage};

pub fn decrypt_prekey_message(store: &mut SignalProtocolStore,
                              remote_address: &SignalProtocolAddress,
                              ciphertext: &PreKeySignalMessage)
                              -> Result<Vec<u8>, SignalError> {
    // TODO: locking
    let mut record = store.load_session(&remote_address);
    let process_result = store.process_prekey_message(
        remote_address,
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
