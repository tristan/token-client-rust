extern crate crypto;

use self::crypto::{symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use self::crypto::symmetriccipher::Decryptor;

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

fn int_to_byte_array(value: u32) -> [u8;4] {
    let mut bytes: [u8;4] = [0;4];
    bytes[3] = value as u8;
    bytes[2] = (value >> 8) as u8;
    bytes[1] = (value >> 16) as u8;
    bytes[0] = (value >> 24) as u8;
    bytes
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
