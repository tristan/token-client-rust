extern crate bip39;
extern crate crypto;

use secp256k1;
use ::{SECP256K1};
use keys::SecretKey;
use self::crypto::hmac::Hmac;
use self::crypto::mac::Mac;
use self::crypto::sha2::Sha512;
use self::bip39::{Bip39, KeyType, Language};

pub fn int_to_byte_array(value: u32) -> [u8;4] {
    let mut bytes: [u8;4] = [0;4];
    bytes[3] = value as u8;
    bytes[2] = (value >> 8) as u8;
    bytes[1] = (value >> 16) as u8;
    bytes[0] = (value >> 24) as u8;
    bytes
}

pub struct ChainCode([u8; 32]);

impl ChainCode {
    pub fn from_slice(slice: &[u8]) -> ChainCode {
        let mut res: [u8;32] = [0;32];
        res.copy_from_slice(&slice);
        ChainCode(res)
    }
}

pub struct HDWallet {
    mnemonic: String,
    master_key: SecretKey,
    chain_code: ChainCode
}

impl HDWallet {
    pub fn new(passphrase: &str) -> HDWallet {
        let kt = KeyType::for_word_length(12).unwrap();
        let b = Bip39::new(&kt, Language::English, passphrase).unwrap();

        HDWallet::from_bip39(&b)
    }

    pub fn from_mnemonic(mnemonic: &str, passphrase: &str) -> HDWallet {
        let b = Bip39::from_mnemonic(mnemonic, Language::English, passphrase).unwrap();
        HDWallet::from_bip39(&b)
    }

    fn from_bip39(b: &Bip39) -> HDWallet {

        let mut result = [0; 64];
        let mut hmac = Hmac::new(Sha512::new(), b"Bitcoin seed");
        hmac.input(&b.seed);
        hmac.raw_result(&mut result);

        let extprivkey = SecretKey::deserialize(&result[..32]);
        let chaincode = ChainCode::from_slice(&result[32..]);

        HDWallet {
            mnemonic: b.mnemonic.to_string(),
            master_key: extprivkey,
            chain_code: chaincode
        }
    }

    pub fn mnemonic(&self) -> String {
        self.mnemonic.clone()
    }

