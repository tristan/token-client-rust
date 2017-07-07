use eth::{Address,SecretKey};
use signal::protocol::{SignalProtocolStore,SignalProtocolAddress};
use signal::keys::{IdentityKeyPair};
use signal::state::{PreKeyRecord, SignedPreKeyRecord};
use ::{request,signed_request,Method};
use json::JsonValue;
use signal::SignalService;
use rustc_serialize::base64::{ToBase64, STANDARD};
use curl::easy::{List};
use signal::message::{PreKeySignalMessage,SignalMessage,DataMessage,TokenMessage};

pub struct ChatService {
    store: SignalProtocolStore,
    base_url: String,
    signing_key: SecretKey,
    token_id: Address,
    password: String
}

#[allow(dead_code)]
pub enum PushServiceType { APN, GCM }

impl ChatService {
    pub fn new(store: &SignalProtocolStore, base_url: &str, signing_key: &SecretKey, token_id: &Address, password: &str) -> ChatService {
        ChatService {
            store: store.clone(),
            base_url: base_url.to_string(),
            signing_key: signing_key.clone(),
            token_id: token_id.clone(),
            password: password.to_string()
        }
    }

    fn get_base_headers(&self) -> List {
        let auth = format!("Authorization: Basic {}", format!("{}.1:{}", self.token_id.to_string(), self.password).as_bytes().to_base64(STANDARD));
        let mut headers = List::new();
        headers.append(auth.as_str()).unwrap();
        headers.append("X-Signal-Agent: Token-Client-Rust").unwrap();
        headers
    }

