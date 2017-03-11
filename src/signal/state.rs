extern crate time;

use ::signal::keys::{ECPrivateKey,ECPublicKey,ECKeyPair,IdentityKeyPair};
use ::signal::LocalStorageProtocol;
use ::signal::curve::{curve25519_sign, curve25519_verify};
use protobuf::{Message};

#[derive(Clone)]
pub struct PreKeyBundle {
    registration_id: u32,
    device_id: u32,
    prekey_id: Option<u32>,
    prekey_public: Option<ECPublicKey>,
    signed_prekey_id: u32,
    signed_prekey_public: ECPublicKey,
    signed_prekey_signature: Vec<u8>,
    identity_key: ECPublicKey
}

impl PreKeyBundle {
    pub fn new(registration_id: u32,
               device_id: u32,
               prekey_id: Option<u32>,
               prekey_public: Option<&ECPublicKey>,
               signed_prekey_id: u32,
               signed_prekey_public: &ECPublicKey,
               signed_prekey_signature: &Vec<u8>,
               identity_key: &ECPublicKey)
               -> PreKeyBundle {
        PreKeyBundle {
            registration_id: registration_id,
            device_id: device_id,
            prekey_id: prekey_id,
            prekey_public: match prekey_public {
                Some(key) => Some(key.clone()),
                None => None
            },
            signed_prekey_id: signed_prekey_id,
            signed_prekey_public: signed_prekey_public.clone(),
            signed_prekey_signature: signed_prekey_signature.clone(),
            identity_key: identity_key.clone()
        }
    }

    pub fn get_registration_id(&self) -> u32 {
        self.registration_id
    }

    pub fn get_prekey_id(&self) -> Option<u32> {
        self.prekey_id
    }

    pub fn get_prekey(&self) -> Option<&ECPublicKey> {
        self.prekey_public.as_ref()
    }

    pub fn get_identity_key(&self) -> &ECPublicKey {
        &self.identity_key
    }

    pub fn get_signed_prekey_id(&self) -> u32 {
        self.signed_prekey_id
    }

    pub fn get_signed_prekey(&self) -> &ECPublicKey {
        &self.signed_prekey_public
    }

    pub fn get_signed_prekey_signature(&self) -> &Vec<u8> {
        &self.signed_prekey_signature
    }

    pub fn get_signed_prekey_signature_as_slice(&self) -> [u8;64] {
        // TODO: this seems bad
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&self.signed_prekey_signature);
        arr
    }
}

// Prekeys
#[derive(Copy, Clone)]
pub struct PreKeyRecord {
    id: u32,
    keypair: ECKeyPair
}

