use ::keys::ECPublicKey;

#[derive(Clone)]
pub struct PreKeyBundle {
    registration_id: u32,
    device_id: u32,
    prekey_id: Option<u32>,
    prekey_public: Option<ECPublicKey>,
    signed_prekey_id: u32,
    signed_prekey_public: ECPublicKey,
    signed_prekey_signature: Vec<u8>,
    identity_key: ECPublicKey
}

impl PreKeyBundle {
    pub fn new(registration_id: u32,
               device_id: u32,
               prekey_id: Option<u32>,
               prekey_public: Option<&ECPublicKey>,
               signed_prekey_id: u32,
               signed_prekey_public: &ECPublicKey,
               signed_prekey_signature: &Vec<u8>,
               identity_key: &ECPublicKey)
               -> PreKeyBundle {
        PreKeyBundle {
            registration_id: registration_id,
            device_id: device_id,
            prekey_id: prekey_id,
            prekey_public: match prekey_public {
                Some(key) => Some(key.clone()),
                None => None
            },
            signed_prekey_id: signed_prekey_id,
            signed_prekey_public: signed_prekey_public.clone(),
            signed_prekey_signature: signed_prekey_signature.clone(),
            identity_key: identity_key.clone()
        }
    }

    pub fn get_registration_id(&self) -> u32 {
        self.registration_id
    }

    pub fn get_prekey_id(&self) -> Option<u32> {
        self.prekey_id
    }

    pub fn get_prekey(&self) -> Option<&ECPublicKey> {
        self.prekey_public.as_ref()
    }

    pub fn get_identity_key(&self) -> &ECPublicKey {
        &self.identity_key
    }

    pub fn get_signed_prekey_id(&self) -> u32 {
        self.signed_prekey_id
    }

    pub fn get_signed_prekey(&self) -> &ECPublicKey {
        &self.signed_prekey_public
    }

    pub fn get_signed_prekey_signature(&self) -> &Vec<u8> {
        &self.signed_prekey_signature
    }

    pub fn get_signed_prekey_signature_as_slice(&self) -> [u8;64] {
        // TODO: this seems bad
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&self.signed_prekey_signature);
        arr
    }
}
