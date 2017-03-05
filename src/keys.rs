extern crate rand;
extern crate crypto;
extern crate time;
extern crate protobuf;

use signal::LocalStorageProtocol;
use curve::{curve25519_sign, curve25519_verify};
use protobuf::{Message};

use std;
use self::crypto::curve25519::{curve25519_base,curve25519};
use self::rand::{OsRng, Rng};

// Identity keys
#[derive(Copy, Clone, PartialEq)]
pub struct ECPrivateKey([u8;32]);
#[derive(Copy, Clone, PartialEq)]
pub struct ECPublicKey([u8;32]);

macro_rules! impl_lower_hex_fmt {
    ($x:ty) => {
        impl std::fmt::LowerHex for $x {
            fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                for byte in self.0.to_vec() {
                    try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_to_vec {
    ($x:ty) => {
        impl $x {
            pub fn to_vec(&self) -> std::vec::Vec<u8> {
                self.0.to_vec()
            }
        }
    };
}

impl_lower_hex_fmt!(ECPrivateKey);
impl_lower_hex_fmt!(ECPublicKey);
impl_to_vec!(ECPrivateKey);
impl_to_vec!(ECPublicKey);

impl ECPublicKey {
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized: Vec<u8> = Vec::with_capacity(33);
        serialized.push(0x05);
        serialized.append(&mut self.0.to_vec());
        serialized
    }
    pub fn deserialize(vec: &Vec<u8>) -> ECPublicKey {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(&vec[1..33]);
        ECPublicKey(pk)
    }
}

impl ECPrivateKey {
    pub fn serialize(&self) -> Vec<u8> {
        self.0.to_vec()
    }
    pub fn deserialize(vec: &Vec<u8>) -> ECPrivateKey {
        let mut sk = [0u8; 32];
        sk.copy_from_slice(&vec);
        ECPrivateKey(sk)
    }

    pub fn calculate_agreement(&self, public_key: &ECPublicKey) -> Vec<u8> {
        curve25519(&self.0, &public_key.0).to_vec()
    }
}

macro_rules! create_keypair_type {
    ($x:ident) => {
        #[derive(Copy, Clone)]
        pub struct $x {
            private_key: ECPrivateKey,
            public_key: ECPublicKey
        }

        impl $x {
            pub fn generate() -> $x {
                let mut rng = OsRng::new().ok().unwrap();

                let mut private_key = [0u8; 32];
                rng.fill_bytes(&mut private_key);
                private_key[0] &= 248;
                private_key[31] &= 127;
                private_key[31] |= 64;
                let public_key = curve25519_base(&private_key);
                $x {
                    private_key: ECPrivateKey(private_key),
                    public_key: ECPublicKey(public_key)
                }
            }

            pub fn create(private_key: ECPrivateKey, public_key: ECPublicKey) -> $x {
                $x {
                    private_key: private_key,
                    public_key: public_key
                }
            }

            pub fn get_public_key(&self) -> &ECPublicKey {
                &self.public_key
            }
            pub fn get_private_key(&self) -> &ECPrivateKey {
                &self.private_key
            }

            // curve25519 signature helpers
            pub fn curve25519_sign(&self, message: &[u8]) -> [u8; 64] {
                curve25519_sign(&self.private_key.0, &message)
            }
        }
    }
}

create_keypair_type!(ECKeyPair);
create_keypair_type!(IdentityKeyPair);

impl IdentityKeyPair {
    pub fn serialize(&self) -> Vec<u8> {
        let mut structure = LocalStorageProtocol::IdentityKeyPairStructure::new();
        structure.set_publicKey(self.public_key.serialize());
        structure.set_privateKey(self.private_key.serialize());
        let serialized = structure.write_to_bytes().unwrap();
        serialized
    }
    pub fn deserialize(serialized: &Vec<u8>) -> IdentityKeyPair {
        let mut structure = LocalStorageProtocol::IdentityKeyPairStructure::new();
        structure.merge_from_bytes(serialized).unwrap();
        IdentityKeyPair {
            public_key: ECPublicKey::deserialize(&structure.take_publicKey()),
            private_key: ECPrivateKey::deserialize(&structure.take_privateKey())
        }
    }
}
// Prekeys
#[derive(Copy, Clone)]
pub struct PreKeyRecord {
    id: u32,
    keypair: ECKeyPair
}

impl PreKeyRecord {
    fn from_keypair(id: u32, keypair: &ECKeyPair) -> PreKeyRecord {
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
            keypair: ECKeyPair {
                public_key: ECPublicKey::deserialize(&structure.take_publicKey()),
                private_key: ECPrivateKey::deserialize(&structure.take_privateKey())
            }
        }
    }

    pub fn generate(start: u32) -> PreKeyRecord {
        PreKeyRecord::from_keypair(start, &ECKeyPair::generate())
    }

    pub fn generate_prekeys(start: u32, count: u32) -> Vec<PreKeyRecord> {
        let mut vec = Vec::with_capacity(count as usize);
        for i in 0..count {
            vec.push(PreKeyRecord::from_keypair((start + i) % 0xFFFFFE, &ECKeyPair::generate()));
        }
        vec
    }

    pub fn get_public_key(&self) -> &ECPublicKey {
        &self.keypair.public_key
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
    pub fn generate(identity_keypair: &IdentityKeyPair, id: u32) -> SignedPreKeyRecord {
        // generate public key to sign
        let keypair = ECKeyPair::generate();
        let signature = identity_keypair.curve25519_sign(&keypair.public_key.serialize());
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
            keypair: ECKeyPair {
                public_key: ECPublicKey::deserialize(&structure.take_publicKey()),
                private_key: ECPrivateKey::deserialize(&structure.take_privateKey())
            }
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_public_key(&self) -> &ECPublicKey {
        &self.keypair.public_key
    }

    pub fn get_signature(&self) -> &[u8;64] {
        &self.signature
    }

    pub fn get_key_pair(&self) -> &ECKeyPair {
        &self.keypair
    }
}

#[cfg(test)]
mod tests {

    extern crate crypto;
    use super::*;

    #[test]
    fn test_curve_lib_generates_valid_keys() {
        // verifies that the curve25519 library generates the expected
        // public key from a given private key

        use self::crypto::curve25519::curve25519_base;

        let private_key: [u8;32] = [
            0xe8, 0x41, 0x39, 0x1e, 0xb0, 0x4b, 0x25, 0xfa, 0x8f, 0x9f,
            0xcb, 0x05, 0x5f, 0x05, 0x98, 0xa7, 0x4b, 0xbf, 0xf7, 0xf5,
            0x7f, 0x14, 0x62, 0x12, 0x1d, 0xc8, 0xc0, 0xa4, 0x2d, 0x28,
            0x12, 0x5e];
        let expecetd_public_key: [u8; 32] = [ // 0x05,
            0x18, 0x52, 0x23, 0x6e, 0xd5, 0xf2, 0x49, 0x4f, 0xdc, 0xbe,
            0x0e, 0x78, 0x23, 0xe9, 0x3d, 0x16, 0xf5, 0xa7, 0xc7, 0x42,
            0x7f, 0xb4, 0x51, 0xc0, 0x33, 0xc1, 0x2b, 0xd6, 0x45, 0x8c,
            0x10, 0x69];

        let public_key: [u8;32] = curve25519_base(&private_key);
        assert!(public_key == expecetd_public_key);
        //println!("0x{:x}", ECPublicKey(public_key));
    }

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

        let keypair = ECKeyPair { private_key: ECPrivateKey::deserialize(&sk),
                                  public_key: ECPublicKey::deserialize(&pk) };
        let record = PreKeyRecord::from_keypair(id, &keypair);
        assert!(record.serialize() == serialisation);
        let record = PreKeyRecord::deserialize(&serialisation);
        assert!(record.keypair.private_key.to_vec() == sk);
        assert!(record.keypair.public_key.to_vec()[..] == pk[1..33]);
        assert!(record.id == id);
    }

    #[test]
    fn generate_signed_pre_key_record() {
        SignedPreKeyRecord::generate(&IdentityKeyPair::generate(), 0);
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
            keypair: ECKeyPair { private_key: ECPrivateKey::deserialize(&kp_sk),
                                 public_key: ECPublicKey::deserialize(&kp_pk) }
        };
        assert!(spkr.serialize() == serialisation);
        let record = SignedPreKeyRecord::deserialize(&serialisation);
        assert!(record.keypair.private_key.to_vec() == kp_sk);
        assert!(record.keypair.public_key.to_vec()[..] == kp_pk[1..33]);
        assert!(record.id == id);
        assert!(record.timestamp == timestamp);
        assert!(record.signature[0..32] == sig[0..32]);
        assert!(record.signature[32..64] == sig[32..64]);
    }

    #[test]
    fn test_signed_pre_key_valid() {
        let idkp = IdentityKeyPair::generate();
        let spkr = SignedPreKeyRecord::generate(&idkp, 0);
        assert!(curve25519_verify(
            &idkp.public_key.0,
            &spkr.get_public_key().serialize(),
            &spkr.signature) == true);
    }

    #[test]
    fn generate_identity_key_pair() {
        IdentityKeyPair::generate();
    }

    #[test]
    fn serialize_identity_key_pair() {
        let keypair = IdentityKeyPair {
            public_key: ECPublicKey([
                0x96, 0x3d, 0x4a, 0xca, 0xe3, 0xbd, 0xb8, 0xda, 0x74, 0x77, 0xc7, 0xd5, 0x62, 0xdb, 0x71, 0x6c,
                0x51, 0x3a, 0xba, 0x2c, 0x0c, 0x86, 0xfe, 0x93, 0x88, 0x11, 0x45, 0xd3, 0x4a, 0x24, 0xfa, 0x5c]),
            private_key: ECPrivateKey([
                0x98, 0xa0, 0x57, 0xed, 0x63, 0xe6, 0xdc, 0x62, 0xf4, 0x6c, 0x74, 0x3f, 0x02, 0x4d, 0x4d, 0x44,
                0xda, 0x66, 0x91, 0x8f, 0xf7, 0x7e, 0xb6, 0x0f, 0x2a, 0x09, 0x58, 0x9e, 0x28, 0x09, 0xea, 0x52])
        };
        let expected_serialisation = vec![
            0x0a, 0x21, 0x05, 0x96, 0x3d, 0x4a, 0xca, 0xe3, 0xbd, 0xb8, 0xda, 0x74, 0x77, 0xc7, 0xd5, 0x62,
            0xdb, 0x71, 0x6c, 0x51, 0x3a, 0xba, 0x2c, 0x0c, 0x86, 0xfe, 0x93, 0x88, 0x11, 0x45, 0xd3, 0x4a,
            0x24, 0xfa, 0x5c, 0x12, 0x20, 0x98, 0xa0, 0x57, 0xed, 0x63, 0xe6, 0xdc, 0x62, 0xf4, 0x6c, 0x74,
            0x3f, 0x02, 0x4d, 0x4d, 0x44, 0xda, 0x66, 0x91, 0x8f, 0xf7, 0x7e, 0xb6, 0x0f, 0x2a, 0x09, 0x58,
            0x9e, 0x28, 0x09, 0xea, 0x52];
        assert!(keypair.serialize() == expected_serialisation);

        // test deserialization
        let keypair2 = IdentityKeyPair::deserialize(&expected_serialisation);
        assert!(keypair.get_public_key().to_vec() == keypair2.get_public_key().to_vec());
        assert!(keypair.get_private_key().to_vec() == keypair2.get_private_key().to_vec());
    }

    #[test]
    fn test_calculate_agreement() {

        macro_rules! run_test_case {
            ($sk:expr, $pk:expr, $out:expr) => {
                let pubkey = ECPublicKey($pk);
                let privkey = ECPrivateKey($sk);
                let out = privkey.calculate_agreement(&pubkey);
                assert_eq!(out, $out);
            };
        }

        run_test_case!([0x98, 0xff, 0x77, 0x3b, 0x61, 0x24, 0xca, 0x05, 0x50, 0x9a, 0x4d, 0xfb, 0x25, 0x40, 0x8f, 0xc8,
                        0xf5, 0xbb, 0x27, 0x47, 0x3f, 0x0b, 0x85, 0xdd, 0xa0, 0x2a, 0x18, 0xc5, 0xbd, 0x32, 0xf2, 0x6b],
                       [0x44, 0x5a, 0xef, 0x96, 0x56, 0xa4, 0x3c, 0x28, 0x13, 0xf2, 0xc7, 0x29, 0x89, 0xfe, 0x4e, 0x88,
                        0x72, 0x7c, 0x78, 0x2c, 0x33, 0x44, 0xca, 0x3a, 0x18, 0x01, 0x33, 0xf6, 0x77, 0xe9, 0xd2, 0x08],
                       [0x8c, 0x07, 0x28, 0x25, 0x79, 0x4a, 0xb7, 0xdd, 0xbd, 0x3d, 0xde, 0x99, 0xb6, 0x1b, 0x3f, 0x4c,
                        0x25, 0x18, 0x04, 0x48, 0x72, 0xae, 0xdf, 0x6a, 0x5b, 0x6d, 0x1c, 0x7c, 0x2b, 0x1f, 0xb0, 0x2c]
        );

        run_test_case!([0x38, 0x64, 0x27, 0xc2, 0x06, 0xde, 0x88, 0xf9, 0xd0, 0xaf, 0xa1, 0xd2, 0x52, 0x78, 0xf6, 0x68,
                        0x28, 0x1d, 0x88, 0xf7, 0x5e, 0xd2, 0x83, 0x41, 0x98, 0xbe, 0x9b, 0x8c, 0x91, 0xc1, 0x00, 0x6d],
                       [0x6d, 0x7f, 0x69, 0xc2, 0x4a, 0x95, 0xde, 0x99, 0x5a, 0xa6, 0x76, 0x64, 0xbd, 0x56, 0xd6, 0xf7,
                        0x4d, 0x17, 0x24, 0x2c, 0xee, 0xe6, 0xf5, 0x31, 0xc8, 0xda, 0x0c, 0x57, 0x2b, 0x11, 0xf2, 0x0f],
                       [0x4e, 0x9a, 0x2f, 0xfe, 0xca, 0x03, 0xec, 0xaa, 0x65, 0x0e, 0xc9, 0x17, 0xda, 0xfe, 0x8e, 0x16,
                        0xa1, 0x85, 0xc7, 0x6a, 0x57, 0x4c, 0x9e, 0x70, 0x14, 0xda, 0xa4, 0x67, 0x54, 0x36, 0xb0, 0x3c]
        );
        run_test_case!([0x88, 0xb9, 0xa7, 0xe5, 0x47, 0xb6, 0x45, 0x0f, 0xaf, 0x25, 0x15, 0x7e, 0xb6, 0xc1, 0x03, 0x4c,
                         0x8b, 0x5b, 0x9b, 0x13, 0x3e, 0x09, 0xc1, 0x31, 0x0f, 0x17, 0xa1, 0x4b, 0x0c, 0xa3, 0x30, 0x49],
                       [0xf3, 0x05, 0x63, 0x52, 0xc6, 0x86, 0x2b, 0x71, 0x65, 0x5b, 0x30, 0xa7, 0x77, 0x26, 0x7f, 0xa9,
                        0x19, 0x6f, 0x5d, 0xac, 0xcb, 0x05, 0x45, 0xdd, 0xc6, 0xb1, 0x75, 0x87, 0x03, 0xdb, 0x6d, 0x48],
                       [0x7b, 0xae, 0x21, 0x46, 0x51, 0x98, 0xa2, 0xa9, 0xf1, 0x16, 0x46, 0x0d, 0x3c, 0x3f, 0x00, 0x92,
                        0xe0, 0x41, 0x3e, 0xf4, 0x76, 0x3b, 0x56, 0xfc, 0x04, 0x27, 0x40, 0xd2, 0xb2, 0xc0, 0x19, 0x0f]
        );
        run_test_case!([0xb8, 0x9a, 0xd9, 0x3d, 0x6c, 0x21, 0x2e, 0xa5, 0xa8, 0x44, 0x92, 0x19, 0xbc, 0xb0, 0xb5, 0x4d,
                        0x60, 0xc9, 0xf2, 0xbd, 0x9a, 0x91, 0x01, 0x88, 0xc5, 0x44, 0x44, 0x69, 0xa9, 0x85, 0x5f, 0x5f],
                       [0xce, 0x52, 0x53, 0xa3, 0xf0, 0x14, 0x1f, 0x93, 0x3d, 0x4f, 0x0c, 0x9c, 0x29, 0xac, 0x5d, 0xa6,
                        0x87, 0xb5, 0xb9, 0x54, 0x4b, 0x6b, 0x6d, 0x39, 0xcf, 0xda, 0xa8, 0x11, 0x38, 0xc3, 0x89, 0x72],
                       [0x98, 0xbf, 0x9a, 0xd9, 0x8d, 0x3f, 0x6e, 0x06, 0x5c, 0x96, 0x1c, 0x2c, 0x83, 0x48, 0xe8, 0x73,
                        0x06, 0x23, 0xed, 0x9e, 0xdd, 0xe1, 0x56, 0x52, 0x3b, 0xc9, 0x48, 0xba, 0x23, 0x82, 0xca, 0x4d]
        );
        run_test_case!([0x80, 0xec, 0x10, 0xdd, 0xd1, 0xbb, 0x68, 0x80, 0x3a, 0x76, 0xeb, 0xe7, 0x13, 0xe5, 0xc5, 0xd8,
                        0xd1, 0x2d, 0x1b, 0xb0, 0xd6, 0xb4, 0x83, 0x39, 0x91, 0x6b, 0xe3, 0xe9, 0xee, 0x75, 0x9a, 0x48],
                       [0xf8, 0x72, 0xa9, 0x4d, 0x4d, 0xab, 0x47, 0xd3, 0xeb, 0x46, 0x13, 0x25, 0x0b, 0xb8, 0x23, 0x28,
                        0xdd, 0xa7, 0xc3, 0x57, 0x69, 0xd5, 0xa7, 0xa9, 0x5e, 0xc9, 0x4a, 0x8c, 0x5d, 0x6c, 0x8f, 0x1b],
                       [0xa3, 0x2d, 0x0f, 0x04, 0x37, 0x20, 0x07, 0xbb, 0xad, 0x04, 0x37, 0xd8, 0xad, 0x69, 0x6d, 0x4b,
                        0xa6, 0x7a, 0x56, 0x01, 0x96, 0x44, 0xac, 0xaa, 0x78, 0xee, 0xed, 0xef, 0x35, 0xb5, 0xe4, 0x48]
        );
        run_test_case!([0x38, 0x72, 0x34, 0x6c, 0x36, 0x5e, 0x01, 0x07, 0xb7, 0x9a, 0x42, 0xd6, 0xb5, 0x89, 0x97, 0xcc,
                        0x52, 0x97, 0x72, 0x8c, 0xf6, 0xcb, 0x02, 0x2b, 0x3d, 0x56, 0x45, 0xbe, 0xdb, 0xdf, 0x5e, 0x69],
                       [0x4f, 0x94, 0x4c, 0xd7, 0x70, 0xf5, 0x1c, 0xa9, 0x1c, 0x61, 0xa7, 0xeb, 0xc0, 0x01, 0xd7, 0x72,
                        0x6d, 0xee, 0x3e, 0x31, 0x9a, 0x96, 0xa1, 0x71, 0x18, 0x98, 0x36, 0x97, 0x61, 0xb8, 0x11, 0x20],
                       [0xcc, 0x8e, 0x42, 0x4b, 0x3d, 0x06, 0x2a, 0x8d, 0xa5, 0xf9, 0xf7, 0xc0, 0x94, 0x2f, 0xd0, 0xad,
                        0x19, 0x95, 0x0b, 0x6a, 0xb8, 0x71, 0x4b, 0xcc, 0xfe, 0x60, 0x2a, 0xa9, 0x89, 0x1e, 0xe9, 0x56]
        );
        run_test_case!([0x78, 0xb9, 0xe1, 0x31, 0xce, 0xa1, 0x03, 0x4c, 0x23, 0x0e, 0xb3, 0xdb, 0x4d, 0x66, 0x58, 0x50,
                        0xf0, 0xe6, 0x10, 0x6b, 0x6e, 0x59, 0x6f, 0x13, 0xbc, 0x30, 0x5e, 0x80, 0x24, 0x39, 0x95, 0x51],
                       [0xed, 0xc9, 0x8b, 0xda, 0x04, 0x86, 0x79, 0x00, 0x6a, 0x60, 0xde, 0x3b, 0x0c, 0x2e, 0xff, 0x15,
                        0x12, 0x9f, 0xe1, 0x97, 0x98, 0xd8, 0xcc, 0x4e, 0xfa, 0x0b, 0xd8, 0x81, 0x36, 0xa2, 0x78, 0x54],
                       [0x6a, 0xd9, 0x94, 0x91, 0x11, 0x36, 0x5b, 0xc2, 0x39, 0x19, 0x4d, 0xca, 0x1f, 0x5e, 0xbb, 0xa4,
                        0xa5, 0xbd, 0xb2, 0xcc, 0x12, 0x4e, 0x5d, 0xb1, 0xe6, 0xb8, 0x2e, 0x7d, 0xa7, 0xdd, 0x15, 0x38]
        );
        run_test_case!([0x78, 0xc6, 0x49, 0x10, 0xdf, 0x89, 0x55, 0xc7, 0x2e, 0xbc, 0x12, 0xf2, 0x9a, 0x11, 0xed, 0x10,
                        0xe4, 0x5e, 0x7b, 0xa9, 0x1c, 0x35, 0xbb, 0x1b, 0x1b, 0x44, 0xcc, 0xaf, 0x9c, 0xfb, 0xc9, 0x6c],
                       [0x37, 0x1e, 0x5a, 0x98, 0x93, 0x65, 0x70, 0xb0, 0xd5, 0xf5, 0x12, 0x68, 0xed, 0xff, 0x4d, 0x9b,
                        0xe5, 0x94, 0x9b, 0x3d, 0x79, 0xbe, 0xe0, 0x17, 0x33, 0xd6, 0xa8, 0x74, 0x25, 0xe7, 0x4c, 0x5e],
                       [0xb5, 0xaf, 0xe9, 0x0c, 0x6d, 0x62, 0x31, 0x5d, 0x4a, 0x42, 0x10, 0xb4, 0x07, 0xe3, 0xa5, 0xc5,
                        0x6b, 0xb5, 0x03, 0x37, 0x6d, 0x9a, 0x93, 0x50, 0x0a, 0x99, 0x2a, 0x3e, 0xed, 0x54, 0xba, 0x12]
        );
        run_test_case!([0x68, 0x70, 0xd9, 0x02, 0x81, 0x07, 0xee, 0x4d, 0x1d, 0xdd, 0xda, 0x44, 0xa9, 0x08, 0xa3, 0x6b,
                        0x01, 0xc1, 0x94, 0x95, 0x39, 0x9d, 0x52, 0xd4, 0xee, 0xf9, 0x40, 0xc4, 0x57, 0xfa, 0x71, 0x46],
                       [0x10, 0xdb, 0x38, 0x9c, 0x51, 0x78, 0xaa, 0xf4, 0x43, 0x2e, 0x03, 0x3c, 0x06, 0xe3, 0x82, 0xb1,
                        0x57, 0x89, 0xdf, 0xad, 0x2a, 0xe0, 0xb5, 0xf8, 0xc3, 0x82, 0x13, 0xae, 0x9e, 0xe3, 0xb8, 0x01],
                       [0x0b, 0x63, 0x5b, 0xd8, 0xa9, 0x6d, 0xbf, 0x93, 0x50, 0x05, 0xb1, 0x5f, 0xcf, 0x7a, 0xf2, 0x5d,
                        0xf4, 0x64, 0x6b, 0xcc, 0x8c, 0x51, 0xa9, 0x6a, 0xc6, 0x20, 0x12, 0x90, 0x65, 0xb9, 0x70, 0x0a]
        );
        run_test_case!([0x20, 0x40, 0x00, 0xb8, 0x07, 0x5c, 0x2d, 0x4a, 0x63, 0x6d, 0x14, 0x81, 0x43, 0x0d, 0x66, 0x32,
                        0xac, 0xe7, 0x8c, 0x6e, 0x53, 0x4d, 0xb5, 0x2a, 0xab, 0x78, 0xd0, 0x39, 0xf5, 0xe6, 0xa1, 0x4f],
                       [0x3e, 0x75, 0x59, 0x30, 0x3e, 0xb2, 0x13, 0x23, 0x76, 0x31, 0x62, 0x7f, 0xa0, 0x9c, 0x99, 0x14,
                        0x26, 0x29, 0x74, 0xc4, 0xe7, 0x88, 0xd3, 0xc4, 0x92, 0x39, 0xf5, 0xe6, 0xa1, 0xae, 0xca, 0x66],
                       [0x68, 0xf1, 0x3d, 0x04, 0xc0, 0x16, 0xcf, 0x6e, 0xdd, 0xb4, 0x8d, 0x4e, 0xa4, 0x3f, 0x7e, 0xbf,
                        0xbc, 0xc0, 0x9b, 0x09, 0xdd, 0x5b, 0x36, 0xb9, 0x35, 0xb1, 0x53, 0x50, 0x7b, 0x85, 0xe2, 0x22]
        );

    }

}
