use ::eth::{Address,SecretKey};
use ::signal::protocol::{SignalProtocolStore};
use ::signal::keys::{IdentityKeyPair};
use ::signal::state::{PreKeyRecord, SignedPreKeyRecord};
use super::{request,signed_request,Method};
use json::JsonValue;
use super::rustc_serialize::base64::{ToBase64, STANDARD};
use super::curl::easy::{List};

pub struct ChatService {
    store: SignalProtocolStore,
    base_url: String,
    signing_key: SecretKey,
    token_id: Address,
    password: String
}

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
                                        "video" => false,
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
}

mod sender;
mod receiver;
