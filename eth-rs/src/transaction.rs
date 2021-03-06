use rlp;
use secp256k1;
use bigint::U256;
use tiny_keccak::Keccak;
use address::Address;
use signature::Signature;
use keys::SecretKey;
use ::SECP256K1;

pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Vec<u8>,
    pub signature: Option<Signature>
}

impl rlp::Decodable for Transaction {
    fn decode(d: &rlp::UntrustedRlp) -> Result<Self, rlp::DecoderError> {
        Ok(Transaction {
            nonce: d.val_at(0)?,
            gas_price: d.val_at(1)?,
            gas: d.val_at(2)?,
            to: {
                let s: Vec<u8> = d.val_at(3)?;
                if s.len() == 0 {
                    None
                } else {
                    Some(Address::from_slice(&s))
                }
            },
            value: d.val_at(4)?,
            data: d.val_at(5)?,
            signature: {
                if d.item_count()? != 9 {
                    None
                } else {
                    let v: U256 = d.val_at(6)?;
		    let r: U256 = d.val_at(7)?;
		    let s: U256 = d.val_at(8)?;
                    Some(Signature::from_parts(&v, &r, &s))
                }
            }
        })
    }
}

impl rlp::Encodable for Transaction {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        self.rlp_encode_to_stream(s, self.signature.is_some());
    }
}

impl Transaction {

    pub fn rlp_encode(&self, signed: bool) -> Vec<u8> {
        let mut s = rlp::RlpStream::new();
        self.rlp_encode_to_stream(&mut s, signed);
        let out = s.out();
        out
    }

    fn rlp_encode_to_stream(&self, strm: &mut rlp::RlpStream, signed: bool) {
        let signed = signed && self.signature.is_some();
        if signed {
            strm.begin_list(9);
        } else {
            strm.begin_list(6);
        }
	strm.append(&self.nonce);
	strm.append(&self.gas_price);
	strm.append(&self.gas);
	if let Some(ref addr) = self.to.as_ref() {
            strm.append(*addr);
        } else {
            strm.append_empty_data();
        }
	strm.append(&self.value);
        if self.data.len() > 0 {
            strm.append(&self.data);
        } else {
            strm.append_empty_data();
        }
        if signed {
            if let Some(ref sig) = self.signature.as_ref() {
                let (v, r, s) = sig.get_parts();
                strm.append(&v);
                strm.append(&r);
                strm.append(&s);
            } else {
                unreachable!();
            }
        }
    }

    pub fn sender(&self) -> Option<Address> {
        if let Some(ref sig) = self.signature.as_ref() {
            let recsig = sig.to_recoverable_signature();
            let rawhash = self.hash_optional_sig(false);
            let msg = secp256k1::Message::from_slice(&rawhash).unwrap();
            let pubkey = SECP256K1.recover(&msg, &recsig).unwrap();
            if ! pubkey.is_valid() {
                return None;
            }
            let pubkey = pubkey.serialize_vec(&SECP256K1, false);

            let mut sha3 = Keccak::new_keccak256();
            sha3.update(&pubkey[1..]);
            let mut res: [u8;32] = [0;32];
            sha3.finalize(&mut res);
            Some(Address::from_slice(&res[12..32]))
        } else {
            None
        }
    }

    fn hash_optional_sig(&self, with_signature: bool) -> Vec<u8> {
        let mut sha3 = Keccak::new_keccak256();
        sha3.update(&self.rlp_encode(with_signature));
        let mut res: [u8;32] = [0;32];
        sha3.finalize(&mut res);
        res.to_vec()
    }

    pub fn hash(&self) -> Vec<u8> {
        self.hash_optional_sig(self.signature.is_some())
    }

