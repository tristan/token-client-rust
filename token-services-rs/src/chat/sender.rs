use super::ChatService;
use ::{request,signed_request,Method};
use eth::{Address};
use signal::protocol::{SignalProtocolAddress};
use signal::state::PreKeyBundle;
use signal::keys::ECPublicKey;
use signal::message::{MessageType,PreKeySignalMessage,SignalMessage,DataMessage};
use rustc_serialize::base64::{ToBase64,FromBase64,STANDARD};
use json;
use json::JsonValue;
use std::cmp;
use std::collections::LinkedList;

macro_rules! get_prekey_path {() => ("/v2/keys/{}")}
macro_rules! get_prekey_device_path {() => ("/v2/keys/{}/{}")}
macro_rules! message_path {() => ("/v1/messages/{}")}

const DEFAULT_DEVICE_ID: u32 = 1;

struct EncryptedMessageResult(u32, u32, u32, String);

impl ChatService {

    pub fn send_message(&mut self, to: &Address, message: &str) -> Result<bool,String> {
        // get encrypted messages
        let mut messages: LinkedList<EncryptedMessageResult> = LinkedList::new();

        // build data message
        let message = DataMessage::with_body(message);
        let timestamp = message.get_timestamp();
        let message = message.serialize();

        if &self.token_id != to {
            let msg = self.get_encrypted_message(to, DEFAULT_DEVICE_ID, &message)?;
            messages.push_back(msg);
        }

        let sub_device_sessions = {
            self.store.get_sub_device_sessions(&to.to_string())
        };

        for device_id in sub_device_sessions {
            let msg = self.get_encrypted_message(to, device_id, &message)?;
            messages.push_back(msg);
        }

        let path = format!(message_path!(), to.to_string());
        let bundle = object!{
            "destination" => to.to_string(),
            "relay" => JsonValue::Null,
            "timestamp" => timestamp,
            "messages" => JsonValue::Array({
                let mut v = Vec::new();
                for msg in messages {
                    // println!("type: {:?}", msg.0);
                    // println!("devid: {:?}", msg.1);
                    // println!("destregid: {:?}", msg.2);
                    // println!("body: {:?}", msg.3);
                    // println!("timestamp: {:?}", timestamp);
                    v.push(object!{
                        "type" => msg.0,
                        "destinationDeviceId" => msg.1,
                        "destinationRegistrationId" => msg.2,
                        "body" => msg.3,
                        "content" => JsonValue::Null,
                        "legacy" => true
                    })
                }
                v
            })
        };
        let result = signed_request(
            &self.signing_key,
            Method::PUT,
            self.base_url.as_str(),
            &path,
            Some(self.get_base_headers()),
            Some(&bundle));
        match result? {
            Some(rval) => {
                Ok(rval["needsSync"].as_bool().unwrap())
            },
            None => {
                Ok(false)
            }
        }
    }

    fn get_encrypted_message(&mut self, recipient: &Address, device_id: u32, plaintext: &[u8]) -> Result<EncryptedMessageResult,String> {

        let addr = SignalProtocolAddress::new(&recipient.to_string(), device_id);
        if ! self.store.contains_session(&addr) {
            match self.get_prekeys(recipient, device_id) {
                Ok(bundles) => {
                    for bundle in bundles.iter() {
                        let remote_addr = SignalProtocolAddress::new(&recipient.to_string(), bundle.get_device_id());
                        match self.store.process_prekey_bundle(&remote_addr, &bundle) {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(format!("Error processing prekey bundle: {:?}", e));
                            }
                        }
                    }
                },
                Err(msg) => {
                    return Err(msg);
                }
            };
        }

