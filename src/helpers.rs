
macro_rules! impl_array_struct {
    ($name:ident, $ty:ty, $size:expr) => {

        impl $name {
            pub fn to_vec(&self) -> std::vec::Vec<u8> {
                self.0.to_vec()
            }

            pub fn deserialize(serialised: &[$ty]) -> $name {
                let mut res: [$ty;$size] = [0;$size];
                res.copy_from_slice(&serialised);
                $name(res)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                for byte in self.0.to_vec() {
                    try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
                }
                Ok(())
            }
        }
    };
}


pub fn int_to_byte_array(value: u32) -> [u8;4] {
    let mut bytes: [u8;4] = [0;4];
    bytes[3] = value as u8;
    bytes[2] = (value >> 8) as u8;
    bytes[1] = (value >> 16) as u8;
    bytes[0] = (value >> 24) as u8;
    bytes
}
