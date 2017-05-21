use rand::{thread_rng, Rng};
use secp256k1;
use tiny_keccak::Keccak;
use std::fmt;

use address::Address;
use signature::Signature;
use ::{SECP256K1};

#[derive(Clone)]
pub struct SecretKey([u8;32]);

impl SecretKey {

    pub fn new() -> SecretKey {
        let mut v = [0u8; 32];
        thread_rng().fill_bytes(&mut v);
        SecretKey(v)
    }

    pub fn address(&self) -> Address {
        let mut sha3 = Keccak::new_keccak256();
        let privkey = secp256k1::key::SecretKey::from_slice(&SECP256K1, &self.0).unwrap();
        let pubkey = secp256k1::key::PublicKey::from_secret_key(&SECP256K1, &privkey).unwrap();
        let pubkeydata = &pubkey.serialize_vec(&SECP256K1, false);
        sha3.update(&pubkeydata[1..]);
        let mut res: [u8;32] = [0;32];
        sha3.finalize(&mut res);
        Address::from_slice(&res[12..32])
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn deserialize(serialised: &[u8]) -> SecretKey {
        let mut res: [u8;32] = [0;32];
        res.copy_from_slice(&serialised);
        SecretKey(res)
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let sk = secp256k1::key::SecretKey::from_slice(&SECP256K1, &self.0).unwrap();
        let rawhash = {
            let mut hash_output: [u8; 32] = [0; 32];
            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&data);
            sha3.finalize(&mut hash_output);
            secp256k1::Message::from_slice(&hash_output).unwrap()
        };
        let sig = SECP256K1.sign_recoverable(&rawhash, &sk).unwrap();
        let (recid, ret) = sig.serialize_compact(&SECP256K1);
        let mut res = [0u8;65];
        &res[..64].copy_from_slice(&ret);
        res[64] = recid.to_i32() as u8;
        Signature::from_slice(&res)
    }
}

impl AsRef<[u8]> for SecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::LowerHex for SecretKey {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for byte in self.0.to_vec() {
            try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}