        self.encrypt(&addr, plaintext)
    }

    fn get_prekeys(&self, address: &Address, device_id: u32) -> Result<Vec<PreKeyBundle>,String> {

        let device_id = if device_id == 1 {
            "*".to_string()
        } else {
            device_id.to_string()
        };

        let path = format!(get_prekey_device_path!(), address.to_string(), device_id);
        let result = request(
            Method::GET,
            self.base_url.as_str(),
            &path,
            Some(self.get_base_headers()),
            None);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let result = result.ok().unwrap().unwrap();
        let mut bundles = Vec::new();
        for device in result["devices"].members() {
            match create_prekey_bundle(&result, &device) {
                Ok(bundle) => bundles.push(bundle),
                Err(msg) => return Err(msg)
            };
        }

        Ok(bundles)
    }

    // fn get_prekey(&self, address: &Address, device_id: u32) -> Result<PreKeyBundle,String> {

    //     let path = format!(get_prekey_device_path!(), address.to_string(), device_id);
    //     // TODO: relay
    //     // e.g. https://github.com/WhisperSystems/libsignal-service-java/blob/master/java/src/main/java/org/whispersystems/signalservice/internal/push/PushServiceSocket.java#L263
    //     let result = request(
    //         Method::GET,
    //         self.base_url.as_str(),
    //         &path,
    //         Some(self.get_base_headers()),
    //         None);

    //     if result.is_err() {
    //         return Err(result.err().unwrap());
    //     }

    //     let result = result.ok().unwrap().unwrap();

    //     if result["devices"].is_null() || result["devices"].len() < 1 {
    //         return Err("Empty prekey list".to_string());
    //     }

    //     let ref device: json::JsonValue = result["devices"][0];
    //     create_prekey_bundle(&result, &device)
    // }

    fn encrypt(&mut self, destination: &SignalProtocolAddress, plaintext: &[u8]) -> Result<EncryptedMessageResult,String> {
        let mut session = self.store.load_session(&destination);
        let msg_version = session.get_session_state().get_session_version();
        if msg_version < 2 {
            //  TODO: error "Unknown version"
            return Err(format!("Unknown Version: {}", msg_version));
        }
        // pad message if needed
        let plaintext = if msg_version == 2 {
            plaintext.to_vec()
        } else {
            // get padded msg length
            let padded_len = {
                let len = plaintext.len() + 2;
                (((len / 160) + cmp::min(len % 160, 1)) * 160) - 1
            };

            // pad plaintext
            let mut paddedtext = plaintext.to_vec();
            paddedtext.resize(padded_len, 0);
            paddedtext[plaintext.len()] = 0x80;
            paddedtext
        };
        match session.encrypt(&plaintext) {
            Ok(msg) => {
                // store session changes
                self.store.store_session(&destination, &session);
                // return message content
                let remote_version = session.get_session_state().get_remote_registration_id();
                let (msg_type, serialized) = match msg.get_type() {
                    MessageType::PREKEY => {
                        (3, msg.as_any().downcast_ref::<PreKeySignalMessage>().unwrap().serialize())
                    },
                    MessageType::WHISPER => {
                        (1, msg.as_any().downcast_ref::<SignalMessage>().unwrap().serialize())
                    },
                    // _ => {
                    //     return Err(format!("Bad Type: {:?}", msg.get_type()));
                    // }
                };
                //println!("{:?}", serialized);
                Ok(EncryptedMessageResult(msg_type, destination.get_device_id(), remote_version, serialized.to_base64(STANDARD)))
            },
            Err(e) => {
                Err(format!("Error encrypting: {:?}", e))
            }
        }
    }
}

fn create_prekey_bundle(result: &json::JsonValue, device: &json::JsonValue) -> Result<PreKeyBundle,String> {

    //println!("{:#}", result);

    let (prekey_id, prekey_pubkey) = if device["preKey"].is_null() {
        (None, None)
    } else {
        (Some(device["preKey"]["keyId"].as_u32().unwrap()),
         Some(ECPublicKey::deserialize(
             &device["preKey"]["publicKey"].as_str().unwrap().from_base64().unwrap()
         )))
    };
    if device["signedPreKey"].is_null() {
        return Err("Signed PreKey unexpectedly null".to_string());
    }

    Ok(PreKeyBundle::new(
        device["registrationId"].as_u32().unwrap(),
        device["deviceId"].as_u32().unwrap(),
        // prekey
        prekey_id,
        prekey_pubkey.as_ref(),
        // signed prekey
        device["signedPreKey"]["keyId"].as_u32().unwrap(),
        &ECPublicKey::deserialize(
            &device["signedPreKey"]["publicKey"].as_str().unwrap().from_base64().unwrap()
        ),
        &device["signedPreKey"]["signature"].as_str().unwrap().from_base64().unwrap(),
        // id key
        &ECPublicKey::deserialize(
            &result["identityKey"].as_str().unwrap().from_base64().unwrap()
        )))
}


#[cfg(test)]
mod tests {

    use super::*;
    use signal::tests::test_protocol_store::TestProtocolStore;
    use signal::keys::{IdentityKeyPair};
    use signal::protocol::SignalProtocolStore;
    use eth;

