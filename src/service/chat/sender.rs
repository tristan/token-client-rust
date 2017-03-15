use super::ChatService;
use super::{request,signed_request,Method};
use ::eth::{Address,SecretKey};
use ::signal::protocol::{SignalProtocolAddress, SignalProtocolStore, SignalError};
use ::signal::state::PreKeyBundle;
use ::signal::keys::ECPublicKey;
use super::base64::STANDARD_NO_PADDING;
use ::rustc_serialize::base64::{FromBase64};
use json;

macro_rules! get_prekey_path {() => ("/v2/keys/{}")}
macro_rules! get_prekey_device_path {() => ("/v2/keys/{}/{}")}

impl<'a> ChatService<'a> {

    pub fn send_message(&self, to: &Address, message: &String) {

    }

    fn get_encrypted_message(&'a mut self, address: &Address, device_id: u32, plaintext: &Vec<u8>) {

        //self.store

        match self.get_pre_keys(address, device_id) {
            Ok(bundles) => {
                for bundle in bundles.iter() {
                    let remote_addr = SignalProtocolAddress::new(address.to_string(), bundle.get_device_id());
                    //process_prekey_bundle(&mut self.store, &remote_addr, &bundle);
                    match self.store.process_prekey_bundle(&remote_addr, &bundle) {
                        Ok(_) => {},
                        Err(e) => {
                            // TODO: signalerror
                        }
                    }
                }
            },
            Err(msg) => {
                // TODO: string
            }
        };
    }

    fn get_pre_keys(&self, address: &Address, device_id: u32) -> Result<Vec<PreKeyBundle>,String> {

        let device_id = if device_id == 1 {
            "*".to_string()
        } else {
            device_id.to_string()
        };

        let path = format!(get_prekey_device_path!(), address.to_string(), device_id);
        let result = request(
            Method::GET,
            self.base_url,
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

    fn get_pre_key(&self, address: &Address, device_id: u32) -> Result<PreKeyBundle,String> {

        let path = format!(get_prekey_device_path!(), address.to_string(), device_id);
        // TODO: relay
        // e.g. https://github.com/WhisperSystems/libsignal-service-java/blob/master/java/src/main/java/org/whispersystems/signalservice/internal/push/PushServiceSocket.java#L263
        let result = request(
            Method::GET,
            self.base_url,
            &path,
            Some(self.get_base_headers()),
            None);

        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let result = result.ok().unwrap().unwrap();

        if result["devices"].is_null() || result["devices"].len() < 1 {
            return Err("Empty prekey list".to_string());
        }

        let ref device: json::JsonValue = result["devices"][0];
        create_prekey_bundle(&result, &device)
    }
}

fn create_prekey_bundle(result: &json::JsonValue, device: &json::JsonValue) -> Result<PreKeyBundle,String> {

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
