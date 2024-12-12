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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_signature_success() {
        let signing_key = SigningKey::new();
        let message = b"test message";
        let signature = signing_key.create_signature(message).unwrap();
        assert!(!signature.is_empty());
    }

    #[test]
    fn create_signature_no_secret_key() {
        let mut signing_key = SigningKey::new();
        signing_key.sk = None;
        let message = b"test message";
        let result = signing_key.create_signature(message);
        assert_eq!(result, Err("No secret key available."));
    }

    #[test]
    fn verify_signature_success() {
        let signing_key = SigningKey::new();
        let message = b"test message";
        let signature = signing_key.create_signature(message).unwrap();
        assert!(signing_key.verify_signature(message, signature));
    }

    #[test]
    fn verify_signature_failure() {
        let signing_key = SigningKey::new();
        let message = b"test message";
        let invalid_signature = "invalidsignature";
        assert!(!signing_key.verify_signature(message, invalid_signature.to_string()));
    }

    #[test]
    fn get_public_key_success() {
        let signing_key = SigningKey::new();
        let public_key = signing_key.get_public_key();
        assert!(!public_key.is_empty());
    }

    #[test]
    fn get_secret_key_success() {
        let signing_key = SigningKey::new();
        let secret_key = signing_key.get_secret_key().unwrap();
        assert!(!secret_key.is_empty());
    }

    #[test]
    fn get_secret_key_none() {
        let mut signing_key = SigningKey::new();
        signing_key.sk = None;
        assert!(signing_key.get_secret_key().is_none());
    }
}
