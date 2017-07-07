#![feature(plugin)]
#![plugin(mod_path)]

extern crate protobuf;
extern crate rustc_serialize;
extern crate time;
extern crate curl;
extern crate tiny_keccak;
extern crate eth;
extern crate signal;

#[macro_use]
extern crate json;

use rustc_serialize::base64::{ToBase64, STANDARD};
use eth::{SecretKey};
use curl::easy::{Easy, List};
use json::JsonValue;
use std::io::Read;
use std::{str};
use tiny_keccak::Keccak;

#[allow(dead_code)]
pub enum Method { GET, POST, PUT, DELETE }

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
            match method {
                Method::DELETE => { easy.custom_request("DELETE").unwrap(); },
                _ => {}
            };
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
    let address = signing_key.address().to_string();
    list.append(format!("Token-Timestamp: {}", timespec.sec).as_str()).unwrap();
    list.append(format!("Token-Signature: 0x{:x}", sig).as_str()).unwrap();
    list.append(format!("Token-ID-Address: {}", address).as_str()).unwrap();

    do_request(method, base_url, path, list, body)

}

pub mod chat;
pub mod rep;
pub mod id;
