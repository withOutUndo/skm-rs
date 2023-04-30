use std::{
    fs::{self, File},
    io::{self, ErrorKind},
    os::unix::process::CommandExt,
};

use anyhow::{Ok, Result};
use clap::{Command, Parser, Subcommand};
use colored::Colorize;
use lazy_static::__Deref;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    i_error::SkmError,
    models::key_type::{self, SSHKey, SUPPORTED_KEY_TYPES},
    utils::{is_empty, load_keys::load_keys},
    CHECK_SYMBOL, CROSS_SYMBOL, DEFAULT_KEY, PRIVATE_KEY, PUBLIC_KEY,
};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct SetOptions {
    /// SSH key alias
    pub alias: Option<String>,
}

impl super::CommandRunner for SetOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let &SkmCliOptions {
            store_path,
            ssh_path,
            ..
        } = &cli_option;

        let keys = load_keys(cli_option)?;

        let alias = if let Some(als) = &self.alias {
            (*als).to_string()
        } else {
            let options = keys.iter();
            let keys = options.clone().map(|(s, _)| s).collect::<Vec<_>>();
            let index = options
                .enumerate()
                .find_map(|(i, (_, key))| if key.is_default { Some(i) } else { None })
                .unwrap_or(0);
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select one SSH key")
                .items(&keys)
                .default(index)
                .interact()
                .unwrap();
            format!("{}", keys[selection])
        };

        if keys.get(&alias).is_none() {
            return Err(SkmError::OtherError.into());
        }
        let key = keys.get(&alias).unwrap();

        let store_private_key_path =
            format!("{}/{}/{}", store_path, alias, key.key_type.private_key());
        let store_public_key_path =
            format!("{}/{}/{}", store_path, alias, key.key_type.public_key());
        let link_private_key_path = format!("{}/{}", ssh_path, key.key_type.private_key());
        let link_public_key_path = format!("{}/{}", ssh_path, key.key_type.public_key());

        fs::remove_file(&link_private_key_path)?;
        fs::remove_file(&link_public_key_path)?;

        fs::soft_link(store_private_key_path, link_private_key_path)?;
        fs::soft_link(store_public_key_path, link_public_key_path)?;

        println!("{}", format!("Now using SSH key: {}", alias).green());
        Ok(())
    }
}
