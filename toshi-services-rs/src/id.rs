use eth::{Address,SecretKey};
use super::{request,signed_request,Method};
use json::JsonValue;

pub struct IdService {
    base_url: String,
    signing_key: SecretKey
}

impl IdService {
    pub fn new(base_url: &str, signing_key: &SecretKey) -> IdService {
        IdService {
            base_url: base_url.to_string(),
            signing_key: signing_key.clone()
        }
    }

    pub fn create_user(&self, username: &str, payment_address: &Address) -> Result<(), (String)> {
        let result = signed_request(&self.signing_key,
                                    Method::POST,
                                    self.base_url.as_str(),
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
                             self.base_url.as_str(),
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
