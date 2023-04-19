use std::{collections::HashMap, fs};

use anyhow::Result;

use crate::{commands::SkmCliOptions, models::key_type::SSHKey};

use super::load_single_key::load_single_key;

pub fn load_keys(cli_option: &SkmCliOptions) -> Result<HashMap<String, SSHKey>> {
    let &SkmCliOptions {
        store_path,
        ssh_path,
        ..
    } = &cli_option;
    let mut map = HashMap::new();

    for entry in fs::read_dir(store_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if let Ok(key) = load_single_key(
                entry.path().to_str().unwrap().to_string(),
                format!("{}", ssh_path),
            ) {
                map.insert(format!("{}", entry.file_name().to_string_lossy()), key);
            }
        }
    }
    Ok(map)
}
