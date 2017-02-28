extern crate rustc_serialize;
extern crate time;
extern crate curl;
extern crate tiny_keccak;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use super::eth::{Address,SecretKey};
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
        Method::POST => {
            easy.post(true).unwrap();
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
            match response_code {
                204 => Ok(None),
                200 => {
                    let respobj = json::parse(str::from_utf8(&response_data).unwrap()).unwrap();
                    Ok(Some(respobj))
                },
                _ => {
                    let respobj = json::parse(str::from_utf8(&response_data).unwrap()).unwrap();
                    if respobj["errors"].len() > 0 {
                        Err(format!("{}", respobj["errors"][0]["message"]))
                    } else {
                        Err(format!("Got unexpected response code: {}", response_code))
                    }
                }
            }
        },
        Err(e) => {
            Err(format!("{:?}", e))
        }
    }
}

fn request(method: Method, base_url: &str, path: &str, body: Option<&JsonValue>) -> Result<(Option<JsonValue>), (String)> {
    do_request(method, base_url, path, List::new(), body)
}

fn signed_request(signing_key: &SecretKey, method: Method, base_url: &str, path: &str, body: Option<&JsonValue>) -> Result<(Option<JsonValue>), (String)> {

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
    let mut list = List::new();
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
                                  Some(&object!{
                                      "reviewee" => reviewee,
                                      "score" => rating,
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
