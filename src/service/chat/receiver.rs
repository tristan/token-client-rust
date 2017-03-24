use super::ChatService;
use super::{request,signed_request,Method};
use ::eth::{Address,SecretKey};
use ::signal::protocol::{SignalProtocolAddress, SignalProtocolStore, SignalError};
use ::signal::state::PreKeyBundle;
use ::signal::keys::ECPublicKey;
use ::signal::message::{MessageType,PreKeySignalMessage,SignalMessage};
use super::base64::STANDARD_NO_PADDING;
use ::rustc_serialize::base64::{ToBase64,FromBase64};
use json;
use super::super::time;
use json::JsonValue;
use std::cmp;
use std::collections::LinkedList;

macro_rules! acknowledge_message_path {() => ("/v1/messages/{}/{}")}

impl ChatService {
    pub fn get_messages(&self) -> Result<JsonValue, String> {
        match signed_request(&self.signing_key,
                             Method::GET,
                             self.base_url.as_str(),
                             "/v1/messages",
                             Some(self.get_base_headers()),
                             None) {
            Ok(val) => Ok(val.unwrap()),
            Err(e) => Err(e)
        }
    }

    pub fn acknowledge_message(&self, sender: &str, timestamp: i64) -> Result<(), String> {
        match signed_request(&self.signing_key,
                             Method::DELETE,
                             self.base_url.as_str(),
                             format!(acknowledge_message_path!(), &sender, timestamp).as_str(),
                             None,
                             None) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

}


#[cfg(test)]
mod tests {

}
