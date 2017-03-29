use eth;
use rand::{OsRng, Rng};
use signal::keys::IdentityKeyPair;
use signal::protocol::{SignalProtocolStore};
use signal::state::{PreKeyRecord,SignedPreKeyRecord};
use service;

mod TokenAccount;
use protobuf::Message;

pub struct Account {
    username: String,
    private_key: eth::SecretKey,
    token_id: eth::Address,

    // signal specific
    registration_id: u32,
    password: String,
    identity_keypair: IdentityKeyPair,
    device_id: u32,
    signaling_key: [u8;52]
}

pub trait AccountStore {
    fn load_account(&self, username: &str) -> Option<Account>;
    fn store_account(&mut self, account: &Account);
    fn contains_account(&self, username: &str) -> bool;
    fn remove_account(&mut self, username: &str);
}

impl Account {
    pub fn new(username: &str) -> Account {

        let mut rng = OsRng::new().ok().unwrap();

        // generate token_id
        let private_key = eth::generate_secret_key();

        // signal details
        // generate random password
        let password: String = rng.gen_iter::<char>()
            .take(16)
            .collect();
        // id key pair
        let id_keypair = IdentityKeyPair::generate();
        // registration id
        let registration_id = rng.gen_range(1, 16381);
        // signaling key
        let mut signaling_key = [0u8; 52];
        rng.fill_bytes(&mut signaling_key);

        Account {
            username: username.to_string(),
            token_id: private_key.address(),
            private_key: private_key,

            registration_id: registration_id,
            password: password,
            identity_keypair: id_keypair,
            device_id: 1,
            signaling_key: signaling_key
        }
    }

    pub fn initialize(&self, store: &mut SignalProtocolStore, id_service_url: &str, chat_service_url: &str) -> Result<(), String> {

        match service::id::IdService::new(id_service_url, &self.private_key)
            .create_user(&self.username, &self.token_id) {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Unable to create user: {:?}", e));
                }
            };

        // generate keys for new user
        let prekeys = PreKeyRecord::generate_prekeys(0, 100);
        // TODO: this seems to be what the java client always sets, figure out why
        let last_resort_key = PreKeyRecord::generate(16777215);
        let signed_prekey_record = SignedPreKeyRecord::generate(0, &self.identity_keypair);
        match service::chat::ChatService::new(
            store, chat_service_url,
            &self.private_key, &self.token_id, &self.password)
            .bootstrap_account(&self.identity_keypair,
                               &last_resort_key,
                               &prekeys,
                               &signed_prekey_record,
                               self.registration_id,
                               &self.signaling_key) {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Unable to create user: {:?}", e));
                }
            };

        for key in prekeys {
            store.store_prekey(key.get_id(), &key);
        }
        store.store_prekey(last_resort_key.get_id(), &last_resort_key);
        store.store_signed_prekey(signed_prekey_record.get_id(), &signed_prekey_record);

        Ok(())
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_identity_keypair(&self) -> &IdentityKeyPair {
        &self.identity_keypair
    }

    pub fn get_registration_id(&self) -> u32 {
        self.registration_id
    }

    pub fn get_private_key(&self) -> &eth::SecretKey {
        &self.private_key
    }

    pub fn get_token_id(&self) -> &eth::Address {
        &self.token_id
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn get_signaling_key(&self) -> &[u8;52] {
        &self.signaling_key
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = TokenAccount::Account::new();
        buf.set_username(self.username.clone());
        buf.set_private_key(self.private_key.serialize());
        buf.set_registration_id(self.registration_id);
        buf.set_password(self.password.clone());
        buf.set_identity_keypair(self.identity_keypair.serialize());
        buf.set_device_id(self.device_id);
        buf.set_signaling_key(self.signaling_key.to_vec());
        buf.write_to_bytes().unwrap()
    }

    pub fn deserialize(serialized: &Vec<u8>) -> Account {
        let mut buf = TokenAccount::Account::new();
        buf.merge_from_bytes(serialized).unwrap();
        let pk = eth::SecretKey::deserialize(&buf.take_private_key());
        let mut signaling_key = [0u8; 52];
        signaling_key.copy_from_slice(buf.get_signaling_key());
        Account {
            username: buf.take_username(),
            token_id: pk.address(),
            private_key: pk,

            registration_id: buf.get_registration_id(),
            password: buf.take_password(),
            identity_keypair: IdentityKeyPair::deserialize(&buf.take_identity_keypair()),
            device_id: buf.get_device_id(),
            signaling_key: signaling_key
        }
    }
}
