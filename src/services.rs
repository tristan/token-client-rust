extern crate rustc_serialize;
extern crate time;
extern crate curl;
extern crate tiny_keccak;

use self::rustc_serialize::base64::{ToBase64, STANDARD};
use super::eth::SecretKey;
use self::curl::easy::{Easy, List};
use json::JsonValue;
use std::io::Read;
use self::tiny_keccak::Keccak;

pub struct ReputationService<'a> {
    base_url: &'static str,
    signing_key: &'a SecretKey
}

#[allow(dead_code)]
enum Method { GET, POST, PUT, DELETE }

fn sign_request(signing_key: &SecretKey, method: Method, base_url: &str, path: &str, body: Option<&JsonValue>) -> Result<(), (String)> {

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

    println!("Singing key: 0x{:x}", signing_key);
    println!("Token-Signature: 0x{:x}", sig);
    println!("Token-Timestamp: {}", timespec.sec);
    println!("Token-ID-Address: 0x{:x}", signing_key.address());
    println!("URL: {}{}", base_url, path);
    println!("data: {}", data);

    let mut easy = Easy::new();
    easy.url(format!("{}{}", base_url, path).as_str()).unwrap();
    // set headers
    let mut list = List::new();
    list.append(format!("Token-Timestamp: {}", timespec.sec).as_str()).unwrap();
    list.append(format!("Token-Signature: 0x{:x}", sig).as_str()).unwrap();
    list.append(format!("Token-ID-Address: 0x{:x}", signing_key.address()).as_str()).unwrap();
    list.append("Content-Type: application/json").unwrap();
    easy.http_headers(list).unwrap();

    let result = match method {
        Method::POST => {
            easy.post(true).unwrap();
            let data = match body {
                Some(data) => data.dump(),
                None => "".to_string()
            };
            println!("data: `{:?}` ({} bytes)", data, data.len() as u64);
            let mut data = data.as_bytes();
            easy.post_field_size(data.len() as u64).unwrap();
            let mut transfer = easy.transfer();
            transfer.read_function(|buf| {
                let bytes_read = data.read(buf).unwrap_or(0);
                println!("bytes read: {}", bytes_read);
                Ok(bytes_read)
            }).unwrap();
            transfer.perform()
        },
        _ => {
            easy.perform()
        }
    };
    match result {
        Ok(_) => {
            let response_code = easy.response_code().unwrap();
            match response_code {
                204 => Ok(()),
                200 => {
                    // TODO: read body
                    Ok(())
                },
                _ => {
                    Err(format!("Got unexpected response code: {}", response_code))
                }
            }
        },
        Err(e) => {
            Err(format!("{:?}", e))
        }
    }
}


impl<'a> ReputationService<'a> {
    pub fn new(base_url: &'static str, signing_key: &'a SecretKey) -> ReputationService<'a> {
        ReputationService {
            base_url: base_url,
            signing_key: signing_key
        }
    }

    pub fn submit_review(&self, reviewee: &str, rating: f32, review: &str) -> Result<(), (String)> {
        sign_request(self.signing_key,
                     Method::POST,
                     self.base_url,
                     "/v1/review/submit",
                     Some(&object!{
                         "reviewee" => reviewee,
                         "score" => rating,
                         "review" => review
                     }))
    }
}
