use std::{fs, os::unix::prelude::MetadataExt, path::Path};

use anyhow::{Error, Ok, Result};
use colored::Colorize;

use crate::{
    commands::SkmCliOptions,
    i_error::SkmError,
    models::key_type::{get_by_filename, KeyType, SSHKey},
};

pub fn load_single_key(key_path: String, ssh_path: String) -> Result<SSHKey> {
    let mut key = SSHKey::default();

    if let std::io::Result::Ok(entries) = fs::read_dir(key_path) {
        let entries = entries.collect::<Vec<_>>();
        if entries.len() == 0 {
            return Err(SkmError::OtherError.into());
        }
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }

            if path.to_string_lossy().contains(".pub") {
                if let Some(name) = path.to_str() {
                    key.public_key = name.to_string();
                }
                continue;
            }

            let (kt, ok) = if let Some(name) = path.file_name() {
                get_by_filename(format!("{}", name.to_string_lossy()))
            } else {
                (KeyType::default(), true)
            };
            if !ok {
                continue;
            }
            let key_base_name = kt.key_base_name.clone();
            key.key_type = kt;
            if let Some(name) = path.to_str() {
                key.private_key = name.to_string();
            }

            let parsed_path = parsed_path(format!("{}/{}", ssh_path, key_base_name.clone()))?;

            if parsed_path == path.to_str().unwrap() {
                key.is_default = true
            }
        }
    };
    Ok(key)
}

pub fn parsed_path(path: String) -> Result<String> {
    let file_path = Path::new(&path);
    let info = fs::symlink_metadata(&path)?;

    if info.is_symlink() {
        let origin_file = fs::read_link(&path)?;
        if origin_file.is_absolute() {
            return Ok(format!("{}", origin_file.as_path().to_str().unwrap()));
        } else {
            return Ok(format!(
                "{}{}",
                file_path.canonicalize()?.to_str().unwrap(),
                origin_file.to_str().unwrap()
            ));
        }
    }

    return Ok(path);
}
