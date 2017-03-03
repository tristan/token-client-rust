extern crate rustc_serialize;

use rustc_serialize::base64::{ToBase64, FromBase64, STANDARD};
use protobuf::{Message as ProtoBufMessage};
use ::keys::{ECPublicKey};
use super::WhisperTextProtocol;

struct SignalMessage {
    version: u32,
    sender_ratchet_key: ECPublicKey,
    counter: u32,
    previous_counter: u32,
    cyphertext: Vec<u8>,
    mac: Vec<u8>
}

const MAC_LENGTH: usize = 8;

enum MessageTypes {
    WHISPER = 2,
    PREKEY,
    SENDERKEY,
    SENDERKEY_DISTRIBUTION,
    ENCRYPTED_MESSAGE_OVERHEAD = 53
}

const UNSUPPORTED_VERSION: u32 = 1;
const CURRENT_VERSION: u32 = 3;

#[inline(always)]
fn high_bits_to_int(value: u8) -> u32 {
    (value & 0xFF) as u32 >> 4
}

impl SignalMessage {
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
            cyphertext: whispermessage.take_ciphertext(),
            mac: mac.to_vec()
        })

    }
}

pub struct PreKeySignalMessage {
    version: u32,
    registration_id: u32,
    pre_key_id: Option<u32>,
    signed_pre_key_id: Option<u32>,
    base_key: ECPublicKey,
    identity_key: ECPublicKey,
    message: SignalMessage
}

impl PreKeySignalMessage {

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
            pre_key_id: if prekeywhispermessage.has_preKeyId() { Some(prekeywhispermessage.get_preKeyId()) } else { None },
            signed_pre_key_id: if prekeywhispermessage.has_signedPreKeyId() { Some(prekeywhispermessage.get_signedPreKeyId()) } else { None },
            base_key: ECPublicKey::deserialize(&prekeywhispermessage.get_baseKey().to_vec()),
            identity_key: ECPublicKey::deserialize(&prekeywhispermessage.get_identityKey().to_vec()),
            message: message
        })
    }

    pub fn get_identity_key(&self) -> &ECPublicKey {
        &self.identity_key
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
    fn test_deserialize_pre_key_message() {
        let enc = "MwgAEiEFQOTVzkCPZs3b3uZPCUxID32du0j1sWFkV2dFophSNhQaIQXhlL+gTObHHts8kvIIUuXQlZjilwmb/IKW3tww+ZhwBCLTATMKIQWuJkY2IQxxzdosVTIcvCqJzJawaN5ElaWQiZkDfXgIERAAGAAioAE+FeNIvG9P2UOOPlT/FIdjS3uuz0kEe1x1hXEgIstZYQmyKn8KJ2Hg1HFXvhfX5Gm6970nqLtoOFuASFObLnCOGJVzjTjk/w13OARLn1DVXOF3QL4oTc2Wf75/oOZuJWGgn9F8SPXnJ2BphVQW2jzkskyaOXiR/zk1ESeVgIbCpuuXiHbjhbGj7QGu06Zky6iMp0YQXB1ipL8qCRocwOUB46R0i5PDTgYovGcwAA==";
        let serialized: Vec<u8> = enc.from_base64().unwrap();

        let exp_serialized: Vec<u8> = vec![0x33, 0x8, 0x0, 0x12, 0x21, 0x5, 0x40, 0xe4, 0xd5, 0xce, 0x40, 0x8f, 0x66, 0xcd, 0xdb, 0xde,
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
}
