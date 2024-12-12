use base64::prelude::*;
use pqcrypto::sign::sphincsshake256ssimple::*;
use pqcrypto::traits::sign::{
    DetachedSignature, PublicKey as PublicKeyTrait, SecretKey as SecretKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SigningKey {
    pk: PublicKey,
    sk: Option<SecretKey>,
}

impl SigningKey {
    pub fn new() -> Self {
        let (pk, sk) = keypair();
        Self { pk, sk: Some(sk) }
    }

    pub fn create_signature(&self, message: &[u8]) -> Result<String, &'static str> {
        self.sk
            .as_ref()
            .map(|sk| BASE64_STANDARD.encode(detached_sign(message, sk).as_bytes()))
            .ok_or("No secret key available.")
    }

    pub fn verify_signature(&self, message: &[u8], signature: String) -> bool {
        DetachedSignature::from_bytes(&BASE64_STANDARD.decode(signature).unwrap())
            .map(|sig| verify_detached_signature(&sig, message, &self.pk).is_ok())
            .unwrap_or(false)
    }

    pub fn get_public_key(&self) -> String {
        BASE64_STANDARD.encode(self.pk.as_bytes())
    }

    pub fn get_secret_key(&self) -> Option<String> {
        self.sk
            .as_ref()
            .map(|sk| BASE64_STANDARD.encode(sk.as_bytes()))
    }
}
