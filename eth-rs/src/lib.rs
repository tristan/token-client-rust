extern crate bigint;
extern crate rand;
extern crate rlp;
extern crate rustc_serialize;
extern crate tiny_keccak;
extern crate secp256k1;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

pub mod address;
pub mod keys;
pub mod signature;
pub mod wallet;
pub mod transaction;

pub type Address = address::Address;
pub type SecretKey = keys::SecretKey;