    pub fn sign(&self, secret_key: &SecretKey) -> Transaction {

        let data = self.hash_optional_sig(false);
        let sig = secret_key.sign(&data);
        Transaction {
            nonce: self.nonce.clone(),
            gas_price: self.gas_price.clone(),
            gas: self.gas.clone(),
            to: self.to.clone(),
            value: self.value.clone(),
            data: self.data.clone(),
            signature: Some(sig)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_crypto_sign_modified() {

        macro_rules! run_test_case {
            ($sk:expr, $addr:expr, $msg:expr, $sig:expr) => {
                let msg: &[u8] = $msg;
                let sk: [u8; 32] = $sk;
                let expected_sig: [u8; 65] = $sig;
                let expected_addr = $addr;
                let sk = SecretKey::deserialize(&sk.as_ref());
                assert_eq!(format!("0x{:x}", sk.address()), expected_addr);
                let sig = sk.sign(msg);
                let sig = sig.as_ref();
                assert_eq!(sig[0..32], expected_sig[0..32]);
                assert_eq!(sig[32..64], expected_sig[32..64]);
                assert_eq!(sig[64], expected_sig[64]);
            };
        }

        run_test_case!(
            [0x82, 0xae, 0x99, 0x1e, 0xbb, 0x33, 0x0a, 0xa5, 0x93, 0x0b, 0xff, 0x8e, 0x0e, 0x63, 0xff, 0xa7,
             0x8d, 0x1e, 0x1a, 0xf1, 0x0c, 0x8a, 0x9e, 0xa5, 0x26, 0xe4, 0x4c, 0xc3, 0x86, 0xfe, 0xa7, 0x63],
            "0x67669080a24705679dba1c775fbc53aeae9cf076",
            b"hello, world!",
            [0x3e, 0xbc, 0x80, 0xec, 0x62, 0x08, 0x2a, 0xb3, 0x41, 0xec, 0x45, 0x9e, 0x67, 0x7a, 0x12, 0x9d,
             0x20, 0x38, 0x15, 0x4b, 0xf0, 0x73, 0x28, 0xd8, 0x7d, 0x3f, 0x4f, 0x9b, 0x36, 0x91, 0x35, 0x2e,
             0x19, 0x78, 0x1f, 0x4c, 0xc5, 0xa9, 0x21, 0xfc, 0x32, 0x92, 0xb3, 0x66, 0xca, 0xce, 0x75, 0xc0,
             0x2d, 0x1e, 0x56, 0xd7, 0xfd, 0x48, 0x88, 0x77, 0xae, 0x5c, 0x0b, 0x07, 0xae, 0x27, 0xc9, 0xf5,
             0x01]
        );
        run_test_case!(
            [0x5e, 0x37, 0x87, 0xb7, 0xb1, 0xf7, 0x3d, 0xc9, 0x70, 0x4d, 0xb4, 0x92, 0x64, 0x7a, 0xd5, 0x2d,
             0xc6, 0x33, 0x29, 0xb5, 0xd3, 0x33, 0xb3, 0x8a, 0x0b, 0x9f, 0x95, 0x0f, 0xd6, 0xaf, 0x7f, 0xd1],
            "0xe54e69cdc9f6fbff343446a672258fa460288f71",
            b"hello, world!",
            [0xbb, 0x59, 0xb6, 0x7d, 0x39, 0x43, 0xb6, 0x75, 0x88, 0x36, 0xd8, 0x15, 0x9f, 0x1d, 0x55, 0x07,
             0xdb, 0xe4, 0x50, 0xa1, 0x4e, 0x6e, 0x91, 0xe8, 0xe4, 0x35, 0x9c, 0x7c, 0x62, 0xd8, 0x8b, 0xcc,
             0x64, 0x06, 0x0b, 0xa5, 0x2a, 0x48, 0x2a, 0x64, 0x18, 0xd7, 0x97, 0xa7, 0x5f, 0xcb, 0xbe, 0x2c,
             0x81, 0x01, 0xb4, 0x7a, 0x3a, 0x57, 0xe4, 0x92, 0x9a, 0xc4, 0x9d, 0x6d, 0x6e, 0x29, 0x25, 0x91,
             0x00]
        );

        run_test_case!(
            [0x76, 0xa6, 0x43, 0xbf, 0x59, 0xf7, 0xe7, 0x46, 0x8d, 0xe2, 0xf4, 0xe2, 0xa1, 0xa8, 0x56, 0xbc,
             0x62, 0x3a, 0x74, 0xb7, 0x05, 0x6f, 0x8a, 0x8a, 0xd1, 0xc8, 0xf1, 0x30, 0x7e, 0xb3, 0x41, 0x1e],
            "0x426fc8435b05706e6f471bd58ddf007fe6f0ffa0",
            b"hello, world!",
            [0x0d, 0xc2, 0x66, 0x4f, 0xc4, 0xcc, 0x90, 0xf7, 0xd4, 0xf0, 0xc0, 0xc3, 0xab, 0xbf, 0x8e, 0x8e,
             0xa1, 0xce, 0x5f, 0x6d, 0x16, 0x85, 0x6b, 0x5c, 0x0e, 0x31, 0xf5, 0xed, 0x4f, 0x36, 0x7f, 0x69,
             0x06, 0xdb, 0xc5, 0x8a, 0xad, 0xe7, 0xc3, 0x72, 0xff, 0xa3, 0x20, 0x00, 0xcc, 0x11, 0x32, 0x3b,
             0x2b, 0x40, 0xf4, 0x59, 0x41, 0x11, 0xa5, 0xdd, 0x16, 0x8b, 0x3c, 0x64, 0x69, 0x28, 0x6b, 0x6e,
             0x00]
        );

        run_test_case!(
            [0xc7, 0x77, 0x18, 0x9d, 0x7c, 0xd3, 0x68, 0xe9, 0x24, 0x1d, 0xc9, 0x90, 0xb5, 0xd1, 0xc5, 0x12,
             0x4c, 0xc4, 0x92, 0x47, 0x98, 0x48, 0xd1, 0x66, 0xab, 0xfd, 0xaf, 0xb4, 0x81, 0xe0, 0xc4, 0x2f],
            "0x10f27c35db258e8595b9e733be1549e41108a561",
            b"hello, world!",
            [0x22, 0x03, 0xae, 0x6c, 0x2d, 0xb9, 0x6d, 0x2e, 0xbf, 0x0e, 0x4e, 0xdd, 0x08, 0xe9, 0x09, 0x18,
             0x38, 0x30, 0xae, 0xd6, 0x0a, 0x8d, 0xa8, 0xab, 0x41, 0x02, 0x13, 0x48, 0xb5, 0x73, 0x3a, 0x59,
             0x15, 0x27, 0x32, 0xeb, 0xc6, 0xb6, 0xd2, 0x14, 0xc8, 0x7e, 0xa6, 0xd4, 0x78, 0x39, 0x03, 0xe9,
             0x6f, 0xb4, 0xb2, 0x07, 0xad, 0x03, 0xa3, 0xd2, 0x87, 0xc4, 0x27, 0x35, 0x93, 0x30, 0x40, 0x12,
             0x00]
        );

        run_test_case!(
            [0x85, 0x21, 0x8c, 0xd4, 0xbb, 0x9b, 0xd3, 0xbc, 0x8f, 0xf9, 0x1e, 0xc8, 0xe3, 0xa7, 0xb2, 0x0b,
             0xcb, 0xe8, 0xe2, 0xce, 0x63, 0x89, 0xf2, 0xa4, 0xf0, 0xaa, 0x5e, 0xd9, 0x6b, 0x14, 0x63, 0xc2],
            "0x4f2fb36a06d1a9fcf11264b5a4db26205182404a",
            b"hello, world!",
            [0x8b, 0xa3, 0x00, 0xd3, 0x05, 0x40, 0x84, 0x93, 0x60, 0x6e, 0x0f, 0x96, 0x4a, 0x91, 0xb1, 0x7e,
             0x68, 0xec, 0xcd, 0x36, 0xa6, 0x39, 0xd6, 0x59, 0xce, 0xcd, 0xfe, 0x6f, 0x57, 0x65, 0x08, 0x64,
             0x72, 0x33, 0x53, 0x74, 0xb3, 0xe0, 0x0c, 0xac, 0xa9, 0x15, 0xdf, 0xcb, 0x0a, 0x77, 0x31, 0xbd,
             0x68, 0xab, 0xbf, 0xbe, 0x26, 0x1b, 0xc2, 0xed, 0x05, 0x2d, 0x87, 0x59, 0x52, 0x47, 0xf0, 0xf4,
             0x01]
        );

        run_test_case!(
            [0x92, 0x46, 0x46, 0x1c, 0x58, 0xca, 0xe0, 0x0b, 0x44, 0xe9, 0x5e, 0xc0, 0xd6, 0x79, 0xd7, 0x9e,
             0x0a, 0x0f, 0x3e, 0x5e, 0xd6, 0xa2, 0xbf, 0xe3, 0xd6, 0x40, 0x62, 0x69, 0x37, 0xa9, 0xb9, 0xb5],
            "0x6625d2f20fa02ed5fe6557300cea41397ee20464",
            b"hello, world!",
            [0x10, 0xd5, 0x28, 0x9e, 0xf0, 0x0b, 0x19, 0x3d, 0xef, 0x88, 0x38, 0xec, 0x39, 0xad, 0x71, 0x36,
             0x8f, 0x12, 0x15, 0xfd, 0x05, 0x62, 0xc2, 0x71, 0x05, 0x40, 0x27, 0xe1, 0x1a, 0x1c, 0xa6, 0x5b,
             0x3d, 0x39, 0x71, 0xd6, 0xea, 0x30, 0xe2, 0xb7, 0x49, 0xec, 0xa0, 0x47, 0x86, 0x2e, 0x4f, 0x63,
             0x84, 0x97, 0xd0, 0x2c, 0xab, 0xdd, 0x50, 0xef, 0x87, 0x3c, 0xe4, 0x6a, 0x54, 0x06, 0x06, 0x34,
             0x01]
        );

        run_test_case!(
            [0x43, 0x40, 0x61, 0x95, 0x2a, 0x28, 0x11, 0x39, 0x11, 0x7b, 0x0a, 0xf2, 0x58, 0x8c, 0x50, 0x2c,
             0x34, 0x0b, 0x17, 0x93, 0x00, 0x40, 0x0a, 0x7f, 0xda, 0x86, 0xf2, 0x98, 0xf1, 0x9f, 0x36, 0x41],
            "0xa93106f39db78c665368e1904ad6fb9cd329264c",
            b"hello, world!",
            [0xb6, 0x0e, 0x7a, 0x6f, 0xf6, 0x1e, 0x3a, 0x30, 0xf9, 0xb6, 0x55, 0xfd, 0xa8, 0xb0, 0x9c, 0xe8,
             0x0d, 0x99, 0xc0, 0xd9, 0x02, 0x28, 0xbb, 0x2f, 0x52, 0x17, 0xcb, 0xcb, 0xa4, 0x20, 0x75, 0xc5,
             0x26, 0xce, 0x4b, 0xcd, 0x46, 0x8e, 0x3b, 0x6c, 0x09, 0x81, 0xd4, 0x39, 0xde, 0x79, 0xa0, 0xba,
             0x22, 0xba, 0x7d, 0xbb, 0x5c, 0xa3, 0xb6, 0x38, 0x88, 0xaf, 0x7e, 0x91, 0x8c, 0xaa, 0xc6, 0xf2,
             0x01]
        );

        run_test_case!(
            [0x9d, 0x47, 0xad, 0xe0, 0xdd, 0x32, 0xdd, 0x87, 0x65, 0xf4, 0x64, 0x10, 0xe9, 0x4c, 0xbe, 0xa6,
             0x0d, 0x66, 0x93, 0xc5, 0x0f, 0x13, 0x38, 0x36, 0xa2, 0xef, 0xf9, 0x64, 0x76, 0x71, 0x99, 0xe6],
            "0x13b3c8c6e8d829101bc34dc01bc3839357ea0fe1",
            b"hello, world!",
            [0xec, 0x6f, 0xd8, 0x60, 0x8b, 0xad, 0xbf, 0x91, 0x90, 0x63, 0x9a, 0xc6, 0x74, 0x9a, 0x6d, 0xbb,
             0x66, 0x87, 0xfb, 0x5e, 0xe8, 0x58, 0x49, 0x23, 0x3f, 0xff, 0xb2, 0xd1, 0xb1, 0x24, 0x47, 0x00,
             0x20, 0x75, 0x74, 0xd5, 0xc4, 0xf9, 0xad, 0xcb, 0x57, 0xa7, 0x65, 0x82, 0x9a, 0x18, 0x71, 0xa3,
             0x0c, 0xf8, 0x80, 0x49, 0xc8, 0x77, 0x9d, 0xcb, 0x5f, 0x42, 0x1d, 0x89, 0xc5, 0xab, 0xf7, 0xbd,
             0x00]
        );

        run_test_case!(
            [0x45, 0xae, 0x56, 0x95, 0xe2, 0x73, 0xdb, 0x55, 0xbd, 0x58, 0xe7, 0x54, 0xb9, 0xdd, 0x23, 0xe4,
             0x10, 0xcf, 0xd8, 0x20, 0x35, 0xfa, 0x84, 0xd9, 0xcb, 0x0a, 0x2c, 0xfc, 0x55, 0x34, 0x18, 0x33],
            "0xcf8de6e25b54c56a12f15b8ae919c815797809ea",
            b"hello, world!",
            [0x7a, 0x7d, 0x9c, 0x21, 0x5f, 0xc6, 0xe8, 0xd2, 0xb6, 0xb1, 0xfe, 0x9f, 0xdc, 0xec, 0x3b, 0xfd,
             0xdb, 0x15, 0x2b, 0xa8, 0xd3, 0xdb, 0xeb, 0x55, 0xdc, 0x95, 0x62, 0x4c, 0x51, 0x2a, 0x09, 0x4e,
             0x1d, 0xd1, 0xa0, 0xe5, 0xd1, 0x53, 0x0c, 0xce, 0xd3, 0x1a, 0xdc, 0xc6, 0x49, 0x0a, 0x7c, 0x14,
             0xb6, 0x81, 0x48, 0x36, 0x49, 0xf6, 0xcf, 0x59, 0x17, 0xab, 0xb1, 0x6a, 0x20, 0x80, 0x2c, 0xec,
             0x01]
        );

        run_test_case!(
            [0xd8, 0xe9, 0x2c, 0x92, 0xde, 0x74, 0x89, 0x06, 0x7e, 0x91, 0x03, 0x75, 0x50, 0xaf, 0x48, 0x4c,
             0x8a, 0xe7, 0xb9, 0x32, 0xa6, 0x67, 0x83, 0x55, 0x18, 0x5b, 0x0a, 0x9c, 0x5e, 0x92, 0xdb, 0xc0],
            "0xe6009ab89381ed2fa178714e19f57ff0682bf2a0",
            b"hello, world!",
            [0x7a, 0xfb, 0x3c, 0x21, 0x4a, 0x97, 0xe8, 0x48, 0x5e, 0xe9, 0x05, 0x54, 0x5e, 0xbf, 0x2b, 0x1d,
             0xe8, 0x52, 0x34, 0xf7, 0xfa, 0x1a, 0xa9, 0xb8, 0xef, 0xbe, 0xee, 0x1a, 0xf0, 0xdc, 0x97, 0x53,
             0x74, 0x66, 0x0e, 0x7a, 0x6f, 0xcd, 0xae, 0x9c, 0x3c, 0xdb, 0x00, 0x8c, 0x2d, 0xe6, 0x35, 0xbc,
             0xed, 0xe7, 0xbb, 0x5a, 0x4d, 0x0b, 0x32, 0x02, 0xd4, 0x45, 0x5a, 0x6d, 0xd9, 0x20, 0x0b, 0x54,
             0x00]
        );

        run_test_case!(
            [0xe1, 0xb5, 0xe1, 0x19, 0x33, 0x77, 0x13, 0x40, 0x32, 0x4a, 0xe5, 0xc7, 0x34, 0xcb, 0x9b, 0xd4,
             0xde, 0x56, 0x73, 0xe2, 0x40, 0x97, 0xfb, 0xbf, 0xd8, 0x16, 0xf7, 0x51, 0xee, 0xaf, 0xa6, 0xe8],
            "0x99d2a37a3db5777b3fd9cca8409a0c8687aaccaf",
            b"hello, world!",
            [0x80, 0xfa, 0x58, 0x01, 0x1b, 0xd0, 0x92, 0x1b, 0xf9, 0x0a, 0x45, 0x4b, 0x64, 0x60, 0x3b, 0xa0,
             0xf6, 0x43, 0x1e, 0x80, 0x8c, 0x6e, 0xcd, 0xa3, 0x8e, 0xef, 0x46, 0x5c, 0x88, 0xc1, 0x67, 0x89,
             0x6a, 0xd2, 0x49, 0x42, 0x99, 0xfc, 0xe1, 0xb9, 0x09, 0xe9, 0x74, 0x3c, 0x6f, 0x36, 0xb2, 0x09,
             0x92, 0x61, 0x92, 0x1f, 0xad, 0x44, 0x19, 0x83, 0x75, 0x9c, 0x92, 0x15, 0x1e, 0x48, 0xde, 0x7f,
             0x00]
        );

    }

    #[test]
    fn test_int_to_big_endian() {
        let i = U256::from_str("ac23ec5f33e73983a96b8480bd50b30513518c2ca6b76498ef7cbf9352e4b33f").unwrap();
        let v = vec![0xac, 0x23, 0xec, 0x5f, 0x33, 0xe7, 0x39, 0x83, 0xa9, 0x6b,
                     0x84, 0x80, 0xbd, 0x50, 0xb3, 0x05, 0x13, 0x51, 0x8c, 0x2c,
                     0xa6, 0xb7, 0x64, 0x98, 0xef, 0x7c, 0xbf, 0x93, 0x52, 0xe4,
                     0xb3, 0x3f];
        let mut wtr = [0u8; 32];
        i.to_big_endian(&mut wtr);
        assert_eq!(v, wtr);

                let i = U256::from_str("5f33e73983a96b8480bd50b30513518c2ca6b76498ef7cbf9352e4b33f").unwrap();
        let v = vec![0x00, 0x00, 0x00, 0x5f, 0x33, 0xe7, 0x39, 0x83, 0xa9, 0x6b,
                     0x84, 0x80, 0xbd, 0x50, 0xb3, 0x05, 0x13, 0x51, 0x8c, 0x2c,
                     0xa6, 0xb7, 0x64, 0x98, 0xef, 0x7c, 0xbf, 0x93, 0x52, 0xe4,
                     0xb3, 0x3f];
        let mut wtr = [0u8; 32];
        i.to_big_endian(&mut wtr);
        assert_eq!(v, wtr);
    }

    #[test]
    fn test_transaction_rlp_decode() {

        macro_rules! run_test_case {
            ($tx_rlp:expr, $nonce:expr, $gasprice:expr, $gas:expr, $to:expr, $value:expr, $data:expr, $sig:expr) => {
                let unsigned_bytes: Vec<u8> = $tx_rlp;
                let nonce: U256 = U256::from($nonce);
                let gasprice = U256::from_str($gasprice).unwrap();
                let gas = U256::from($gas);
                let to: Option<Address> = $to;
                let value: U256 = U256::from_str($value).unwrap();
                let data: Vec<u8> = $data;
                let signature: Option<Signature> = $sig;

                let tx: Transaction = rlp::decode(&unsigned_bytes);
                assert_eq!(tx.nonce, nonce);
                assert_eq!(tx.gas_price, gasprice);
                assert_eq!(tx.gas, gas);
                assert_eq!(tx.to, to);
                assert_eq!(tx.value, value);
                assert_eq!(tx.data, data);
                assert_eq!(tx.signature, signature);
            };
        }
        run_test_case!(
            vec![0xe9, 0x83, 0x10, 0x00, 0x00, 0x85, 0x04, 0xa8, 0x17, 0xc8,
                 0x00, 0x82, 0x52, 0x08, 0x94, 0x05, 0x6d, 0xb2, 0x90, 0xf8,
                 0xba, 0x32, 0x50, 0xca, 0x64, 0xa4, 0x5d, 0x16, 0x28, 0x4d,
                 0x04, 0xbc, 0x6f, 0x5f, 0xbf, 0x85, 0x02, 0x54, 0x0b, 0xe4,
                 0x00, 0x80],
            1048576,
            "4a817c800",
            21000,
            Some(Address::from_slice(
                &[0x05, 0x6d, 0xb2, 0x90, 0xf8, 0xba, 0x32, 0x50, 0xca, 0x64,
                  0xa4, 0x5d, 0x16, 0x28, 0x4d, 0x04, 0xbc, 0x6f, 0x5f, 0xbf])),
            "2540be400",
            vec![],
            None);

        run_test_case!(
            vec![0xdc, 0x83, 0x10, 0x00, 0x00, 0x85, 0x04, 0xa8, 0x17, 0xc8,
                 0x00, 0x83, 0x08, 0x16, 0x50, 0x80, 0x80, 0x8b, 0x48, 0x65,
                 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64],
            1048576,
            "4a817c800",
            530000,
            None,
            "0",
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64],
            None);

        run_test_case!(
            vec![0xf8, 0x5f, 0x83, 0x10, 0x00, 0x00, 0x85, 0x04, 0xa8, 0x17,
                 0xc8, 0x00, 0x83, 0x08, 0x16, 0x50, 0x80, 0x80, 0x8b, 0x48,
                 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
                 0x1c, 0xa0, 0xac, 0x23, 0xec, 0x5f, 0x33, 0xe7, 0x39, 0x83,
                 0xa9, 0x6b, 0x84, 0x80, 0xbd, 0x50, 0xb3, 0x05, 0x13, 0x51,
                 0x8c, 0x2c, 0xa6, 0xb7, 0x64, 0x98, 0xef, 0x7c, 0xbf, 0x93,
                 0x52, 0xe4, 0xb3, 0x3f, 0xa0, 0x2f, 0xc8, 0x5b, 0x42, 0x0d,
                 0xf4, 0x9f, 0x10, 0x34, 0xb3, 0xd1, 0xcd, 0xcf, 0x4a, 0xf0,
                 0xac, 0xa5, 0x93, 0x6e, 0x26, 0x72, 0x69, 0x4a, 0x11, 0x61,
                 0xd2, 0xb0, 0x77, 0xd9, 0x1d, 0x13, 0x44],
            1048576,
            "4a817c800",
            530000,
            None,
            "0",
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64],
            Some(Signature::from_slice(
                &[0xac, 0x23, 0xec, 0x5f, 0x33, 0xe7, 0x39, 0x83, 0xa9, 0x6b,
                  0x84, 0x80, 0xbd, 0x50, 0xb3, 0x05, 0x13, 0x51, 0x8c, 0x2c,
                  0xa6, 0xb7, 0x64, 0x98, 0xef, 0x7c, 0xbf, 0x93, 0x52, 0xe4,
                  0xb3, 0x3f, 0x2f, 0xc8, 0x5b, 0x42, 0x0d, 0xf4, 0x9f, 0x10,
                  0x34, 0xb3, 0xd1, 0xcd, 0xcf, 0x4a, 0xf0, 0xac, 0xa5, 0x93,
                  0x6e, 0x26, 0x72, 0x69, 0x4a, 0x11, 0x61, 0xd2, 0xb0, 0x77,
                  0xd9, 0x1d, 0x13, 0x44, 0x01])));
    }

