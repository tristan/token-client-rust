
extern crate tiny_keccak;
extern crate secp256k1;

use std;
use self::tiny_keccak::Keccak;
use rand::{thread_rng, Rng};

lazy_static! {
	static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

pub struct SecretKey([u8;32]);
pub struct Address([u8;20]);

impl std::fmt::LowerHex for Address {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(fmtr.write_fmt(format_args!("0x")));
        for byte in &self.0 {
            try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

impl SecretKey {
    pub fn address(&self) -> Address {
        let mut sha3 = Keccak::new_keccak256();
        let privkey = secp256k1::key::SecretKey::from_slice(&SECP256K1, &self.0).unwrap();
        let pubkey = secp256k1::key::PublicKey::from_secret_key(&SECP256K1, &privkey).unwrap();
        let pubkeydata = &pubkey.serialize_vec(&SECP256K1, false);
        sha3.update(&pubkeydata[1..]);
        let mut res: [u8;32] = [0;32];
        sha3.finalize(&mut res);
        let mut addr: [u8;20] = [0;20];
        addr.copy_from_slice(&res[12..32]);
        Address(addr)
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn deserialize(serialised: &Vec<u8>) -> SecretKey {
        let mut res: [u8;32] = [0;32];
        res.copy_from_slice(&serialised);
        SecretKey(res)
    }
}

pub fn generate_secret_key() -> SecretKey {
    let mut v = [0u8; 32];
    thread_rng().fill_bytes(&mut v);
    SecretKey(v)
}
