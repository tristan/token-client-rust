use bigint::U256;
use std::{fmt,cmp};
use secp256k1;
use ::SECP256K1;

pub struct Signature([u8;65]);

impl cmp::PartialEq for Signature {
    fn eq(&self, rhs: &Signature) -> bool {
        self.0[..32] == rhs.0[..32] && self.0[32..64] == rhs.0[32..64] && self.0[64] == rhs.0[64]
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[").unwrap();
        for b in self.0.iter() {
            write!(f, "{}, ", b).unwrap();
        }
        write!(f, "]")
    }
}

impl fmt::LowerHex for Signature {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for byte in self.0.to_vec() {
            try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

impl Signature {

    pub fn from_slice(slice: &[u8]) -> Signature {
        let mut sig: [u8;65] = [0;65];
        sig.copy_from_slice(slice);
        Signature(sig)
    }

    pub fn from_parts(v: &U256, r: &U256, s: &U256) -> Signature {
        let mut sig = [0u8; 65];
        r.to_big_endian(&mut sig[..32]);
        s.to_big_endian(&mut sig[32..64]);
        sig[64] = v.byte(0) - 27;
        Signature(sig)
    }

    pub fn to_recoverable_signature(&self) -> secp256k1::RecoverableSignature {
        let recid = secp256k1::RecoveryId::from_i32((self.0[64]) as i32).unwrap();
        secp256k1::RecoverableSignature::from_compact(&SECP256K1, &self.0[..64], recid).unwrap()
    }

    pub fn get_parts(&self) -> (U256, U256, U256) {
        let r: U256 = U256::from(&self.0[..32]);
	let s: U256 = U256::from(&self.0[32..64]);
	let v: U256 = U256::from(self.0[64] + 27);
        (v, r, s)
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
