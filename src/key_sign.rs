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

    pub fn new_from_public_key(public_key: String) -> Result<Self, &'static str> {
        let decoded_pk = BASE64_STANDARD
            .decode(public_key)
            .map_err(|_| "Invalid base64 encoding.")?;
        let pk = PublicKey::from_bytes(&decoded_pk).map_err(|_| "Invalid public key.")?;
        Ok(Self { pk, sk: None })
    }

    pub fn create_signature(&self, message: &[u8]) -> Result<String, &'static str> {
        self.sk
            .as_ref()
            .map(|sk| BASE64_STANDARD.encode(detached_sign(message, sk).as_bytes()))
            .ok_or("No secret key available.")
    }

    pub fn verify_signature(&self, message: &[u8], signature: String) -> bool {
        if let Ok(decoded_sig) = BASE64_STANDARD.decode(signature) {
            if let Ok(sig) = DetachedSignature::from_bytes(&decoded_sig) {
                return verify_detached_signature(&sig, message, &self.pk).is_ok();
            }
        }
        false
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
    use pqcrypto::sign::sphincsshake256ssimple::keypair;

    #[test]
    fn new_creates_valid_keypair() {
        let signing_key = SigningKey::new();
        assert!(signing_key.sk.is_some());
        assert!(!signing_key.get_public_key().is_empty());
    }

    #[test]
    fn new_from_public_key_creates_instance_with_only_public_key() {
        let (pk, _) = keypair();
        let public_key = BASE64_STANDARD.encode(pk.as_bytes());
        let signing_key = SigningKey::new_from_public_key(public_key).unwrap();
        assert!(signing_key.sk.is_none());
        assert!(!signing_key.get_public_key().is_empty());
    }

    #[test]
    fn new_from_public_key_returns_error_for_invalid_key() {
        let result = SigningKey::new_from_public_key("invalid_key".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn create_signature_returns_signature_when_secret_key_is_present() {
        let signing_key = SigningKey::new();
        let message = b"test message";
        let signature = signing_key.create_signature(message).unwrap();
        assert!(!signature.is_empty());
    }

    #[test]
    fn create_signature_returns_error_when_secret_key_is_absent() {
        let (pk, _) = keypair();
        let public_key = BASE64_STANDARD.encode(pk.as_bytes());
        let signing_key = SigningKey::new_from_public_key(public_key).unwrap();
        let message = b"test message";
        let result = signing_key.create_signature(message);
        assert!(result.is_err());
    }

    #[test]
    fn verify_signature_returns_true_for_valid_signature() {
        let signing_key = SigningKey::new();
        let message = b"test message";
        let signature = signing_key.create_signature(message).unwrap();
        assert!(signing_key.verify_signature(message, signature));
    }

    #[test]
    fn verify_signature_returns_false_for_invalid_signature() {
        let signing_key = SigningKey::new();
        let message = b"test message";
        let invalid_signature = "invalid_signature".to_string();
        assert!(!signing_key.verify_signature(message, invalid_signature));
    }

    #[test]
    fn get_public_key_returns_non_empty_string() {
        let signing_key = SigningKey::new();
        assert!(!signing_key.get_public_key().is_empty());
    }

    #[test]
    fn get_secret_key_returns_some_when_secret_key_is_present() {
        let signing_key = SigningKey::new();
        assert!(signing_key.get_secret_key().is_some());
    }

    #[test]
    fn get_secret_key_returns_none_when_secret_key_is_absent() {
        let (pk, _) = keypair();
        let public_key = BASE64_STANDARD.encode(pk.as_bytes());
        let signing_key = SigningKey::new_from_public_key(public_key).unwrap();
        assert!(signing_key.get_secret_key().is_none());
    }
}
