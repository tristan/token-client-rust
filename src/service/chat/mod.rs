use ::eth::{Address,SecretKey};
use ::signal::protocol::SignalProtocolStore;
use ::signal::keys::{IdentityKeyPair};
use ::signal::state::{PreKeyRecord, SignedPreKeyRecord};
use super::{request,signed_request,Method};
use json::JsonValue;
use super::rustc_serialize::base64::{ToBase64, STANDARD};
use super::curl::easy::{List};

pub struct ChatService<'a> {
    store: &'a mut SignalProtocolStore,
    base_url: &'static str,
    signing_key: &'a SecretKey,
    account: String,
    password: String
}

impl<'a> ChatService<'a> {
    pub fn new(store: &'a mut SignalProtocolStore, base_url: &'static str, signing_key: &'a SecretKey, account: &str, password: &str) -> ChatService<'a> {
        ChatService {
            store: store,
            base_url: base_url,
            signing_key: signing_key,
            account: account.to_string(),
            password: password.to_string()
        }
    }

    fn get_base_headers(&self) -> List {
        let auth = format!("Authorization: Basic {}", format!("{}.1:{}", self.account, self.password).as_bytes().to_base64(STANDARD));
        let mut headers = List::new();
        headers.append(auth.as_str()).unwrap();
        headers.append("X-Signal-Agent: Token-Client-Rust").unwrap();
        headers
    }

    pub fn bootstrap_account(&self,
                             identity_key: &IdentityKeyPair,
                             last_resort_key: &PreKeyRecord,
                             pre_keys: &Vec<PreKeyRecord>,
                             signed_pre_key: &SignedPreKeyRecord,
                             registration_id: u32,
                             signaling_key: &[u8;52]
    ) -> Result<(), (String)> {

        let mut prekeyobjs = Vec::with_capacity(pre_keys.len());
        for record in pre_keys {
            prekeyobjs.push(object!{
                "keyId" => record.get_id(),
                "publicKey" => record.get_public_key().serialize().to_base64(STANDARD)
            });
        }

        let result = signed_request(self.signing_key,
                                    Method::PUT,
                                    self.base_url,
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
                                            "keyId" => signed_pre_key.get_id(),
                                            "publicKey" => signed_pre_key.get_public_key().serialize().to_base64(STANDARD),
                                            "signature" => signed_pre_key.get_signature().to_base64(STANDARD)
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
                        Err(val.to_string())
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn get_messages(&self) -> Result<(JsonValue), (String)> {
        match signed_request(&self.signing_key,
                             Method::GET,
                             self.base_url,
                             "/v1/messages",
                             Some(self.get_base_headers()),
                             None) {
            Ok(val) => Ok(val.unwrap()),
            Err(e) => Err(e)
        }
    }
}

mod base64;
mod sender;
