extern crate rustc_serialize;
extern crate time;
extern crate curl;
extern crate tiny_keccak;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use super::eth::{Address,SecretKey};
use super::keys::{IdentityKeyPair,PreKeyRecord,SignedPreKeyRecord};
use self::curl::easy::{Easy, List};
use json::JsonValue;
use json;
use std::io::Read;
use std::{str};
use self::tiny_keccak::Keccak;

#[allow(dead_code)]
enum Method { GET, POST, PUT, DELETE }

fn do_request(method: Method, base_url: &str, path: &str, mut headers: List, body: Option<&JsonValue>) -> Result<(Option<JsonValue>), (String)> {
    // TODO: optional per request
    headers.append("Content-Type: application/json").unwrap();

    let mut easy = Easy::new();
    easy.url(format!("{}{}", base_url, path).as_str()).unwrap();
    easy.http_headers(headers).unwrap();

    let mut response_data: Vec<u8> = Vec::new();

    let result = match method {
        Method::POST | Method::PUT => {
            match method {
                Method::POST => { easy.post(true).unwrap(); },
                Method::PUT => { easy.put(true).unwrap(); },
                _ => {}
            };
            let data = match body {
                Some(data) => data.dump(),
                None => "".to_string()
            };
            let mut data = data.as_bytes();
            easy.post_field_size(data.len() as u64).unwrap();
            let mut transfer = easy.transfer();
            transfer.read_function(|buf| {
                let bytes_read = data.read(buf).unwrap_or(0);
                Ok(bytes_read)
            }).unwrap();
            transfer.write_function(|outdata| {
                response_data.extend_from_slice(outdata);
                Ok(outdata.len())
            }).unwrap();
            transfer.perform()
        },
        _ => {
            let mut transfer = easy.transfer();
            transfer.write_function(|outdata| {
                response_data.extend_from_slice(outdata);
                Ok(outdata.len())
            }).unwrap();
            transfer.perform()
        }
    };

    match result {
        Ok(_) => {
            let response_code = easy.response_code().unwrap();
            let resp_body = str::from_utf8(&response_data).unwrap();
            let resp_body = if resp_body.len() > 0 {
                match json::parse(resp_body) {
                    Ok(data) => data,
                    Err(_) => JsonValue::String(resp_body.to_string())
                }
            } else {
                JsonValue::Null
            };
            match response_code {
                204 => Ok(None),
                200 => {
                    Ok(Some(resp_body))
                },
                _ => {
                    if resp_body["errors"].len() > 0 {
                        Err(format!("{}", resp_body["errors"][0]["message"]))
                    } else {
                        Err(format!("Got unexpected response code: {}\n{:#}", response_code, resp_body))
                    }
                }
            }
        },
        Err(e) => {
            Err(format!("{:?}", e))
        }
    }
}

fn request(method: Method, base_url: &str, path: &str, headers: Option<List>, body: Option<&JsonValue>) -> Result<(Option<JsonValue>), (String)> {
    do_request(method, base_url, path, match headers {
        Some(list) => list,
        None => List::new()
    }, body)
}

fn signed_request(signing_key: &SecretKey, method: Method, base_url: &str, path: &str, headers: Option<List>, body: Option<&JsonValue>) -> Result<(Option<JsonValue>), (String)> {

    let timespec = time::get_time();
    // timespec.sec;
    let hash = match body {
        Some(data) => {
            let mut hash_output: [u8; 32] = [0; 32];
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&data.dump().as_bytes());
            sha3.finalize(&mut hash_output);
            hash_output[..].to_base64(STANDARD)
        }
        None => "".to_string()
    };
    let data = format!("{}\n{}\n{}\n{}",
                       match method {Method::GET => "GET",
                                     Method::POST => "POST",
                                     Method::PUT => "PUT",
                                     Method::DELETE => "DELETE"},
                       path,
                       timespec.sec,
                       hash);
    let sig = signing_key.sign(data.as_bytes());

    // println!("Singing key: 0x{:x}", signing_key);
    // println!("Token-Signature: 0x{:x}", sig);
    // println!("Token-Timestamp: {}", timespec.sec);
    // println!("Token-ID-Address: 0x{:x}", signing_key.address());
    // println!("URL: {}{}", base_url, path);
    // println!("data: {}", data);


    // set headers
    let mut list = match headers {
        Some(list) => list,
        None => List::new()
    };
    list.append(format!("Token-Timestamp: {}", timespec.sec).as_str()).unwrap();
    list.append(format!("Token-Signature: 0x{:x}", sig).as_str()).unwrap();
    list.append(format!("Token-ID-Address: 0x{:x}", signing_key.address()).as_str()).unwrap();

    do_request(method, base_url, path, list, body)

}

pub struct ReputationService<'a> {
    base_url: &'static str,
    signing_key: &'a SecretKey
}

impl<'a> ReputationService<'a> {
    pub fn new(base_url: &'static str, signing_key: &'a SecretKey) -> ReputationService<'a> {
        ReputationService {
            base_url: base_url,
            signing_key: signing_key
        }
    }

    pub fn submit_review(&self, reviewee: &str, rating: f32, review: &str) -> Result<(), (String)> {
        let result = signed_request(self.signing_key,
                                    Method::POST,
                                    self.base_url,
                                    "/v1/review/submit",
                                    None,
                                    Some(&object!{
                                        "reviewee" => reviewee,
                                        "rating" => rating,
                                        "review" => review
                                    }));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}

pub struct IdService<'a> {
    base_url: &'static str,
    signing_key: &'a SecretKey
}

impl<'a> IdService<'a> {
    pub fn new(base_url: &'static str, signing_key: &'a SecretKey) -> IdService<'a> {
        IdService {
            base_url: base_url,
            signing_key: signing_key
        }
    }

    pub fn create_user(&self, username: &str, payment_address: &Address) -> Result<(), (String)> {
        let result = signed_request(self.signing_key,
                                    Method::POST,
                                    self.base_url,
                                    "/v1/user",
                                    None,
                                    Some(&object!{
                                        "username" => username,
                                        "payment_address" => format!("0x{:x}", payment_address)
                                    }));
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<(JsonValue), (String)> {
        let result = request(Method::GET,
                             self.base_url,
                             format!("/v1/user/{}", username).as_str(),
                             None,
                             None);
        match result {
            Ok(val) => {
                match val {
                    Some(val) => Ok(val),
                    None => Err("unexpected error".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }
}

pub struct ChatService<'a> {
    base_url: &'static str,
    signing_key: &'a SecretKey,
    account: String,
    password: String
}

impl<'a> ChatService<'a> {
    pub fn new(base_url: &'static str, signing_key: &'a SecretKey, account: &str, password: &str) -> ChatService<'a> {
        ChatService {
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