    #[test]
    fn test_transaction_get_sender_and_hash() {
        let tx = vec![0xf8, 0x6c, 0x83, 0x10, 0x00, 0x00, 0x85, 0x04, 0xa8, 0x17, 0xc8, 0x00, 0x82, 0x52, 0x08, 0x94,
                      0x05, 0x6d, 0xb2, 0x90, 0xf8, 0xba, 0x32, 0x50, 0xca, 0x64, 0xa4, 0x5d, 0x16, 0x28, 0x4d, 0x04,
                      0xbc, 0x6f, 0x5f, 0xbf, 0x85, 0x02, 0x54, 0x0b, 0xe4, 0x00, 0x80, 0x1c, 0xa0, 0x11, 0x46, 0x55,
                      0xdb, 0x48, 0x98, 0xa6, 0x58, 0x0f, 0x0a, 0xbf, 0xc5, 0x3f, 0xc0, 0xc0, 0xa8, 0x81, 0x10, 0x72,
                      0x4a, 0xbf, 0x8d, 0x41, 0xf2, 0xab, 0xf2, 0x06, 0xc6, 0x9d, 0x7d, 0x4c, 0x82, 0xa0, 0x1e, 0xd2,
                      0xcd, 0xf6, 0x93, 0x94, 0x84, 0xef, 0x6a, 0xeb, 0xc3, 0x9c, 0xe5, 0x66, 0x23, 0x63, 0xb8, 0x21,
                      0x40, 0x10, 0x6b, 0xbc, 0x37, 0x4a, 0x0f, 0x13, 0x81, 0xb6, 0x94, 0x82, 0x14, 0xb0];
        let address = Some(Address::from_string("0xde3d2d9dd52ea80f7799ef4791063a5458d13913"));

        let tx: Transaction = rlp::decode(&tx);
        assert_eq!(tx.sender(), address);

        // test tx with data
        let tx = vec![
            0xf9, 0x01, 0x0c, 0x83, 0x10, 0x00, 0x00, 0x85, 0x04, 0xa8, 0x17, 0xc8, 0x00, 0x82, 0x56, 0x18,
            0x94, 0xa0, 0xc4, 0xd4, 0x9f, 0xe1, 0xa0, 0x0e, 0xb5, 0xee, 0x3d, 0x85, 0xdc, 0x7a, 0x28, 0x7d,
            0x84, 0xd8, 0xc6, 0x66, 0x99, 0x80, 0xb8, 0xa4, 0x94, 0xd9, 0xcf, 0x8f, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0xa0, 0x5a, 0x76,
            0x32, 0xc1, 0xd2, 0x35, 0xaa, 0x99, 0xa1, 0xf2, 0xaa, 0x85, 0x49, 0xb5, 0x34, 0x1c, 0x06, 0xed,
            0x6f, 0xee, 0xb5, 0xbd, 0x0c, 0xd0, 0x1e, 0xf9, 0x93, 0x54, 0xf8, 0x2e, 0x3d, 0xee, 0xa0, 0x3f,
            0x98, 0x19, 0x73, 0xa4, 0xdc, 0x46, 0xdc, 0x9c, 0x8d, 0x74, 0xf5, 0xae, 0xfd, 0x06, 0xb4, 0xf6,
            0x26, 0x6e, 0xd3, 0x5f, 0x21, 0x99, 0x60, 0x58, 0x35, 0x5a, 0x3f, 0xd1, 0xf7, 0xf8, 0x00];
        let hash = vec![
            0x87, 0x40, 0x0d, 0x7a, 0x8f, 0xbb, 0x0c, 0x26, 0xeb, 0xe0, 0xad, 0x4e, 0x4c, 0x9a, 0x84, 0xb0,
            0x33, 0xe3, 0x59, 0x08, 0x41, 0x55, 0x57, 0xb0, 0xb5, 0xeb, 0x73, 0x8e, 0x2a, 0x41, 0xc0, 0x67];
        let tx: Transaction = rlp::decode(&tx);
        assert_eq!(tx.sender(), address);
        assert_eq!(tx.hash(), hash);

    }
}
