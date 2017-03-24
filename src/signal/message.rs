extern crate rustc_serialize;
extern crate crypto;

use rustc_serialize::base64::{ToBase64, FromBase64, STANDARD};
use protobuf::{Message as ProtobufMessage};
use super::keys::{ECPublicKey};
use super::WhisperTextProtocol;
use self::crypto::mac::Mac;
use self::crypto::hmac::Hmac;
use self::crypto::sha2::Sha256;
use std::any::Any;

use super::SignalService::{DataMessage as SignalDataMessage, DataMessage_Flags, Envelope as SignalEnvelope};
use super::time;

const MAC_LENGTH: usize = 8;

#[derive(Debug,PartialEq)]
pub enum MessageType {
    WHISPER = 2,
    PREKEY,
    // SENDERKEY,
    // SENDERKEY_DISTRIBUTION,
    // ENCRYPTED_MESSAGE_OVERHEAD = 53
}

const UNSUPPORTED_VERSION: u32 = 1;
pub const CURRENT_VERSION: u32 = 3;

#[inline(always)]
fn high_bits_to_int(value: u8) -> u32 {
    (value & 0xFF) as u32 >> 4
}

#[inline(always)]
fn ints_to_byte_high_and_low(high: u32, low: u32) -> u8 {
    ((high << 4 | low) & 0xFF) as u8
}

pub struct SignalMessage {
    version: u32,
    sender_ratchet_key: ECPublicKey,
    counter: u32,
    previous_counter: u32,
    ciphertext: Vec<u8>,
    mac: Vec<u8>
}

pub trait CipherTextMessage {
    fn get_type(&self) -> MessageType;
    fn as_any(&self) -> &Any; // TODO: is this a good idea? (read https://github.com/chris-morgan/mopa)
}

impl CipherTextMessage for SignalMessage {
    fn get_type(&self) -> MessageType {
        MessageType::WHISPER
    }
    fn as_any(&self) -> &Any {
        self
    }
}

impl SignalMessage {

    pub fn new(message_version: u32, mac_key: &Vec<u8>,
               sender_ratchet_key: &ECPublicKey, counter: u32,
               previous_counter: u32, ciphertext: &Vec<u8>,
               sender_identity_key: &ECPublicKey,
               receiver_identity_key: &ECPublicKey) -> SignalMessage {
        // TODO: merge this with serialize fn
        let mut serialized: Vec<u8> = vec![ints_to_byte_high_and_low(
            message_version, CURRENT_VERSION)];
        let mut message = WhisperTextProtocol::SignalMessage::new();
        message.set_ratchetKey(sender_ratchet_key.serialize());
        message.set_counter(counter);
        message.set_previousCounter(previous_counter);
        message.set_ciphertext(ciphertext.clone());
        serialized.extend(message.write_to_bytes().unwrap());
        SignalMessage {
            version: message_version,
            sender_ratchet_key: sender_ratchet_key.clone(),
            counter: counter,
            previous_counter: previous_counter,
            ciphertext: ciphertext.clone(),
            mac: SignalMessage::get_mac(message_version, sender_identity_key,
                                        receiver_identity_key, mac_key,
                                        &serialized)
        }
    }

