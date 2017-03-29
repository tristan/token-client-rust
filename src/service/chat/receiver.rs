use super::ChatService;
use super::{request,signed_request,Method};
use ::signal::protocol::{SignalProtocolAddress};
use ::signal::message::{PreKeySignalMessage,SignalMessage,DataMessage,TokenMessage};
use ::rustc_serialize::base64::{FromBase64};
use std::str;
use std::collections::LinkedList;
use std::iter::Iterator;

macro_rules! acknowledge_message_path {() => ("/v1/messages/{}/{}")}

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

impl ChatService {
    pub fn get_messages(&mut self) -> Result<LinkedList<TokenMessage>, String> {
        let response = signed_request(&self.signing_key,
                                      Method::GET,
                                      self.base_url.as_str(),
                                      "/v1/messages",
                                      Some(self.get_base_headers()),
                                      None)?.unwrap();
        //println!("{:#}", response);
        let mut messages: LinkedList<TokenMessage> = LinkedList::new();
        for envelope in response["messages"].members() {
            let timestamp = envelope["timestamp"].as_i64().unwrap();
            let remote_address = SignalProtocolAddress::new(
                envelope["source"].as_str().unwrap(),
                envelope["sourceDevice"].as_u32().unwrap());

            // acknowledge message
            // TODO: might not make sense to do this before processing the message
            self.acknowledge_message(&remote_address.get_address(), timestamp)?;

            let env_type = envelope["type"].as_u32().unwrap();

            if env_type == 5 {
                // message receipt
                continue;
            } else if env_type != 1 && env_type != 3 {
                println!("Unknown Message Type: {}", envelope["type"]);
                continue;
            }

            let ciphertext = if ! envelope["content"].is_null() {
                envelope["content"].as_str().unwrap().from_base64().unwrap()
            } else if ! envelope["message"].is_null() {
                envelope["message"].as_str().unwrap().from_base64().unwrap()
            } else {
                return Err("Received Invalid Message".to_string());
            };

            let plaintext = if env_type == 1 {
                // CIPHERTEXT (a.k.a. SignalMessage)
                if ! self.store.contains_session(&remote_address) {
                    return Err(format!("No session for: {}", remote_address.get_address()));
                }
                let mut session = self.store.load_session(&remote_address);
                let sm = SignalMessage::deserialize(&ciphertext)?;
                let plaintext = match session.decrypt(&sm) {
                    Ok(val) => {
                        strip_message_body_padding(session.get_session_version(), &val)
                    },
                    Err(e) => {
                        return Err(format!("{:?}", e));
                    }
                };

                self.store.store_session(&remote_address, &session);

                plaintext
            } else if env_type == 3 {
                // PREKEY_BUNDLE (a.k.a. PreKeySignalMessage)
                let mut session = self.store.load_session(&remote_address);
                let pksm = PreKeySignalMessage::deserialize(&ciphertext)?;
                let unsigned_prekey_id = match self.store.process_prekey_message(&remote_address, &mut session, &pksm) {
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
                self.store.store_session(&remote_address, &session);
                if unsigned_prekey_id.is_some() {
                    let unsigned_prekey_id = unsigned_prekey_id.unwrap();
                    self.store.remove_prekey(unsigned_prekey_id);
                }

                plaintext
            } else {
                return Err(format!("Reached unexpected branch"));
            };

            let dm = if ! envelope["content"].is_null() {
                return Err(format!("not implemented"));
            } else if ! envelope["message"].is_null() {
                DataMessage::deserialize(&plaintext, timestamp)
            } else {
                return Err(format!("Reached unexpected branch"));
            };

            messages.push_back(TokenMessage::new(&remote_address.get_address(),
                                                 &dm.get_body(),
                                                 timestamp));
        }

        Ok(messages)
    }

    pub fn acknowledge_message(&self, sender: &str, timestamp: i64) -> Result<(), String> {
        match request(Method::DELETE,
                      self.base_url.as_str(),
                      format!(acknowledge_message_path!(), &sender, timestamp).as_str(),
                      Some(self.get_base_headers()),
                      None) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error acknowledging message {}@{}: {:?}", sender, timestamp, e))
        }
    }

}


#[cfg(test)]
mod tests {

}
