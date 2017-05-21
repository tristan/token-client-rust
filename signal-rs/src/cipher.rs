extern crate crypto;

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use self::crypto::symmetriccipher::{Decryptor,Encryptor};
use self::crypto::mac::Mac;
use self::crypto::hmac::Hmac;
use self::crypto::sha2::Sha256;

pub const IV_OFFSET: usize = 1;
pub const IV_LENGTH: usize = 16;
pub const CIPHER_KEY_SIZE: usize = 32;
pub const MAC_SIZE: usize = 10;
//pub const MAC_KEY_SIZE: usize = 20;
pub const CIPHERTEXT_OFFSET: usize = IV_OFFSET + IV_LENGTH;
//pub const VERSION_OFFSET: usize = 0;
pub const VERSION_LENGTH: usize = 1;

pub fn int_to_byte_array(value: u32) -> [u8;4] {
    let mut bytes: [u8;4] = [0;4];
    bytes[3] = value as u8;
    bytes[2] = (value >> 8) as u8;
    bytes[1] = (value >> 16) as u8;
    bytes[0] = (value >> 24) as u8;
    bytes
}

// taken from https://github.com/DaGenix/rust-crypto/blob/master/examples/symmetriccipher.rs
pub fn decrypt_cbc(encrypted_data: &[u8], key: &[u8], iv: &[u8])
                   -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

pub fn decrypt_ctr(encrypted_data: &[u8], key: &[u8], counter: u32)
                   -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::ctr(
        aes::KeySize::KeySize256,
        key,
        &int_to_byte_array(counter));

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}


// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
pub fn encrypt_cbc(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

pub fn encrypt_ctr(data: &[u8], key: &[u8], counter: u32) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut encryptor = aes::ctr(
        aes::KeySize::KeySize256,
        key,
        &int_to_byte_array(counter));

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

pub fn verify_mac(ciphertext: &[u8], mac_key: &[u8]) -> bool {
    let mut mac = Hmac::new(Sha256::new(), &mac_key);
    if ciphertext.len() < MAC_SIZE + 1 {
        panic!("Invalid MAC!");
    }
    mac.input(&ciphertext[..(ciphertext.len() - MAC_SIZE)]);
    let thismac = mac.result().code()[..MAC_SIZE].to_vec();
    let thatmac = ciphertext[(ciphertext.len() - MAC_SIZE)..].to_vec();
    thismac == thatmac
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encrypt_cbc() {
        let plaintext = "L'homme est condamné à être libre".as_bytes();
        let key = vec![0x21, 0x9a, 0x70, 0x18, 0x49, 0x10, 0x6f, 0x84, 0x1c, 0xca, 0xea, 0xb7, 0x16, 0x6b, 0x68, 0x73,
                         0x2b, 0xac, 0xbd, 0x19, 0xc0, 0x41, 0xbb, 0x0d, 0xf5, 0x93, 0x4a, 0x4a, 0x53, 0x4c, 0x58, 0xbb];
        let iv = vec![0x59, 0x67, 0x2a, 0x91, 0x2d, 0x39, 0x35, 0x07, 0x64, 0xb8, 0x2c, 0x45, 0xc9, 0x94, 0x3f, 0x71];
        let expected = vec![0x3e, 0x95, 0x43, 0xf5, 0x90, 0x66, 0x45, 0x0e, 0x50, 0x69, 0x9b, 0xef, 0x5a, 0xf8, 0xe7, 0x0e,
                              0x49, 0xb2, 0x97, 0xdd, 0xf6, 0x5c, 0xba, 0x7f, 0xd9, 0x3d, 0xc4, 0xfb, 0xc8, 0x33, 0xba, 0x5d,
                              0x20, 0x40, 0x77, 0xc8, 0x00, 0xac, 0xfb, 0xb8, 0x10, 0x1b, 0x41, 0xc9, 0x09, 0x74, 0x6a, 0xd5];

        assert_eq!(encrypt_cbc(&plaintext, &key, &iv).unwrap(), expected);
    }
}
