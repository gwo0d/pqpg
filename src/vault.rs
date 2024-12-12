use crate::identity::Identity;
use crate::signing_key::SigningKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Key {
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
        first_name: String,
        last_name: String,
        email: String,
        comment: Option<String>,
        key_types: Vec<char>,
    ) -> Self {
        let primary_identity = Identity::new(first_name, last_name, email, comment);

        let mut vault_keys: Vec<Key> = Vec::new();
        for key_type in key_types {
            match key_type {
                's' => {
                    let signing_key = SigningKey::new();
                    vault_keys.push(Key::Signing(signing_key));
                }
                _ => {}
            }
        }

        Self {
            primary_identity,
            secondary_identities: None,
            vault_keys,
            external_vaults: Vec::new(),
        }
    }

    fn ensure_secondary_identities(&mut self) -> &mut Vec<Identity> {
        if self.secondary_identities.is_none() {
            self.secondary_identities = Some(Vec::new());
        }
        self.secondary_identities.as_mut().unwrap()
    }

    pub fn add_or_initialise_secondary_identity(&mut self, identity: Identity) {
        self.ensure_secondary_identities().push(identity);
    }

    pub fn export_with_secrets(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    // TODO: Implement export_without_secrets method.
}