    pub fn deserialize(serialized: &Vec<u8>) -> Result<SignalMessage, String> {

        let version = high_bits_to_int(serialized[0]);
        let (message, mac) = serialized[1..].split_at(serialized.len() - 1 - MAC_LENGTH);

        if version <= UNSUPPORTED_VERSION {
            return Err(format!("Legacy message: {}", version));
        }

        if version > CURRENT_VERSION {
            return Err(format!("Unknown version: {}", version));
        }

        let mut whispermessage = WhisperTextProtocol::SignalMessage::new();
        whispermessage.merge_from_bytes(message).unwrap();

        if !whispermessage.has_ciphertext() || !whispermessage.has_counter() || !whispermessage.has_ratchetKey() {
            return Err("Incomplete message".to_string());
        }

        Ok(SignalMessage {
            version: version,
            sender_ratchet_key: ECPublicKey::deserialize(&whispermessage.get_ratchetKey().to_vec()),
            counter: whispermessage.get_counter(),
            previous_counter: whispermessage.get_previousCounter(),
            ciphertext: whispermessage.take_ciphertext(),
            mac: mac.to_vec()
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized: Vec<u8> = vec![ints_to_byte_high_and_low(self.version, CURRENT_VERSION)];
        let mut message = WhisperTextProtocol::SignalMessage::new();
        message.set_ratchetKey(self.sender_ratchet_key.serialize());
        message.set_counter(self.counter);
        message.set_previousCounter(self.previous_counter);
        message.set_ciphertext(self.ciphertext.clone());
        serialized.extend(message.write_to_bytes().unwrap());
        serialized.extend(self.mac.clone());
        serialized
    }

    pub fn verify_mac(&self, message_version: u32, sender_identity_key: &ECPublicKey,
                      receiver_identity_key: &ECPublicKey, mac_key: &Vec<u8>) -> bool {
        let serialized = self.serialize();
        let (message, their_mac) = serialized.split_at(serialized.len() - 8);
        let our_mac = SignalMessage::get_mac(
            message_version, sender_identity_key,
            receiver_identity_key, mac_key,
            &message.to_vec());
        our_mac == their_mac
    }

    pub fn get_mac(message_version: u32, sender_identity_key: &ECPublicKey,
                   receiver_identity_key: &ECPublicKey, mac_key: &Vec<u8>,
                   serialized: &Vec<u8>) -> Vec<u8> {
        let mut mac = Hmac::new(Sha256::new(), mac_key);
        if message_version >= 3 {
            mac.input(&sender_identity_key.serialize());
            mac.input(&receiver_identity_key.serialize());
        }
        mac.input(serialized);
        mac.result().code()[..8].to_vec()
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }

    pub fn get_sender_ratchet_key(&self) -> &ECPublicKey {
        &self.sender_ratchet_key
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.ciphertext
    }
}

pub struct PreKeySignalMessage {
    version: u32,
    registration_id: u32,
    prekey_id: Option<u32>,
    signed_prekey_id: Option<u32>,
    base_key: ECPublicKey,
    identity_key: ECPublicKey,
    message: SignalMessage
}

impl CipherTextMessage for PreKeySignalMessage {
    fn get_type(&self) -> MessageType {
        MessageType::PREKEY
    }
    fn as_any(&self) -> &Any {
        self
    }
}

impl PreKeySignalMessage {

    pub fn new(version: u32,
               registration_id: u32,
               prekey_id: Option<u32>,
               signed_prekey_id: Option<u32>, // TODO: this probably doesn't need to be an option
               base_key: &ECPublicKey,
               identity_key: &ECPublicKey,
               message: SignalMessage) -> PreKeySignalMessage {
        PreKeySignalMessage {
            version: version,
            registration_id: registration_id,
            prekey_id: prekey_id,
            signed_prekey_id: signed_prekey_id,
            base_key: base_key.clone(),
            identity_key: identity_key.clone(),
            message: message
        }
    }

    pub fn deserialize(serialized: &Vec<u8>) -> Result<PreKeySignalMessage, String> {

        let version = high_bits_to_int(serialized[0]);

        if version < CURRENT_VERSION {
            return Err(format!("Legacy message: {}", version));
        }

        if version > CURRENT_VERSION {
            return Err(format!("Unknown version: {}", version));
        }

        let mut prekeywhispermessage = WhisperTextProtocol::PreKeySignalMessage::new();
        prekeywhispermessage.merge_from_bytes(&serialized[1..]).unwrap();

        if !prekeywhispermessage.has_signedPreKeyId() || !prekeywhispermessage.has_baseKey() || !prekeywhispermessage.has_identityKey() || !prekeywhispermessage.has_message() {
            return Err("Incomplete message.".to_string());
        }

        let message = match SignalMessage::deserialize(&prekeywhispermessage.get_message().to_vec()) {
            Ok(message) => message,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(PreKeySignalMessage {
            version: version,
            registration_id: prekeywhispermessage.get_registrationId(),
            prekey_id: if prekeywhispermessage.has_preKeyId() { Some(prekeywhispermessage.get_preKeyId()) } else { None },
            signed_prekey_id: if prekeywhispermessage.has_signedPreKeyId() {
                Some(prekeywhispermessage.get_signedPreKeyId())
            } else {
                None
            },
            base_key: ECPublicKey::deserialize(&prekeywhispermessage.get_baseKey().to_vec()),
            identity_key: ECPublicKey::deserialize(&prekeywhispermessage.get_identityKey().to_vec()),
            message: message
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized: Vec<u8> = vec![ints_to_byte_high_and_low(self.version, CURRENT_VERSION)];
        let mut message = WhisperTextProtocol::PreKeySignalMessage::new();
        match self.signed_prekey_id {
            Some(id) => message.set_signedPreKeyId(id),
            None => panic!("shouldn't happen") // TODO
        };
        message.set_baseKey(self.base_key.serialize());
        message.set_identityKey(self.identity_key.serialize());
        message.set_message(self.message.serialize());
        message.set_registrationId(self.registration_id);
        match self.prekey_id {
            Some(id) => message.set_preKeyId(id),
            None => {}
        };
        serialized.extend(message.write_to_bytes().unwrap());
        serialized
    }

    pub fn get_identity_key(&self) -> &ECPublicKey {
        &self.identity_key
    }

    pub fn get_registration_id(&self) -> u32 {
        self.registration_id
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_base_key(&self) -> &ECPublicKey {
        &self.base_key
    }

    pub fn get_signed_prekey_id(&self) -> u32 {
        match self.signed_prekey_id {
            Some(val) => val,
            _ => panic!("Unexpectedly called get_signed_prekey_id before key id was set")
        }
    }

    pub fn get_prekey_id(&self) -> Option<u32> {
        self.prekey_id
    }

    pub fn get_whisper_message(&self) -> &SignalMessage {
        &self.message
    }
}

// -------------------------------------------------------------------
// SERVICE MESSAGES
// -------------------------------------------------------------------

pub struct DataMessage {
    timestamp: i64,
    body: String,
    end_session: bool,
    expires_in_seconds: u32,
    expiration_update: bool
    // TODO: attachments, group
}

impl DataMessage {
    pub fn with_body(body: &str) -> DataMessage {
        let timestamp = {
            let timespec = time::get_time();
            (timespec.sec as i64 * 1000) + (timespec.nsec as i64 / 1000 / 1000)
        };
        Self::with_body_and_timestamp(body, timestamp)
    }

    pub fn with_body_and_timestamp(body: &str, timestamp: i64) -> DataMessage {
        DataMessage {
            timestamp: timestamp,
            body: body.to_string(),
            end_session: false,
            expiration_update: false,
            expires_in_seconds: 0
        }
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn serialize(&self) -> Vec<u8> {
        // TODO attachments & groupinfo
        // see: https://github.com/WhisperSystems/libsignal-service-java/blob/master/java/src/main/java/org/whispersystems/signalservice/api/SignalServiceMessageSender.java#L208

        let mut dm = SignalDataMessage::new();
        dm.set_body(self.body.clone());
        if self.end_session {
            dm.set_flags(DataMessage_Flags::END_SESSION as u32);
        }
        if self.expiration_update {
            dm.set_flags(DataMessage_Flags::EXPIRATION_TIMER_UPDATE as u32);
        }
        if self.expires_in_seconds > 0 {
            dm.set_expireTimer(self.expires_in_seconds);
        }
        dm.write_to_bytes().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_high_bits_to_int() {
        assert_eq!(high_bits_to_int(255), 15);
        assert_eq!(high_bits_to_int(200), 12);
        assert_eq!(high_bits_to_int(100), 6);
        assert_eq!(high_bits_to_int(50), 3);
        assert_eq!(high_bits_to_int(3), 0);
        assert_eq!(high_bits_to_int(1), 0);
    }

    #[test]
    fn test_deserialize_prekey_message() {
        let enc = "MwgAEiEFQOTVzkCPZs3b3uZPCUxID32du0j1sWFkV2dFophSNhQaIQXhlL+gTObHHts8kvIIUuXQlZjilwmb/IKW3tww+ZhwBCLTATMKIQWuJkY2IQxxzdosVTIcvCqJzJawaN5ElaWQiZkDfXgIERAAGAAioAE+FeNIvG9P2UOOPlT/FIdjS3uuz0kEe1x1hXEgIstZYQmyKn8KJ2Hg1HFXvhfX5Gm6970nqLtoOFuASFObLnCOGJVzjTjk/w13OARLn1DVXOF3QL4oTc2Wf75/oOZuJWGgn9F8SPXnJ2BphVQW2jzkskyaOXiR/zk1ESeVgIbCpuuXiHbjhbGj7QGu06Zky6iMp0YQXB1ipL8qCRocwOUB46R0i5PDTgYovGcwAA==";
        let serialized: Vec<u8> = enc.from_base64().unwrap();

        let exp_serialized: Vec<u8> = vec![
            0x33, 0x8, 0x0, 0x12, 0x21, 0x5, 0x40, 0xe4, 0xd5, 0xce, 0x40, 0x8f, 0x66, 0xcd, 0xdb, 0xde,
            0xe6, 0x4f, 0x9, 0x4c, 0x48, 0xf, 0x7d, 0x9d, 0xbb, 0x48, 0xf5, 0xb1, 0x61, 0x64, 0x57, 0x67,
            0x45, 0xa2, 0x98, 0x52, 0x36, 0x14, 0x1a, 0x21, 0x5, 0xe1, 0x94, 0xbf, 0xa0, 0x4c, 0xe6, 0xc7,
            0x1e, 0xdb, 0x3c, 0x92, 0xf2, 0x8, 0x52, 0xe5, 0xd0, 0x95, 0x98, 0xe2, 0x97, 0x9, 0x9b, 0xfc,
            0x82, 0x96, 0xde, 0xdc, 0x30, 0xf9, 0x98, 0x70, 0x4, 0x22, 0xd3, 0x1, 0x33, 0xa, 0x21, 0x5,
            0xae, 0x26, 0x46, 0x36, 0x21, 0xc, 0x71, 0xcd, 0xda, 0x2c, 0x55, 0x32, 0x1c, 0xbc, 0x2a, 0x89,
            0xcc, 0x96, 0xb0, 0x68, 0xde, 0x44, 0x95, 0xa5, 0x90, 0x89, 0x99, 0x3, 0x7d, 0x78, 0x8, 0x11,
            0x10, 0x0, 0x18, 0x0, 0x22, 0xa0, 0x1, 0x3e, 0x15, 0xe3, 0x48, 0xbc, 0x6f, 0x4f, 0xd9, 0x43,
            0x8e, 0x3e, 0x54, 0xff, 0x14, 0x87, 0x63, 0x4b, 0x7b, 0xae, 0xcf, 0x49, 0x4, 0x7b, 0x5c, 0x75,
            0x85, 0x71, 0x20, 0x22, 0xcb, 0x59, 0x61, 0x9, 0xb2, 0x2a, 0x7f, 0xa, 0x27, 0x61, 0xe0, 0xd4,
            0x71, 0x57, 0xbe, 0x17, 0xd7, 0xe4, 0x69, 0xba, 0xf7, 0xbd, 0x27, 0xa8, 0xbb, 0x68, 0x38, 0x5b,
            0x80, 0x48, 0x53, 0x9b, 0x2e, 0x70, 0x8e, 0x18, 0x95, 0x73, 0x8d, 0x38, 0xe4, 0xff, 0xd, 0x77,
            0x38, 0x4, 0x4b, 0x9f, 0x50, 0xd5, 0x5c, 0xe1, 0x77, 0x40, 0xbe, 0x28, 0x4d, 0xcd, 0x96, 0x7f,
            0xbe, 0x7f, 0xa0, 0xe6, 0x6e, 0x25, 0x61, 0xa0, 0x9f, 0xd1, 0x7c, 0x48, 0xf5, 0xe7, 0x27, 0x60,
            0x69, 0x85, 0x54, 0x16, 0xda, 0x3c, 0xe4, 0xb2, 0x4c, 0x9a, 0x39, 0x78, 0x91, 0xff, 0x39, 0x35,
            0x11, 0x27, 0x95, 0x80, 0x86, 0xc2, 0xa6, 0xeb, 0x97, 0x88, 0x76, 0xe3, 0x85, 0xb1, 0xa3, 0xed,
            0x1, 0xae, 0xd3, 0xa6, 0x64, 0xcb, 0xa8, 0x8c, 0xa7, 0x46, 0x10, 0x5c, 0x1d, 0x62, 0xa4, 0xbf,
            0x2a, 0x9, 0x1a, 0x1c, 0xc0, 0xe5, 0x1, 0xe3, 0xa4, 0x74, 0x8b, 0x93, 0xc3, 0x4e, 0x6, 0x28,
            0xbc, 0x67, 0x30, 0x0];

        assert_eq!(serialized, exp_serialized);

        let msg = PreKeySignalMessage::deserialize(&serialized).unwrap();

        //msg.process();

        let exp = "hello";
    }

    // #[test]
    // fn test_envelope() {
    //     let pb: Vec<u8> = vec![
    //         0x01, 0xd8, 0xc8, 0x77, 0xd5, 0xf9, 0x69, 0xe2, 0x05, 0x3d, 0x76, 0x80, 0x9c, 0xd6, 0x24, 0xb2,
    //         0x25, 0xf5, 0x64, 0x7d, 0xb6, 0x49, 0xb5, 0x3f, 0xda, 0x35, 0x13, 0x6d, 0x0e, 0xc7, 0x7e, 0xda,
    //         0xf5, 0xfc, 0x26, 0xe9, 0x69, 0x87, 0x5c, 0x16, 0x79, 0x19, 0x3c, 0xbe, 0xd2, 0x73, 0xe8, 0xf7,
    //         0xcb, 0x99, 0xd9, 0x13, 0x64, 0x7b, 0xc5, 0x7e, 0xcd, 0xfb, 0x64, 0x7e, 0xe9, 0x12, 0x8f, 0x25,
    //         0xfd, 0xfe, 0x1e, 0x89, 0x66, 0xb6, 0xa9, 0x6c, 0x69, 0x84, 0x7f, 0x80, 0x80, 0x75, 0x87, 0xdb,
    //         0x3c, 0x28, 0xa8, 0x3c, 0xa8, 0x3d, 0xb6, 0x63, 0xc8, 0xee, 0xf5, 0x97, 0x32, 0xe4, 0x34, 0xe8,
    //         0x67, 0x02, 0x00, 0x69, 0x70, 0x6f, 0x20, 0x1e, 0x69, 0x92, 0x56, 0x9e, 0x1d, 0x49, 0x32, 0x21,
    //         0xfe, 0xf6, 0xaf, 0x43, 0xc2, 0xfb, 0xbc, 0xa4, 0x6c, 0xb2, 0x7c, 0x92, 0x79, 0x15, 0x9f, 0xfe,
    //         0xdd, 0x1d, 0xdf, 0x9b, 0xa5, 0x63, 0x77, 0x43, 0x92, 0xbb, 0xb8, 0x64, 0x4a, 0xe1, 0x16, 0x1a,
    //         0xa6, 0xa8, 0xb1, 0x72, 0xee, 0x17, 0x77, 0xee, 0xfd, 0xd3, 0x98, 0x04, 0xcb, 0xdd, 0x1c, 0x00,
    //         0xb1, 0xf0, 0x09, 0x72, 0x4a, 0xab, 0xaa, 0xf3, 0x36, 0x6f, 0xe8, 0x92, 0x8a, 0x39, 0x45, 0x94,
    //         0xa7, 0xb6, 0x25, 0xf5, 0xec, 0xa1, 0x5a, 0x96, 0x42, 0xb0, 0xd4, 0x11, 0x57, 0x47, 0x20, 0x23,
    //         0x5f, 0x06, 0xae, 0xa6, 0xbb, 0x14, 0xb1, 0x79, 0xa9, 0x29, 0xdd, 0xb9, 0x35, 0x3b, 0x0f, 0xb5,
    //         0x6a, 0x96, 0x13, 0x1b, 0x80, 0x54, 0x39, 0x3e, 0x87, 0xc4, 0x6a, 0xfb, 0x02, 0xc3, 0x8b, 0x4d,
    //         0xb4, 0x68, 0xdc, 0x4e, 0xe7, 0x28, 0x22, 0x38, 0x45, 0xeb, 0x03, 0xd0, 0xd1, 0x45, 0x7f, 0x76,
    //         0xcb, 0xec, 0x3b, 0xec, 0x85, 0x52, 0x85, 0x23, 0x67, 0x08, 0xc9, 0x6f, 0xdf, 0xcc, 0xb8, 0x81,
    //         0xdb, 0x70, 0x7d, 0x36, 0xd5, 0x80, 0x93, 0x14, 0xd0, 0x8a, 0x38, 0xdb, 0x05, 0x7d, 0x04, 0x28,
    //         0x77, 0xf7, 0x56, 0x1e, 0x32, 0xd5, 0x3f, 0x2c, 0xc6, 0x9f, 0xa7, 0x3a, 0x36, 0x55, 0x79, 0x78,
    //         0x2b, 0x74, 0x05, 0xaa, 0xef, 0x09, 0x9d, 0x5a, 0xa1, 0x32, 0xa0, 0xde, 0x41, 0x5e, 0x32, 0x61,
    //         0xef, 0x24, 0x27, 0xcd, 0x91, 0x1b, 0xc4, 0x71, 0xa4, 0x1b, 0x2b, 0xdd, 0xc7, 0x45, 0xa1, 0xd9,
    //         0x75, 0xcb, 0x9a, 0x9c, 0xf8, 0xd4, 0x24, 0x51, 0x64, 0x50, 0x78, 0x09, 0x8f, 0x0d, 0xbe, 0x7b,
    //         0xe9, 0xf3, 0x84, 0x10, 0xd9, 0xd8, 0xbd, 0x6c, 0xee, 0x5f, 0x15, 0x05, 0x15, 0xd0, 0xc7, 0xaa,
    //         0x46, 0xc7, 0x4d, 0x63, 0xf7, 0xdc, 0x53, 0x45, 0x1e, 0xda, 0x92, 0x49, 0x3e, 0xc4, 0xce, 0xd3,
    //         0xf1, 0x89, 0x37, 0xe5, 0x44, 0x6f, 0x71, 0x99, 0xda, 0x41, 0xe6, 0xd0, 0x64, 0x1e, 0x23, 0xf4,
    //         0x0b, 0xe1, 0xb9, 0x1e, 0x57, 0xfb, 0xac, 0xf1, 0x66, 0xc0, 0x53];

    //     let mut envelope = SignalEnvelope::new();
    //     envelope.merge_from_bytes(&pb).unwrap();
    // }
}