    #[test]
    fn test_get_encrypted_message() {

        let alice_token_id = eth::Address::from_string("0x946377c114849aaa1b08deae139d481df6974519");
        let bob_token_id = eth::Address::from_string("0x6ae8cd08d623d34bddbeea4ea29131b4bc7b11f5");

        //let alice_address = SignalProtocolAddress::new(alice_token_id.to_string(), 1);
        let bob_address = SignalProtocolAddress::new(&bob_token_id.to_string(), 1);

        let alice_identity_keypair = IdentityKeyPair::deserialize(&vec![
            0x0a, 0x21, 0x05, 0x1e, 0x3d, 0xda, 0x2a, 0x49, 0x35, 0x21, 0xa0, 0x60, 0x3a, 0x34, 0x25, 0x99,
            0x3d, 0xe0, 0x8c, 0x88, 0x86, 0x59, 0x6a, 0x16, 0x85, 0xe8, 0xaa, 0xbd, 0xfd, 0x52, 0xea, 0xec,
            0x30, 0x10, 0x31, 0x12, 0x20, 0xe0, 0x2b, 0xe6, 0x09, 0xef, 0xed, 0xf4, 0xf9, 0xbc, 0x20, 0xf0,
            0x00, 0xd0, 0x90, 0xd0, 0xa1, 0x6a, 0x61, 0xed, 0xd6, 0x9b, 0xae, 0x55, 0x28, 0x7a, 0xb0, 0x4a,
            0x30, 0x2e, 0x4f, 0x8c, 0x61]);
        let alice_registration_id = 23234;

        let mut alice_store = new_protocol_store!(TestProtocolStore::new(&alice_identity_keypair, alice_registration_id));

        let prekeybundle_json = json::parse(r#"
            {
                "identityKey": "BS8rSm9EcgaQhAGWqy/CYf+wufQdIspFNB7qRNQ4hlp4",
                "devices": [
                    {
                        "deviceId": 1,
                        "registrationId": 10964,
                        "signedPreKey": {
                            "keyId": 2955020,
                            "publicKey": "BT5LSudYHEXo4HNfrI7G2gT/auYrl45C8U2edpthA+JU",
                            "signature": "EPUyjFuoZ1Q00XLbj8eWbMhFCW0mvn5PNLKSGq8/fVzFYVokyt6Yl9QE5t2pbTGWB0QoHEHm/nMZT3gVmaXvgA"
                        },
                        "preKey": {
                            "keyId": 10338114,
                            "publicKey": "BfC2snlRlImvoJlku9CBbmlv1LWTFZxsY2nOIdptVpYX"
                        }
                    }
                ]
            }
            "#).unwrap();
        let bundle = create_prekey_bundle(&prekeybundle_json, &prekeybundle_json["devices"][0]).unwrap();

        alice_store.process_prekey_bundle(&bob_address, &bundle).unwrap();

        let alice_secret_key = eth::generate_secret_key();
        let mut cs = ChatService::new(&alice_store, "...", &alice_secret_key,
                                      &alice_token_id, &"test");

        let message = DataMessage::with_body_and_timestamp(&"hello", 1490363529623);
        let message = message.serialize();

        let enc = cs.get_encrypted_message(&bob_token_id, 1, &message).unwrap();

        let expected_body = "MyjCtQEIwv72BDCMrrQBEiEFcGoB6nJ8s5f34hi4ow8+N1Y/OBCjM7jQ677yhZqYQSQaIQUePdoqSTUhoGA6NCWZPeCMiIZZahaF6Kq9/VLq7DAQMSLTATMKIQUzL1RSKaGoNadwn0uyKEFrV1kSju7gfL3a3slJbn4sGBAAGAAioAFnjszPBgWNcDcLf8TyLHnhTB0L5m629jPUlDG4WTJfnXPiTSxb/5465QsqA+7R1JshMEcD/urVxdsVJXKLZ2w2gnaKyPvNvKpMRULAhkVFXDejbxPamUE6uNzlCsu96s6TsAPh9IEloAc6bFOZpUEX/cj1ePdBywLigmovk0TqIkV+PbnX3ifNllrgx2idSK5/oyf5Xq3PQfhKfqX9TzMhURMgeXtb4j8=";

        assert_eq!(enc.3, expected_body);

        //
    //     let expected_envelope = vec![
    //         0x01, 0x4a, 0xcf, 0x4f, 0x01, 0x1d, 0xe2, 0x9b, 0x33, 0xea, 0xea, 0xb1, 0xf3, 0x34, 0xe3, 0x12,
    //         0x04, 0x0a, 0xc7, 0x2a, 0x83, 0xf8, 0x4d, 0x23, 0x21, 0x1f, 0x1e, 0x71, 0xb5, 0xd0, 0x75, 0x17,
    //         0x85, 0x46, 0xb5, 0x9b, 0xf7, 0xdc, 0xab, 0x08, 0xec, 0xb4, 0x90, 0x86, 0x3a, 0x67, 0x59, 0xf0,
    //         0x67, 0x92, 0x51, 0x98, 0xcf, 0xe1, 0x81, 0xb5, 0x56, 0x2a, 0x46, 0xef, 0x32, 0x2f, 0xcd, 0x40,
    //         0xc5, 0x20, 0xf5, 0x22, 0x1a, 0x57, 0xb3, 0x81, 0x0d, 0x6b, 0x3d, 0xf4, 0xc8, 0xb6, 0x8d, 0x06,
    //         0xac, 0x59, 0x91, 0x77, 0x5d, 0x80, 0xcf, 0xaf, 0xf8, 0x0c, 0x48, 0xa1, 0x79, 0x05, 0x15, 0x86,
    //         0xb1, 0x58, 0x1e, 0x1b, 0x2a, 0x3c, 0x97, 0x19, 0x6f, 0x29, 0x13, 0x61, 0x18, 0xb5, 0x53, 0xc3,
    //         0xa6, 0xb7, 0xfa, 0xb9, 0x64, 0x17, 0x9a, 0x9a, 0x08, 0xcd, 0x6a, 0x4a, 0xee, 0x61, 0x9f, 0xd3,
    //         0x48, 0x7c, 0x64, 0xd3, 0x81, 0x71, 0x8d, 0xe9, 0xa0, 0xaf, 0xd6, 0x6e, 0x49, 0x29, 0xf7, 0xdd,
    //         0x77, 0x65, 0x3a, 0xe0, 0x64, 0xd3, 0x08, 0xa6, 0x36, 0xdb, 0xf0, 0xd8, 0xfc, 0x45, 0x68, 0xac,
    //         0xbb, 0x99, 0xe3, 0xc1, 0xfd, 0xa6, 0x75, 0x86, 0xa4, 0x7d, 0x46, 0xb1, 0x1c, 0x97, 0x12, 0x40,
    //         0xb2, 0xd6, 0x3f, 0xba, 0xca, 0x8c, 0xea, 0xc3, 0xd8, 0x9e, 0x34, 0xd4, 0xba, 0x61, 0xbc, 0x27,
    //         0x05, 0x38, 0x91, 0x75, 0xbd, 0x8e, 0x4b, 0xa4, 0x47, 0x94, 0xe5, 0xa3, 0x9b, 0xd2, 0x53, 0xe3,
    //         0x76, 0x8b, 0xde, 0x9f, 0x11, 0x64, 0xb8, 0x72, 0x4e, 0x47, 0xc4, 0x0e, 0x6a, 0xda, 0x38, 0x02,
    //         0xaa, 0x83, 0xea, 0xc7, 0xc7, 0x2a, 0xfa, 0xfc, 0xcd, 0xd0, 0x36, 0xff, 0xf7, 0xdb, 0xc4, 0x93,
    //         0xc4, 0x1f, 0x5a, 0x63, 0xa0, 0x26, 0x62, 0xec, 0x95, 0xfd, 0xcd, 0xec, 0x0c, 0x74, 0x34, 0x84,
    //         0x8a, 0x23, 0xc7, 0x4f, 0x29, 0xfb, 0x3f, 0xe0, 0x6d, 0x76, 0x91, 0x6c, 0x9a, 0x7e, 0x69, 0x82,
    //         0x2a, 0x9a, 0x1f, 0x00, 0xbb, 0xb1, 0xff, 0x1f, 0x3f, 0xa7, 0x75, 0x8c, 0x98, 0x35, 0x2b, 0x7c,
    //         0x3e, 0x37, 0xf3, 0xdb, 0xdf, 0x2a, 0xe7, 0xb8, 0x14, 0xf9, 0xf1, 0x10, 0x82, 0xad, 0x8e, 0x38,
    //         0xe0, 0xf0, 0xc7, 0x45, 0x82, 0x9f, 0x42, 0x41, 0x43, 0x99, 0xc0, 0xd0, 0x6b, 0xd4, 0xcf, 0x4e,
    //         0x58, 0x22, 0xdd, 0xc7, 0x77, 0x36, 0x78, 0x5a, 0x81, 0x5f, 0xbd, 0xfa, 0x08, 0xbb, 0x99, 0xd8,
    //         0x61, 0xb1, 0x43, 0xbb, 0xcc, 0x9c, 0x2a, 0x8c, 0xff, 0x9c, 0x80, 0x79, 0xc7, 0xd3, 0x49, 0xee,
    //         0x35, 0x34, 0x8c, 0x66, 0x11, 0x1c, 0xf2, 0x18, 0xbb, 0x9d, 0x72, 0x4d, 0xcf, 0xb0, 0x1a, 0x09,
    //         0x85, 0x44, 0xf5, 0x32, 0x64, 0xf0, 0x35, 0xb3, 0x64, 0x60, 0x39, 0x4f, 0xfd, 0xcf, 0x3d, 0xee,
    //         0x81, 0xdd, 0xc4, 0x74, 0x23, 0x92, 0x21, 0xb5, 0x9c, 0xcf, 0x09];
    }

}