impl PreKeyRecord {
    pub fn new(id: u32, keypair: &ECKeyPair) -> PreKeyRecord {
        PreKeyRecord {
            id: id,
            keypair: keypair.clone()
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut structure = LocalStorageProtocol::PreKeyRecordStructure::new();
        structure.set_id(self.id);
        structure.set_publicKey(self.keypair.get_public_key().serialize());
        structure.set_privateKey(self.keypair.get_private_key().serialize());
        structure.write_to_bytes().unwrap()
    }

    pub fn deserialize(serialized: &Vec<u8>) -> PreKeyRecord {
        let mut structure = LocalStorageProtocol::PreKeyRecordStructure::new();
        structure.merge_from_bytes(serialized).unwrap();
        PreKeyRecord {
            id: structure.get_id(),
            keypair: ECKeyPair::new(&structure.take_privateKey(), &structure.take_publicKey())
        }
    }

    pub fn generate(start: u32) -> PreKeyRecord {
        PreKeyRecord::new(start, &ECKeyPair::generate())
    }

    pub fn generate_prekeys(start: u32, count: u32) -> Vec<PreKeyRecord> {
        let mut vec = Vec::with_capacity(count as usize);
        for i in 0..count {
            vec.push(PreKeyRecord::new((start + i) % 0xFFFFFE, &ECKeyPair::generate()));
        }
        vec
    }

    pub fn get_public_key(&self) -> &ECPublicKey {
        &self.keypair.get_public_key()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_key_pair(&self) -> &ECKeyPair {
        &self.keypair
    }
}

// Signed prekeys
#[derive(Copy)]
pub struct SignedPreKeyRecord {
    id: u32,
    timestamp: u64,
    signature: [u8;64],
    keypair: ECKeyPair
}

impl Clone for SignedPreKeyRecord {
    // TODO: read up on clone, this compiles but seems super dangerous
    fn clone(&self) -> SignedPreKeyRecord {
        *self
    }
}

impl SignedPreKeyRecord {
    pub fn generate(id: u32, identity_keypair: &IdentityKeyPair) -> SignedPreKeyRecord {
        // generate public key to sign
        let keypair = ECKeyPair::generate();
        let signature = identity_keypair.sign(&keypair.get_public_key().serialize());
        let timespec = time::get_time();
        let mills: u64 = timespec.sec as u64 + timespec.nsec as u64 / 1000 / 1000;
        SignedPreKeyRecord {
            id: id,
            timestamp: mills,
            signature: signature,
            keypair: keypair
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut structure = LocalStorageProtocol::SignedPreKeyRecordStructure::new();
        structure.set_id(self.id);
        structure.set_publicKey(self.keypair.get_public_key().serialize());
        structure.set_privateKey(self.keypair.get_private_key().serialize());
        structure.set_signature(self.signature.to_vec());
        structure.set_timestamp(self.timestamp);
        structure.write_to_bytes().unwrap()
    }

    pub fn deserialize(serialized: &Vec<u8>) -> SignedPreKeyRecord {
        let mut structure = LocalStorageProtocol::SignedPreKeyRecordStructure::new();
        structure.merge_from_bytes(serialized).unwrap();
        let mut sig = [0u8; 64];
        sig.copy_from_slice(&structure.take_signature());
        SignedPreKeyRecord {
            id: structure.get_id(),
            timestamp: structure.get_timestamp(),
            signature: sig,
            keypair: ECKeyPair::new(&structure.take_privateKey(), &structure.take_publicKey())
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_public_key(&self) -> &ECPublicKey {
        &self.keypair.get_public_key()
    }

    pub fn get_signature(&self) -> &[u8;64] {
        &self.signature
    }

    pub fn get_key_pair(&self) -> &ECKeyPair {
        &self.keypair
    }
}

#[cfg(test)]
fn test() {

    use super::*;

    #[test]
    fn generate_pre_key_records() {
        PreKeyRecord::generate_prekeys(0, 100);
    }

    #[test]
    fn serialise_pre_key_record() {
        let sk: Vec<u8> = vec![0x38, 0x11, 0x80, 0xe1, 0x87, 0x1d, 0x04, 0x4a, 0xde, 0xf9, 0x0c, 0xe8, 0x78, 0x25, 0xe1, 0xc1,
                               0x2b, 0xed, 0xba, 0x80, 0x28, 0xc2, 0x41, 0x02, 0x09, 0xb7, 0x42, 0x8d, 0xdc, 0xf6, 0x03, 0x49];
        let pk: Vec<u8> = vec![0x05, 0x0a, 0x3b, 0xbf, 0xa3, 0x01, 0xaa, 0x1f, 0xdd, 0x2a, 0x83, 0xab, 0x61, 0x26, 0x50, 0xae,
                               0x97, 0x7a, 0xa0, 0x25, 0x84, 0x14, 0xa9, 0x14, 0x21, 0x3d, 0xe4, 0xc4, 0x1e, 0x0d, 0x7b, 0x97,
                               0x40];
        let id = 119;
        let serialisation: Vec<u8> = vec![0x08, 0x77, 0x12, 0x21, 0x05, 0x0a, 0x3b, 0xbf, 0xa3, 0x01, 0xaa, 0x1f, 0xdd, 0x2a, 0x83, 0xab,
                                          0x61, 0x26, 0x50, 0xae, 0x97, 0x7a, 0xa0, 0x25, 0x84, 0x14, 0xa9, 0x14, 0x21, 0x3d, 0xe4, 0xc4,
                                          0x1e, 0x0d, 0x7b, 0x97, 0x40, 0x1a, 0x20, 0x38, 0x11, 0x80, 0xe1, 0x87, 0x1d, 0x04, 0x4a, 0xde,
                                          0xf9, 0x0c, 0xe8, 0x78, 0x25, 0xe1, 0xc1, 0x2b, 0xed, 0xba, 0x80, 0x28, 0xc2, 0x41, 0x02, 0x09,
                                          0xb7, 0x42, 0x8d, 0xdc, 0xf6, 0x03, 0x49];

        let keypair = ECKeyPair::new(&sk, &pk);
        let record = PreKeyRecord::new(id, &keypair);
        assert!(record.serialize() == serialisation);
        let record = PreKeyRecord::deserialize(&serialisation);
        assert!(record.keypair.get_private_key().to_vec() == sk);
        assert!(record.keypair.get_public_key().to_vec()[..] == pk[1..33]);
        assert!(record.id == id);
    }

    #[test]
    fn generate_signed_pre_key_record() {
        SignedPreKeyRecord::generate(0, &IdentityKeyPair::generate());
    }

    #[test]
    fn serialise_signed_pre_key() {
        let id = 119;
        let kp_sk: Vec<u8> = vec![0x00, 0x43, 0x4d, 0xbc, 0xc6, 0xd8, 0x88, 0x8a, 0xab, 0x50, 0x5e, 0x72, 0x59, 0xfc, 0x6c, 0xea,
                                  0xfe, 0x54, 0x0e, 0xff, 0x45, 0x23, 0xb1, 0x92, 0xe8, 0x42, 0xfa, 0x37, 0xd7, 0x37, 0x3e, 0x42];
        let kp_pk: Vec<u8> = vec![0x05, 0x86, 0x1c, 0x6a, 0x85, 0x62, 0xe5, 0x36, 0xa0, 0x61, 0xd4, 0xd9, 0x22, 0x7d, 0x4d, 0xb1,
                                  0x56, 0x96, 0x12, 0x12, 0x11, 0xc0, 0x4e, 0xf5, 0x38, 0x55, 0x90, 0xf0, 0x8d, 0xed, 0xf4, 0xc5,
                                  0x59];
        let sig: [u8;64] = [0x05, 0xa1, 0x21, 0x81, 0xb4, 0xbb, 0x62, 0x22, 0xfb, 0x3e, 0x19, 0x92, 0x52, 0xe4, 0x91, 0x66,
                            0x01, 0x23, 0xef, 0x2e, 0x7f, 0x01, 0xd9, 0xee, 0xa2, 0x9c, 0xef, 0x7d, 0x76, 0xd1, 0xa0, 0xbb,
                            0xa1, 0xb8, 0x0c, 0x4e, 0xa1, 0x30, 0xfb, 0xf3, 0xeb, 0xfd, 0x70, 0xda, 0x5c, 0xbf, 0x9c, 0xfb,
                            0x72, 0x9b, 0xbe, 0xf0, 0xc9, 0x3e, 0xcb, 0x12, 0x7d, 0x15, 0x7f, 0x29, 0x0e, 0x7a, 0x82, 0x88];
        let timestamp = 1488192145375;
        let serialisation: Vec<u8> = vec![0x08, 0x77, 0x12, 0x21, 0x05, 0x86, 0x1c, 0x6a, 0x85, 0x62, 0xe5, 0x36, 0xa0, 0x61, 0xd4, 0xd9,
                                          0x22, 0x7d, 0x4d, 0xb1, 0x56, 0x96, 0x12, 0x12, 0x11, 0xc0, 0x4e, 0xf5, 0x38, 0x55, 0x90, 0xf0,
                                          0x8d, 0xed, 0xf4, 0xc5, 0x59, 0x1a, 0x20, 0x00, 0x43, 0x4d, 0xbc, 0xc6, 0xd8, 0x88, 0x8a, 0xab,
                                          0x50, 0x5e, 0x72, 0x59, 0xfc, 0x6c, 0xea, 0xfe, 0x54, 0x0e, 0xff, 0x45, 0x23, 0xb1, 0x92, 0xe8,
                                          0x42, 0xfa, 0x37, 0xd7, 0x37, 0x3e, 0x42, 0x22, 0x40, 0x05, 0xa1, 0x21, 0x81, 0xb4, 0xbb, 0x62,
                                          0x22, 0xfb, 0x3e, 0x19, 0x92, 0x52, 0xe4, 0x91, 0x66, 0x01, 0x23, 0xef, 0x2e, 0x7f, 0x01, 0xd9,
                                          0xee, 0xa2, 0x9c, 0xef, 0x7d, 0x76, 0xd1, 0xa0, 0xbb, 0xa1, 0xb8, 0x0c, 0x4e, 0xa1, 0x30, 0xfb,
                                          0xf3, 0xeb, 0xfd, 0x70, 0xda, 0x5c, 0xbf, 0x9c, 0xfb, 0x72, 0x9b, 0xbe, 0xf0, 0xc9, 0x3e, 0xcb,
                                          0x12, 0x7d, 0x15, 0x7f, 0x29, 0x0e, 0x7a, 0x82, 0x88, 0x29, 0xdf, 0x07, 0x2a, 0x7f, 0x5a, 0x01,
                                          0x00, 0x00];
        let spkr = SignedPreKeyRecord {
            id: id,
            timestamp: timestamp,
            signature: sig,
            keypair: ECKeyPair::new(&kp_sk, &kp_pk)
        };
        assert!(spkr.serialize() == serialisation);
        let record = SignedPreKeyRecord::deserialize(&serialisation);
        assert!(record.keypair.get_private_key().to_vec() == kp_sk);
        assert!(record.keypair.get_public_key().to_vec()[..] == kp_pk[1..33]);
        assert!(record.id == id);
        assert!(record.timestamp == timestamp);
        assert!(record.signature[0..32] == sig[0..32]);
        assert!(record.signature[32..64] == sig[32..64]);
    }

    #[test]
    fn test_signed_pre_key_valid() {
        let idkp = IdentityKeyPair::generate();
        let spkr = SignedPreKeyRecord::generate(0, &idkp);
        assert!(curve25519_verify(
            &idkp.get_public_key().as_slice(),
            &spkr.get_public_key().serialize(),
            &spkr.signature) == true);
    }
}
