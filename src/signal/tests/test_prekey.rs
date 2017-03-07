use super::test_protocol_store::TestProtocolStore;

use ::keys::{IdentityKeyPair,ECKeyPair,SignedPreKeyRecord,PreKeyRecord};

use ::signal::message::{CipherTextMessage,MessageType,PreKeySignalMessage,SignalMessage};
use ::signal::protocol::SignalProtocolAddress;
use ::signal::session;
use ::signal::state::PreKeyBundle;
use ::signal::protocol::{SessionStore,SignedPreKeyStore,PreKeyStore,IdentityKeyStore};

use rand::{OsRng, Rng};

#[test]
fn test_basic_prekey_v3() {

    let mut rng = OsRng::new().ok().unwrap();

    let alice_address = SignalProtocolAddress::new("0x946377c114849aaa1b08deae139d481df6974519".to_string(), 1);
    let alice_identity_keypair = IdentityKeyPair::generate();
    let alice_registration_id = rng.gen_range(1, 16381);
    let bob_address = SignalProtocolAddress::new("0x1d23733a6422b83be138d83dcd1746ad8f90dd04".to_string(), 1);
    let bob_identity_keypair = IdentityKeyPair::generate();
    let bob_registration_id = rng.gen_range(1, 16381);

    let mut alice_store = TestProtocolStore::new(&alice_identity_keypair, alice_registration_id);
    //let mut alice_session_builder = SessionBuilder::new(&mut alice_store, &bob_address);
    let mut bob_store = TestProtocolStore::new(&bob_identity_keypair, bob_registration_id);

    let bob_prekey_pair = ECKeyPair::generate();
    let bob_signed_prekey_record = SignedPreKeyRecord::generate(22, &bob_identity_keypair);
    let bob_signed_prekey_pair = bob_signed_prekey_record.get_key_pair();
    let bob_signed_prekey_signature = bob_signed_prekey_record.get_signature();

    let bob_prekey = PreKeyBundle::new(bob_store.get_local_registration_id(), 1,
                      Some(31337), Some(bob_prekey_pair.get_public_key()),
                      22, bob_signed_prekey_pair.get_public_key(),
                      &bob_signed_prekey_signature.to_vec(),
                      bob_store.get_identity_key_pair().get_public_key());

    session::process_prekey_bundle(&mut alice_store, &bob_address, &bob_prekey).unwrap();

    assert!(alice_store.contains_session(&bob_address));
    assert_eq!(alice_store.load_session(&bob_address).get_session_state().get_session_version(), 3);

    let original_message = "L'homme est condamné à être libre".as_bytes().to_vec();
    let outgoing_message = session::encrypt_message(&mut alice_store, &bob_address, &original_message).unwrap();

    assert_eq!(outgoing_message.get_type(), MessageType::PREKEY);
    let outgoing_message = outgoing_message.as_any().downcast_ref::<PreKeySignalMessage>().unwrap();

    let incoming_message = PreKeySignalMessage::deserialize(&outgoing_message.serialize()).unwrap();
    bob_store.store_pre_key(31337, &PreKeyRecord::new(bob_prekey.get_prekey_id().unwrap(), &bob_prekey_pair));
    bob_store.store_signed_pre_key(bob_signed_prekey_record.get_id(), &bob_signed_prekey_record);

    let plaintext = session::decrypt_prekey_message(&mut bob_store, &alice_address, &incoming_message).unwrap();
    assert_eq!(plaintext, original_message);
    // TODO: decryption callback?
    assert!(bob_store.contains_session(&alice_address));
    assert_eq!(bob_store.load_session(&alice_address).get_session_state().get_session_version(), 3);
    assert!(bob_store.load_session(&alice_address).get_session_state().get_alice_base_key().is_some());

    let bob_outgoing_message = session::encrypt_message(&mut bob_store, &alice_address, &original_message).unwrap();
    assert_eq!(bob_outgoing_message.get_type(), MessageType::WHISPER);
    let bob_outgoing_message = bob_outgoing_message.as_any().downcast_ref::<SignalMessage>().unwrap();

    let alice_plain_text = session::decrypt_whisper_message(&mut alice_store, &bob_address, &bob_outgoing_message).unwrap();
    assert_eq!(alice_plain_text, original_message);
}
