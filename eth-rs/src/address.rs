use rlp;
use rustc_hex::FromHex;
use std::fmt;

#[derive(Clone,PartialEq,Debug)]
pub struct Address([u8;20]);

impl Address {

    pub fn from_slice(slice: &[u8]) -> Address {
        let mut addr: [u8;20] = [0;20];
        addr.copy_from_slice(slice);
        Address(addr)
    }

    pub fn to_string(&self) -> String {
        format!("0x{:x}", &self)
    }
    pub fn from_string(string: &str) -> Address {
        assert_eq!(&string[..2], "0x");
        let mut addr: [u8;20] = [0;20];
        addr.copy_from_slice(&string[2..].from_hex().unwrap());
        Address(addr)
    }
}

// TODO: impl address checksums: https://github.com/ethereum/EIPs/issues/55
impl fmt::LowerHex for Address {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for byte in self.0.to_vec() {
            try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        }
        Ok(())
    }
}

impl rlp::Encodable for Address {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
	s.encoder().encode_value(&self.0);
    }
}
