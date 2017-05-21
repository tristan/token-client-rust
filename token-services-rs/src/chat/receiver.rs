use super::ChatService;
use super::{request,signed_request,Method,process_envelope};
use signal::message::{TokenMessage};
use signal::SignalService;
use rustc_serialize::base64::{FromBase64};
use std::str;
use std::collections::LinkedList;
use protobuf::ProtobufEnum;
use json;

macro_rules! acknowledge_message_path {() => ("/v1/messages/{}/{}")}

fn envelope_from_json(json: &json::JsonValue) -> SignalService::Envelope {
    let mut envelope = SignalService::Envelope::new();
    envelope.set_field_type(SignalService::Envelope_Type::from_i32(json["type"].as_i32().unwrap()).unwrap());
    envelope.set_timestamp(json["timestamp"].as_u64().unwrap());
    envelope.set_source(json["source"].as_str().unwrap().to_string());
    envelope.set_sourceDevice(json["sourceDevice"].as_u32().unwrap());
    envelope.set_relay(json["relay"].as_str().unwrap().to_string());
    if ! json["content"].is_null() {
        envelope.set_content(json["content"].as_str().unwrap().from_base64().unwrap());
    }
    if ! json["message"].is_null() {
        envelope.set_legacyMessage(json["message"].as_str().unwrap().from_base64().unwrap());
    }

    envelope
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

            // build envelope from json data
            match process_envelope(&mut self.store, &envelope_from_json(envelope)) {
                Ok((remote_address, timestamp, msg)) => {
                    match msg {
                        Some(tm) => {
                            messages.push_back(tm);
                        },
                        None => {}
                    };
                    self.acknowledge_message(&remote_address.get_address(), timestamp)?;
                },
                Err(error) => {
                    println!("Error processing envelope: {:?}", error);
                }
            };
            // TODO: what happens if parsing the envelope fails!

            // acknowledge message
            // TODO: might not make sense to do this before processing the message

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
