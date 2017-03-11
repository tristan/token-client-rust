
use super::test_protocol_store::TestProtocolStore;

use ::signal::keys::{IdentityKeyPair};
use ::signal::state::{PreKeyRecord, SignedPreKeyRecord};

use ::signal::message::PreKeySignalMessage;
use ::signal::protocol::SignalProtocolAddress;
use ::signal::session::decrypt_prekey_message;
use ::signal::protocol::{SignedPreKeyStore,PreKeyStore};

#[test]
fn decrypt_message() {

    let msg = vec![
        0x33, 0x8, 0x0, 0x12, 0x21, 0x5, 0x40, 0xe4, 0xd5, 0xce, 0x40, 0x8f, 0x66, 0xcd, 0xdb, 0xde,
        0xe6, 0x4f, 0x9, 0x4c, 0x48, 0xf, 0x7d, 0x9d, 0xbb, 0x48, 0xf5, 0xb1, 0x61, 0x64, 0x57, 0x67,
        0x45, 0xa2, 0x98, 0x52, 0x36, 0x14, 0x1a, 0x21, 0x5, 0xe1, 0x94, 0xbf, 0xa0, 0x4c, 0xe6, 0xc7,
        0x1e, 0xdb, 0x3c, 0x92, 0xf2, 0x8, 0x52, 0xe5, 0xd0, 0x95, 0x98, 0xe2, 0x97, 0x9, 0x9b, 0xfc,
        0x82, 0x96, 0xde, 0xdc, 0x30, 0xf9, 0x98, 0x70, 0x4, 0x22, 0xd3, 0x1, 0x33, 0xa, 0x21, 0x5,
        0xae, 0x26, 0x46, 0x36, 0x21, 0xc, 0x71, 0xcd, 0xda, 0x2c, 0x55, 0x32, 0x1c, 0xbc, 0x2a, 0x89,
        0xcc, 0x96, 0xb0, 0x68, 0xde, 0x44, 0x95, 0xa5, 0x90, 0x89, 0x99, 0x3, 0x7d, 0x78, 0x8, 0x11,
        0x10, 0x0, 0x18, 0x0, 0x22, 0xa0, 0x1, 0x3e, 0x15, 0xe3, 0x48, 0xbc, 0x6f, 0x4f, 0xd9, 0x43,
        0x8e, 0x3e, 0x54, 0xff, 0x14, 0x87, 0x63, 0x4b, 0x7b, 0xae, 0xcf, 0x49, 0x4, 0x7b, 0x5c, 0x75,
        0x85, 0x71, 0x20, 0x22, 0xcb, 0x59, 0x61, 0x9, 0xb2, 0x2a, 0x7f, 0xa, 0x27, 0x61, 0xe0, 0xd4,
        0x71, 0x57, 0xbe, 0x17, 0xd7, 0xe4, 0x69, 0xba, 0xf7, 0xbd, 0x27, 0xa8, 0xbb, 0x68, 0x38, 0x5b,
        0x80, 0x48, 0x53, 0x9b, 0x2e, 0x70, 0x8e, 0x18, 0x95, 0x73, 0x8d, 0x38, 0xe4, 0xff, 0xd, 0x77,
        0x38, 0x4, 0x4b, 0x9f, 0x50, 0xd5, 0x5c, 0xe1, 0x77, 0x40, 0xbe, 0x28, 0x4d, 0xcd, 0x96, 0x7f,
        0xbe, 0x7f, 0xa0, 0xe6, 0x6e, 0x25, 0x61, 0xa0, 0x9f, 0xd1, 0x7c, 0x48, 0xf5, 0xe7, 0x27, 0x60,
        0x69, 0x85, 0x54, 0x16, 0xda, 0x3c, 0xe4, 0xb2, 0x4c, 0x9a, 0x39, 0x78, 0x91, 0xff, 0x39, 0x35,
        0x11, 0x27, 0x95, 0x80, 0x86, 0xc2, 0xa6, 0xeb, 0x97, 0x88, 0x76, 0xe3, 0x85, 0xb1, 0xa3, 0xed,
        0x1, 0xae, 0xd3, 0xa6, 0x64, 0xcb, 0xa8, 0x8c, 0xa7, 0x46, 0x10, 0x5c, 0x1d, 0x62, 0xa4, 0xbf,
        0x2a, 0x9, 0x1a, 0x1c, 0xc0, 0xe5, 0x1, 0xe3, 0xa4, 0x74, 0x8b, 0x93, 0xc3, 0x4e, 0x6, 0x28,
        0xbc, 0x67, 0x30, 0x0];

    let expected_plaintext: Vec<u8> = vec![
        0x0a, 0x32, 0x53, 0x4f, 0x46, 0x41, 0x3a, 0x3a, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x3a,
        0x7b, 0x22, 0x62, 0x6f, 0x64, 0x79, 0x22, 0x3a, 0x22, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x22, 0x2c,
        0x22, 0x73, 0x68, 0x6f, 0x77, 0x4b, 0x65, 0x79, 0x62, 0x6f, 0x61, 0x72, 0x64, 0x22, 0x3a, 0x74,
        0x72, 0x75, 0x65, 0x7d, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    let prekeymsg = PreKeySignalMessage::deserialize(&msg).unwrap();
    assert_eq!(prekeymsg.get_version(), 3);
    assert_eq!(prekeymsg.get_registration_id(), 13244);
    assert_eq!(prekeymsg.get_pre_key_id().unwrap(), 0);
    assert_eq!(prekeymsg.get_identity_key().serialize(), vec![
        0x05, 0xe1, 0x94, 0xbf, 0xa0, 0x4c, 0xe6, 0xc7, 0x1e, 0xdb, 0x3c, 0x92, 0xf2, 0x08, 0x52, 0xe5,
        0xd0, 0x95, 0x98, 0xe2, 0x97, 0x09, 0x9b, 0xfc, 0x82, 0x96, 0xde, 0xdc, 0x30, 0xf9, 0x98, 0x70,
        0x04]);
    assert_eq!(prekeymsg.get_base_key().serialize(), vec![
        0x05, 0x40, 0xe4, 0xd5, 0xce, 0x40, 0x8f, 0x66, 0xcd, 0xdb, 0xde, 0xe6, 0x4f, 0x09, 0x4c, 0x48,
        0x0f, 0x7d, 0x9d, 0xbb, 0x48, 0xf5, 0xb1, 0x61, 0x64, 0x57, 0x67, 0x45, 0xa2, 0x98, 0x52, 0x36,
        0x14]);

    let message = prekeymsg.get_whisper_message();
    assert_eq!(message.get_version(), 3);
    assert_eq!(message.get_counter(), 0);
    assert_eq!(message.get_body().to_vec(),
               vec![0x3e, 0x15, 0xe3, 0x48, 0xbc, 0x6f, 0x4f, 0xd9, 0x43, 0x8e, 0x3e, 0x54, 0xff, 0x14, 0x87, 0x63,
                    0x4b, 0x7b, 0xae, 0xcf, 0x49, 0x04, 0x7b, 0x5c, 0x75, 0x85, 0x71, 0x20, 0x22, 0xcb, 0x59, 0x61,
                    0x09, 0xb2, 0x2a, 0x7f, 0x0a, 0x27, 0x61, 0xe0, 0xd4, 0x71, 0x57, 0xbe, 0x17, 0xd7, 0xe4, 0x69,
                    0xba, 0xf7, 0xbd, 0x27, 0xa8, 0xbb, 0x68, 0x38, 0x5b, 0x80, 0x48, 0x53, 0x9b, 0x2e, 0x70, 0x8e,
                    0x18, 0x95, 0x73, 0x8d, 0x38, 0xe4, 0xff, 0x0d, 0x77, 0x38, 0x04, 0x4b, 0x9f, 0x50, 0xd5, 0x5c,
                    0xe1, 0x77, 0x40, 0xbe, 0x28, 0x4d, 0xcd, 0x96, 0x7f, 0xbe, 0x7f, 0xa0, 0xe6, 0x6e, 0x25, 0x61,
                    0xa0, 0x9f, 0xd1, 0x7c, 0x48, 0xf5, 0xe7, 0x27, 0x60, 0x69, 0x85, 0x54, 0x16, 0xda, 0x3c, 0xe4,
                    0xb2, 0x4c, 0x9a, 0x39, 0x78, 0x91, 0xff, 0x39, 0x35, 0x11, 0x27, 0x95, 0x80, 0x86, 0xc2, 0xa6,
                    0xeb, 0x97, 0x88, 0x76, 0xe3, 0x85, 0xb1, 0xa3, 0xed, 0x01, 0xae, 0xd3, 0xa6, 0x64, 0xcb, 0xa8,
                    0x8c, 0xa7, 0x46, 0x10, 0x5c, 0x1d, 0x62, 0xa4, 0xbf, 0x2a, 0x09, 0x1a, 0x1c, 0xc0, 0xe5, 0x01]);
    assert_eq!(message.serialize(),
               vec![0x33, 0x0a, 0x21, 0x05, 0xae, 0x26, 0x46, 0x36, 0x21, 0x0c, 0x71, 0xcd, 0xda, 0x2c, 0x55, 0x32,
                    0x1c, 0xbc, 0x2a, 0x89, 0xcc, 0x96, 0xb0, 0x68, 0xde, 0x44, 0x95, 0xa5, 0x90, 0x89, 0x99, 0x03,
                    0x7d, 0x78, 0x08, 0x11, 0x10, 0x00, 0x18, 0x00, 0x22, 0xa0, 0x01, 0x3e, 0x15, 0xe3, 0x48, 0xbc,
                    0x6f, 0x4f, 0xd9, 0x43, 0x8e, 0x3e, 0x54, 0xff, 0x14, 0x87, 0x63, 0x4b, 0x7b, 0xae, 0xcf, 0x49,
                    0x04, 0x7b, 0x5c, 0x75, 0x85, 0x71, 0x20, 0x22, 0xcb, 0x59, 0x61, 0x09, 0xb2, 0x2a, 0x7f, 0x0a,
                    0x27, 0x61, 0xe0, 0xd4, 0x71, 0x57, 0xbe, 0x17, 0xd7, 0xe4, 0x69, 0xba, 0xf7, 0xbd, 0x27, 0xa8,
                    0xbb, 0x68, 0x38, 0x5b, 0x80, 0x48, 0x53, 0x9b, 0x2e, 0x70, 0x8e, 0x18, 0x95, 0x73, 0x8d, 0x38,
                    0xe4, 0xff, 0x0d, 0x77, 0x38, 0x04, 0x4b, 0x9f, 0x50, 0xd5, 0x5c, 0xe1, 0x77, 0x40, 0xbe, 0x28,
                    0x4d, 0xcd, 0x96, 0x7f, 0xbe, 0x7f, 0xa0, 0xe6, 0x6e, 0x25, 0x61, 0xa0, 0x9f, 0xd1, 0x7c, 0x48,
                    0xf5, 0xe7, 0x27, 0x60, 0x69, 0x85, 0x54, 0x16, 0xda, 0x3c, 0xe4, 0xb2, 0x4c, 0x9a, 0x39, 0x78,
                    0x91, 0xff, 0x39, 0x35, 0x11, 0x27, 0x95, 0x80, 0x86, 0xc2, 0xa6, 0xeb, 0x97, 0x88, 0x76, 0xe3,
                    0x85, 0xb1, 0xa3, 0xed, 0x01, 0xae, 0xd3, 0xa6, 0x64, 0xcb, 0xa8, 0x8c, 0xa7, 0x46, 0x10, 0x5c,
                    0x1d, 0x62, 0xa4, 0xbf, 0x2a, 0x09, 0x1a, 0x1c, 0xc0, 0xe5, 0x01, 0xe3, 0xa4, 0x74, 0x8b, 0x93,
                    0xc3, 0x4e, 0x06]);
    let remote_address = SignalProtocolAddress::new("0x946377c114849aaa1b08deae139d481df6974519".to_string(), 1);

    let idkey = IdentityKeyPair::deserialize(&vec![
        0x0a, 0x21, 0x05, 0x1e, 0x3d, 0xda, 0x2a, 0x49, 0x35, 0x21, 0xa0, 0x60, 0x3a, 0x34, 0x25, 0x99,
        0x3d, 0xe0, 0x8c, 0x88, 0x86, 0x59, 0x6a, 0x16, 0x85, 0xe8, 0xaa, 0xbd, 0xfd, 0x52, 0xea, 0xec,
        0x30, 0x10, 0x31, 0x12, 0x20, 0xe0, 0x2b, 0xe6, 0x09, 0xef, 0xed, 0xf4, 0xf9, 0xbc, 0x20, 0xf0,
        0x00, 0xd0, 0x90, 0xd0, 0xa1, 0x6a, 0x61, 0xed, 0xd6, 0x9b, 0xae, 0x55, 0x28, 0x7a, 0xb0, 0x4a,
        0x30, 0x2e, 0x4f, 0x8c, 0x61]);

    let mut store = TestProtocolStore::new(&idkey, 10267);

    let our_address = SignalProtocolAddress::new("0x1d23733a6422b83be138d83dcd1746ad8f90dd04".to_string(), 1);

    let signedprekey = SignedPreKeyRecord::deserialize(&vec![
        0x08, 0x00, 0x12, 0x21, 0x05, 0xd7, 0xdf, 0x01, 0x29, 0x6b, 0x22, 0x1b, 0x64, 0xab, 0x8a, 0x4e,
        0x22, 0x4e, 0xb7, 0x6a, 0xbf, 0x3e, 0x8e, 0x17, 0x51, 0x9a, 0xab, 0x7b, 0x8c, 0xed, 0xfe, 0x55,
        0x92, 0x04, 0x2f, 0xe7, 0x12, 0x1a, 0x20, 0x58, 0x5b, 0x55, 0xc7, 0xed, 0xd2, 0x76, 0xcb, 0x52,
        0x92, 0x6a, 0xe3, 0x57, 0xff, 0xcf, 0xa2, 0x11, 0x07, 0x1f, 0x5c, 0x36, 0x23, 0x06, 0x42, 0xce,
        0x21, 0xc6, 0xe2, 0x8a, 0x21, 0x7f, 0x72, 0x22, 0x40, 0x8c, 0x64, 0x5a, 0xe8, 0x5e, 0x34, 0xb4,
        0x17, 0x0a, 0xb5, 0x9d, 0x58, 0xcb, 0x4f, 0xcc, 0x28, 0x53, 0xfd, 0x79, 0xb5, 0x06, 0x68, 0xd5,
        0x33, 0x9f, 0x0c, 0xe1, 0x59, 0xef, 0x5e, 0xa1, 0xff, 0x71, 0x62, 0xa7, 0x4b, 0x83, 0xf6, 0x69,
        0xef, 0xb1, 0xc3, 0xce, 0xb6, 0x4f, 0x87, 0x97, 0xbf, 0x34, 0xfd, 0x65, 0x60, 0x8c, 0x1a, 0x0b,
        0xc9, 0xf0, 0x8c, 0x0a, 0xd1, 0x5f, 0xe3, 0x24, 0x82, 0x29, 0xd7, 0x30, 0xb7, 0x58, 0x00, 0x00,
        0x00, 0x00]);

    store.store_signed_pre_key(0, &signedprekey);

    // prekeys
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 0, 18, 33, 5, 149, 80, 187, 153, 203, 28, 97, 47, 188, 94, 55, 33,
              178, 92, 191, 166, 36, 71, 187, 252, 55, 26, 238, 134, 59, 35, 36, 95,
              52, 64, 254, 114, 26, 32, 200, 132, 124, 133, 105, 210, 240, 169, 68,
              74, 22, 227, 186, 105, 58, 175, 19, 177, 64, 42, 13, 8, 192, 18, 118,
              238, 63, 249, 106, 24, 168, 97]);
    store.store_pre_key(0, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 1, 18, 33, 5, 63, 205, 187, 149, 255, 161, 182, 248, 216, 211, 252,
              18, 18, 161, 27, 151, 38, 165, 79, 21, 244, 172, 181, 43, 100, 7, 206,
              161, 208, 170, 219, 66, 26, 32, 208, 203, 73, 223, 136, 83, 40, 90,
              160, 76, 61, 170, 227, 21, 239, 50, 107, 12, 184, 32, 8, 136, 234, 18,
              179, 203, 39, 71, 157, 66, 205, 79]);
    store.store_pre_key(1, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 2, 18, 33, 5, 191, 140, 232, 105, 48, 98, 225, 155, 60, 88, 202, 5,
              250, 178, 155, 231, 129, 123, 42, 223, 189, 140, 206, 181, 152, 61,
              116, 117, 29, 94, 68, 6, 26, 32, 104, 8, 76, 133, 40, 47, 209, 196, 18,
              253, 87, 212, 193, 92, 179, 28, 158, 244, 33, 238, 88, 171, 58, 86,
              129, 49, 116, 79, 200, 74, 239, 72]);
    store.store_pre_key(2, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 3, 18, 33, 5, 178, 29, 77, 4, 142, 120, 216, 253, 92, 110, 71, 198,
              222, 165, 1, 22, 202, 11, 104, 243, 41, 87, 26, 93, 251, 98, 252, 230,
              198, 44, 176, 112, 26, 32, 200, 187, 50, 18, 12, 131, 16, 219, 117,
              210, 152, 118, 138, 122, 52, 216, 154, 217, 192, 17, 49, 121, 237,
              122, 3, 53, 214, 47, 150, 192, 97, 80]);
    store.store_pre_key(3, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 4, 18, 33, 5, 106, 217, 195, 170, 93, 195, 204, 213, 173, 162, 24,
              244, 7, 38, 249, 214, 232, 191, 211, 36, 103, 51, 6, 91, 212, 227,
              133, 250, 95, 155, 240, 80, 26, 32, 224, 146, 2, 50, 82, 172, 168,
              83, 9, 217, 210, 41, 159, 250, 226, 149, 243, 177, 207, 140, 210, 32,
              102, 78, 79, 52, 215, 213, 196, 111, 190, 99]);
    store.store_pre_key(4, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 5, 18, 33, 5, 32, 186, 59, 193, 244, 173, 44, 77, 11, 202, 226,
              53, 65, 206, 224, 102, 129, 139, 2, 226, 104, 1, 160, 63, 222, 226,
              87, 28, 136, 41, 27, 60, 26, 32, 192, 64, 151, 161, 228, 201, 116,
              241, 163, 116, 139, 110, 225, 57, 92, 108, 124, 231, 38, 87, 60,
              208, 229, 246, 6, 181, 234, 240, 157, 96, 43, 73]);
    store.store_pre_key(5, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 6, 18, 33, 5, 239, 93, 202, 189, 122, 192, 139, 197, 105, 91,
              107, 215, 134, 59, 90, 220, 180, 45, 164, 36, 107, 127, 137, 55,
              169, 134, 138, 104, 144, 64, 232, 20, 26, 32, 240, 241, 26, 154,
              74, 92, 61, 140, 77, 58, 154, 157, 91, 125, 5, 224, 15, 231, 43,
              73, 80, 5, 164, 67, 197, 187, 170, 239, 174, 236, 34, 94]);
    store.store_pre_key(6, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 7, 18, 33, 5, 77, 15, 62, 50, 91, 162, 74, 121, 184, 79, 248,
              223, 162, 136, 247, 184, 210, 185, 223, 194, 110, 12, 238, 59,
              121, 32, 226, 127, 158, 97, 13, 107, 26, 32, 184, 208, 146, 60,
              7, 34, 78, 224, 250, 133, 28, 121, 121, 145, 124, 251, 187, 203,
              219, 162, 199, 168, 49, 198, 149, 203, 23, 229, 174, 247, 122, 82]);
    store.store_pre_key(7, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 8, 18, 33, 5, 136, 182, 5, 109, 105, 187, 51, 126, 19, 107,
              33, 213, 167, 61, 231, 131, 55, 210, 184, 241, 112, 221, 65, 164,
              255, 233, 134, 208, 219, 147, 4, 104, 26, 32, 80, 240, 242, 65,
              74, 161, 99, 234, 58, 220, 214, 252, 170, 207, 145, 236, 17, 223,
              176, 117, 148, 241, 65, 20, 11, 229, 252, 143, 130, 161, 52, 121]);
    store.store_pre_key(8, &pre_key);
    let pre_key = PreKeyRecord::deserialize(
        &vec![8, 9, 18, 33, 5, 114, 15, 231, 123, 63, 254, 41, 251, 245, 36,
              178, 120, 57, 148, 111, 208, 120, 44, 56, 152, 253, 67, 145, 123,
              184, 96, 212, 33, 214, 102, 65, 45, 26, 32, 8, 220, 72, 72, 88,
              36, 28, 59, 204, 144, 215, 22, 96, 61, 65, 145, 104, 30, 194,
              143, 46, 112, 163, 195, 246, 73, 66, 42, 140, 130, 175, 96]);
    store.store_pre_key(9, &pre_key);

    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 99, 18, 33, 5, 26, 173, 109, 164, 221, 137, 215, 35, 232, 17, 166,
        252, 155, 14, 48, 205, 234, 46, 149, 147, 15, 92, 229, 54, 121, 252,
        40, 56, 36, 45, 174, 3, 26, 32, 48, 139, 217, 58, 24, 91, 56, 225,
        130, 66, 106, 116, 101, 6, 94, 94, 140, 99, 12, 216, 156, 90, 138,
        41, 99, 121, 32, 25, 225, 238, 27, 121]);
    store.store_pre_key(99, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 98, 18, 33, 5, 214, 89, 96, 63, 214, 34, 53, 152, 210, 226, 122,
        236, 120, 10, 115, 189, 246, 129, 72, 130, 47, 1, 108, 44, 62, 9,
        210, 46, 234, 51, 57, 125, 26, 32, 224, 77, 151, 146, 121, 141, 215,
        128, 110, 181, 203, 217, 214, 201, 92, 54, 254, 12, 142, 117, 12, 36,
        199, 164, 43, 33, 104, 190, 211, 242, 113, 117]);
    store.store_pre_key(98, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 97, 18, 33, 5, 13, 206, 96, 181, 84, 125, 170, 115, 205, 127, 18,
        245, 208, 195, 101, 60, 108, 132, 31, 38, 39, 129, 77, 140, 123, 222,
        246, 113, 60, 68, 233, 97, 26, 32, 216, 11, 168, 172, 32, 114, 69, 42,
        8, 195, 100, 109, 149, 184, 210, 182, 28, 76, 227, 224, 159, 11, 163,
        146, 78, 113, 182, 51, 251, 101, 27, 74]);
    store.store_pre_key(97, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 96, 18, 33, 5, 183, 37, 176, 208, 121, 191, 93, 35, 208, 249, 143,
        224, 237, 244, 233, 68, 133, 106, 109, 184, 226, 182, 143, 29, 3, 170,
        53, 135, 208, 126, 80, 21, 26, 32, 112, 235, 153, 134, 72, 97, 208,
        17, 34, 168, 108, 112, 201, 36, 241, 36, 175, 198, 130, 143, 157, 128,
        202, 227, 65, 66, 62, 229, 157, 249, 205, 66]);
    store.store_pre_key(96, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 95, 18, 33, 5, 124, 166, 214, 248, 152, 19, 241, 67, 59, 173, 141,
        53, 228, 142, 222, 227, 158, 232, 137, 141, 96, 241, 178, 65, 146, 103,
        116, 9, 18, 195, 205, 67, 26, 32, 176, 54, 77, 61, 253, 94, 27, 84,
        181, 253, 172, 70, 56, 174, 124, 206, 164, 253, 113, 228, 60, 84, 169,
        130, 227, 6, 204, 148, 175, 250, 23, 116]);
    store.store_pre_key(95, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 94, 18, 33, 5, 25, 31, 130, 36, 240, 79, 187, 208, 225, 114, 58, 157,
        62, 236, 91, 40, 184, 201, 87, 220, 42, 226, 74, 162, 169, 140, 56, 219,
        128, 67, 183, 107, 26, 32, 184, 62, 119, 157, 2, 143, 174, 91, 129, 251,
        219, 63, 117, 121, 225, 220, 136, 74, 73, 104, 23, 14, 160, 107, 158,
        229, 132, 246, 30, 172, 219, 82]);
    store.store_pre_key(94, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 93, 18, 33, 5, 223, 58, 109, 185, 163, 148, 41, 26, 36, 197, 29, 75,
        53, 27, 67, 108, 141, 115, 107, 51, 43, 64, 146, 167, 108, 27, 84, 243,
        155, 246, 135, 59, 26, 32, 32, 58, 251, 231, 47, 69, 155, 147, 14, 13,
        34, 238, 254, 237, 204, 253, 142, 9, 69, 81, 21, 169, 61, 155, 130, 71,
        188, 110, 82, 13, 200, 112]);
    store.store_pre_key(93, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 92, 18, 33, 5, 173, 246, 155, 171, 253, 246, 33, 232, 70, 42, 89,
        226, 244, 68, 37, 0, 170, 237, 132, 170, 235, 132, 245, 92, 119, 112,
        170, 181, 123, 180, 208, 20, 26, 32, 128, 30, 35, 51, 155, 227, 247,
        164, 72, 34, 171, 170, 110, 3, 189, 103, 132, 212, 188, 122, 125, 214,
        240, 148, 72, 181, 165, 162, 142, 193, 134, 87]);
    store.store_pre_key(92, &pre_key);
    let pre_key = PreKeyRecord::deserialize(&vec![
        8, 91, 18, 33, 5, 68, 24, 180, 137, 117, 220, 186, 203, 97, 72, 77,
        247, 166, 152, 99, 55, 125, 33, 115, 143, 61, 202, 204, 143, 221, 131,
        146, 76, 6, 124, 8, 121, 26, 32, 88, 46, 6, 74, 170, 75, 16, 157, 194,
        102, 201, 113, 252, 197, 185, 41, 108, 220, 71, 237, 111, 12, 149, 47,
        243, 249, 66, 206, 60, 113, 125, 121]);
    store.store_pre_key(91, &pre_key);

    let plaintext = decrypt_prekey_message(&mut store, &remote_address, &prekeymsg).unwrap();
    assert_eq!(plaintext, expected_plaintext);
}
