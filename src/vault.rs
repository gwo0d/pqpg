use crate::identity::Identity;
use crate::signing_key::SigningKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Key {
    Signing(SigningKey),
}

#[derive(Serialize, Deserialize)]
pub struct Vault {
    primary_identity: Identity,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_identities: Option<Vec<Identity>>,
    vault_keys: Vec<Key>,
    external_vaults: Vec<Self>,
}

impl Vault {
    pub fn new(
        primary_identity: Identity,
        secondary_identities: Option<Vec<Identity>>,
        key_types: Vec<Key>,
    ) -> Self {
        let mut vault_keys: Vec<Key> = Vec::new();

        for key_type in key_types {
            match key_type {
                Key::Signing(key) => vault_keys.push(Key::Signing(key)),
            }
        }

        let external_vaults: Vec<Self> = Vec::new();

        Self {
            primary_identity,
            secondary_identities,
            vault_keys,
            external_vaults,
        }
    }

    pub fn get_secret_keys(&self) -> &Vec<Key> {
        &self.vault_keys
    }

    pub fn get_public_keys(&self) -> Vec<Key> {
        let mut public_keys: Vec<Key> = Vec::new();
        for key in self.vault_keys.iter() {
            match key {
                Key::Signing(key) => public_keys.push(Key::Signing(key.get_redacted_key())),
            }
        }

        public_keys
    }

    pub fn get_external_vaults(&self) -> &Vec<Self> {
        &self.external_vaults
    }

    pub fn get_primary_identity(&self) -> &Identity {
        &self.primary_identity
    }

    pub fn get_secondary_identities(&self) -> &Option<Vec<Identity>> {
        &self.secondary_identities
    }
}
