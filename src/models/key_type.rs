use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct KeyType {
    pub name: String,
    pub key_base_name: String,
    pub support_variable_bit_size: bool,
}

#[derive(Clone, Debug)]
pub struct SSHKey {
    pub public_key: String,
    pub private_key: String,
    pub is_default: bool,
    pub key_type: KeyType,
}

impl Default for SSHKey {
    fn default() -> Self {
        SSHKey {
            public_key: "".to_string(),
            private_key: "".to_string(),
            is_default: false,
            key_type: KeyType::default(),
        }
    }
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType {
            name: "".to_string(),
            key_base_name: "".to_string(),
            support_variable_bit_size: false,
        }
    }
}

impl KeyType {
    pub fn public_key(self: &Self) -> String {
        format!("{}.pub", self.key_base_name)
    }

    pub fn private_key(self: &Self) -> String {
        format!("{}", self.key_base_name)
    }
}

lazy_static! {
    pub static ref SUPPORTED_KEY_TYPES: HashMap<String, KeyType> = [
        (
            "rsa".to_string(),
            KeyType {
                name: "rsa".to_string(),
                support_variable_bit_size: true,
                key_base_name: "id_rsa".to_string(),
            },
        ),
        (
            "ed25519".to_string(),
            KeyType {
                name: "ed25519".to_string(),
                support_variable_bit_size: true,
                key_base_name: "id_ed25519".to_string(),
            },
        )
    ]
    .into_iter()
    .collect();
}

pub fn get_by_filename(name: String) -> (KeyType, bool) {
    for (_, kt) in SUPPORTED_KEY_TYPES.iter() {
        if name == kt.key_base_name {
            return ((*kt).clone(), true);
        }
    }
    (KeyType::default(), false)
}