    pub fn bootstrap_account(&self,
                             identity_key: &IdentityKeyPair,
                             last_resort_key: &PreKeyRecord,
                             prekeys: &Vec<PreKeyRecord>,
                             signed_prekey: &SignedPreKeyRecord,
                             registration_id: u32,
                             signaling_key: &[u8;52]
    ) -> Result<(), (String)> {

        let mut prekeyobjs = Vec::with_capacity(prekeys.len());
        for record in prekeys {
            prekeyobjs.push(object!{
                "keyId" => record.get_id(),
                "publicKey" => record.get_public_key().serialize().to_base64(STANDARD)
            });
        }

        let result = signed_request(&self.signing_key,
                                    Method::PUT,
                                    self.base_url.as_str(),
                                    "/v1/accounts/bootstrap",
                                    Some(self.get_base_headers()),
                                    Some(&object!{
                                        "identityKey" => identity_key.get_public_key().serialize().to_base64(STANDARD),
                                        "preKeys" => JsonValue::Array(prekeyobjs),
                                        "lastResortKey" => object!{
                                            "keyId" => last_resort_key.get_id(),
                                            "publicKey" => last_resort_key.get_public_key().serialize().to_base64(STANDARD)
                                        },
                                        "signedPreKey" => object!{
                                            "keyId" => signed_prekey.get_id(),
                                            "publicKey" => signed_prekey.get_public_key().serialize().to_base64(STANDARD),
                                            "signature" => signed_prekey.get_signature().to_base64(STANDARD)
                                        },
                                        "signalingKey" => signaling_key.to_base64(STANDARD),
                                        "registrationId" => registration_id,
                                        "password" => self.password.as_str()
                                    }));
        match result {
            Ok(val) => {
                match val {
                    None => {
                        Ok(())
                    },
                    Some(val) => {
                        // 204 is expected, so this means there's an error
                        Err(val.to_string())
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn create_account(&self, registration_id: u32, signaling_key: &[u8;52]) -> Result<(), String> {
        let result = signed_request(&self.signing_key,
                                    Method::PUT,
                                    self.base_url.as_str(),
                                    "/v1/accounts",
                                    Some(self.get_base_headers()),
                                    Some(&object!{
                                        "signalingKey" => signaling_key.to_base64(STANDARD),
                                        "registrationId" => registration_id,
                                        "voice" => false,
                                        "name" => self.signing_key.address().to_string(),
                                        //"video" => false,
                                        "fetchesMessages" => true
                                    }));
        match result {
            Ok(val) => {
                match val {
                    None => {
                        Ok(())
                    },
                    Some(val) => {
                        // 204 is expected, so this means there's an error
                        Err(val.to_string())
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn deregister_push_notifications(&self, service: PushServiceType) -> Result<(), String> {
        let result = signed_request(&self.signing_key,
                                    Method::DELETE,
                                    self.base_url.as_str(),
                                    match service {
                                        PushServiceType::APN => "/v1/accounts/apn/",
                                        PushServiceType::GCM => "/v1/accounts/gcm/"
                                    },
                                    Some(self.get_base_headers()),
                                    None);
        match result {
            Ok(val) => {
                match val {
                    None => {
                        Ok(())
                    },
                    Some(val) => {
                        // 204 is expected, so this means there's an error
                        Err(val.to_string())
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

fn strip_message_body_padding(version: u32, padded_message: &[u8]) -> Vec<u8> {
    if version > 2 {
        for (i, c) in padded_message.iter().enumerate().rev() {
            if *c == (0x80 as u8) {
                return padded_message[..i].to_vec();
            } else if *c != (0x00 as u8) {
                println!("Padding byte is malformed, returning unstripped padding.");
                return padded_message.to_vec();
            }
        }
        // empty???
        return Vec::new();
    }
    padded_message.to_vec()
}

pub fn process_envelope(store: &mut SignalProtocolStore, envelope: &SignalService::Envelope)
                        -> Result<(SignalProtocolAddress,i64,Option<TokenMessage>), String> {
    let env_type = envelope.get_field_type() as u32;
    let remote_address = SignalProtocolAddress::new(
        envelope.get_source(),
        envelope.get_sourceDevice());
    let timestamp = envelope.get_timestamp() as i64;

    if env_type == 5 {
        // RECEIPT
        return Ok((remote_address,timestamp,None));
    }
    let plaintext = if env_type == 1 {
        // CIPHERTEXT (a.k.a. SignalMessage)

        let ciphertext = if envelope.has_content() {
            envelope.get_content()
        } else {
            envelope.get_legacyMessage()
        };

        if ! store.contains_session(&remote_address) {
            return Err(format!("No session for: {}", remote_address.get_address()));
        }
        let mut session = store.load_session(&remote_address);
        let sm = SignalMessage::deserialize(ciphertext)?;
        let plaintext = match session.decrypt(&sm) {
            Ok(val) => {
                strip_message_body_padding(session.get_session_version(), &val)
            },
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };

        store.store_session(&remote_address, &session);

        plaintext
    } else if env_type == 3 {
        // PREKEY_BUNDLE (a.k.a. PreKeySignalMessage)

        let ciphertext = if envelope.has_content() {
            envelope.get_content()
        } else {
            envelope.get_legacyMessage()
        };

        let mut session = store.load_session(&remote_address);
        let pksm = PreKeySignalMessage::deserialize(ciphertext)?;
        let unsigned_prekey_id = match store.process_prekey_message(&remote_address, &mut session, &pksm) {
            Ok(val) => val,
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };
        let plaintext = match session.decrypt(&pksm.get_whisper_message()) {
            Ok(val) => {
                strip_message_body_padding(session.get_session_version(), &val)
            },
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };
        store.store_session(&remote_address, &session);
        if unsigned_prekey_id.is_some() {
            let unsigned_prekey_id = unsigned_prekey_id.unwrap();
            store.remove_prekey(unsigned_prekey_id);
        }

        plaintext
    } else {
        return Err(format!("Reached unexpected branch"));
    };

    let dm = if envelope.has_content() {
        return Err(format!("not implemented"));
    } else if envelope.has_legacyMessage() {
        DataMessage::deserialize(&plaintext, timestamp)
    } else {
        return Err(format!("Reached unexpected branch"));
    };

    let tm = TokenMessage::new(&remote_address.get_address(),
                               &dm.get_body(),
                               timestamp);
    Ok((remote_address, timestamp, Some(tm)))

}

mod sender;
mod receiver;
mod_path! WebSocketResources (concat!(env!("OUT_DIR"), "/WebSocketResources.rs"));
mod websocket;