    pub fn derive_path(&self, path: &str) -> Result<SecretKey,String> {
        let mut chaincode: [u8;32] = [0;32];
        chaincode.copy_from_slice(&self.chain_code.0);
        let mut secretkey = secp256k1::key::SecretKey::from_slice(
            &SECP256K1, self.master_key.as_ref()).unwrap();

        for node in path.split("/") {
            let node = node.to_string();
            if node == "m" { continue; }
            // validate path node
            let (number, hardened) = {
                if node.ends_with("'") || node.ends_with("h") {
                    (node[..node.len()-1].to_string(), true)
                } else {
                    (node, false)
                }
            };
            let n = match number.parse::<u32>() {
                Ok(n) => n,
                Err(e) => {
                    return Err(format!("Invalid derivation path: {}", e));
                }
            };

            let mut result = [0; 64];
            let mut hmac = Hmac::new(Sha512::new(), chaincode.as_ref());
            if hardened {
                hmac.input(&[0u8]);
                hmac.input(&secretkey[..]);
                hmac.input(&int_to_byte_array(n + (1 << 31)));
            } else {
                hmac.input(&secp256k1::key::PublicKey::from_secret_key(
                    &SECP256K1, &secretkey).unwrap().serialize_vec(&SECP256K1, true)[..]);
                hmac.input(&int_to_byte_array(n));
            }
            hmac.raw_result(&mut result);
            let mut sk = secp256k1::key::SecretKey::from_slice(&SECP256K1, &result[..32]).unwrap();
            sk.add_assign(&SECP256K1, &secretkey).unwrap();
            secretkey = sk;
            chaincode.copy_from_slice(&result[32..]);
        }
        Ok(SecretKey::deserialize(&secretkey[..]))
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_mnemonic() {
        let wallet = HDWallet::from_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "");
        assert_eq!(wallet.chain_code.0,
                   [0x79, 0x23, 0x40, 0x8d, 0xad, 0xd3, 0xc7, 0xb5,
                    0x6e, 0xed, 0x15, 0x56, 0x77, 0x07, 0xae, 0x5e,
                    0x5d, 0xca, 0x08, 0x9d, 0xe9, 0x72, 0xe0, 0x7f,
                    0x3b, 0x86, 0x04, 0x50, 0xe2, 0xa3, 0xb7, 0x0e]);

        assert_eq!(wallet.master_key.as_ref(),
                   [0x18, 0x37, 0xc1, 0xbe, 0x8e, 0x29, 0x95, 0xec,
                    0x11, 0xcd, 0xa2, 0xb0, 0x66, 0x15, 0x1b, 0xe2,
                    0xcf, 0xb4, 0x8a, 0xdf, 0x9e, 0x47, 0xb1, 0x51,
                    0xd4, 0x6a, 0xda, 0xb3, 0xa2, 0x1c, 0xdf, 0x67]);

        let wallet = HDWallet::from_mnemonic(
            "legal winner thank year wave sausage worth useful legal winner thank yellow",
            "");
        assert_eq!(wallet.chain_code.0,
                   [0x59, 0x8b, 0x45, 0x95, 0xea, 0x72, 0x80, 0x27,
                    0x56, 0x51, 0x9e, 0x65, 0xa7, 0x97, 0x23, 0x42,
                    0x31, 0xd7, 0xd4, 0xf1, 0x3d, 0x65, 0x0c, 0xb0,
                    0x6d, 0xb1, 0x59, 0x57, 0xc2, 0x36, 0x8b, 0x1b]);
        assert_eq!(wallet.master_key.as_ref(),
                   [0x7e, 0x56, 0xec, 0xf5, 0x94, 0x3d, 0x79, 0xe1,
                    0xf5, 0xf8, 0x7e, 0x11, 0xc7, 0x68, 0x25, 0x3d,
                    0x7f, 0x3f, 0xcf, 0x30, 0xae, 0x71, 0x33, 0x56,
                    0x11, 0xe3, 0x66, 0xc5, 0x78, 0xb4, 0x56, 0x4e]);

        let wallet = HDWallet::from_mnemonic(
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
            "TREZOR");
        assert_eq!(wallet.chain_code.0,
                   [0x2a, 0xca, 0x53, 0x30, 0xa8, 0x7a, 0x78, 0x90,
                   0xf1, 0x0c, 0x84, 0x9e, 0xbf, 0x65, 0x8c, 0x3c,
                   0xef, 0x30, 0xcc, 0x5b, 0xd9, 0xd3, 0x12, 0x4b,
                   0xd2, 0x36, 0x18, 0x5d, 0x4c, 0xfc, 0xb1, 0x07]);
        assert_eq!(wallet.master_key.as_ref(),
                   [0xe1, 0x33, 0x0e, 0x46, 0xe8, 0x8f, 0x1c, 0x65,
                    0xcc, 0x1e, 0x22, 0x8a, 0x16, 0xe3, 0xf0, 0xb9,
                    0x4a, 0x31, 0x6a, 0xe4, 0xfc, 0xfd, 0xa1, 0xdf,
                    0x49, 0x96, 0xb8, 0x5c, 0x70, 0xd7, 0xb9, 0x09]);

        let wallet = HDWallet::from_mnemonic(
            "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
            "TREZOR");
        assert_eq!(wallet.chain_code.0,
                   [0x6d, 0xf8, 0x3b, 0x12, 0x40, 0x74, 0x19, 0x13,
                    0x98, 0x84, 0xbd, 0x46, 0x49, 0x7e, 0x9f, 0xfa,
                    0xbf, 0x29, 0xff, 0x99, 0x7c, 0x84, 0xb0, 0x0e,
                    0x5b, 0x51, 0x8c, 0xd0, 0x61, 0x08, 0xa4, 0x27]);
        assert_eq!(wallet.master_key.as_ref(),
                   [0x67, 0x9b, 0xf9, 0x2c, 0x04, 0xcf, 0x16, 0x30,
                    0x70, 0x53, 0xcb, 0xed, 0x33, 0x78, 0x4f, 0x3c,
                    0x42, 0x66, 0xb3, 0x62, 0xbf, 0x5f, 0x3d, 0x7e,
                    0xe1, 0x3b, 0xed, 0x6f, 0x27, 0x19, 0x74, 0x3c]);
    }

    #[test]
    fn test_path_derivation() {
        let wallet = HDWallet::from_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "");
        let sk = wallet.derive_path("m/44'/60'/0'/0/0").unwrap();
        assert_eq!(sk.as_ref(),
                   [0x1a, 0xb4, 0x2c, 0xc4, 0x12, 0xb6, 0x18, 0xbd,
                    0xea, 0x3a, 0x59, 0x9e, 0x3c, 0x9b, 0xae, 0x19,
                    0x9e, 0xbf, 0x03, 0x08, 0x95, 0xb0, 0x39, 0xe9,
                    0xdb, 0x1e, 0x30, 0xda, 0xfb, 0x12, 0xb7, 0x27]);
        assert_eq!(sk.address().to_string(), "0x9858effd232b4033e47d90003d41ec34ecaeda94");

        let sk = wallet.derive_path("m/44'/60'/0'/0/18").unwrap();
        assert_eq!(sk.as_ref(),
                   [0x63, 0x4d, 0x86, 0x7c, 0x47, 0xe9, 0x39, 0x64,
                    0x35, 0xe9, 0x0e, 0x9b, 0x63, 0x7c, 0xa1, 0xe6,
                    0xb1, 0xc7, 0x89, 0xf0, 0xe7, 0xbb, 0x15, 0x81,
                    0x4e, 0x11, 0x6b, 0x09, 0xd5, 0xd8, 0xdc, 0xb7]);
        assert_eq!(sk.address().to_string(), "0x944a807c53bce5a96dd4e558e993833ab41ce65f");
    }
}
