use anyhow::Ok;
use clap::Parser;

use crate::{i_error::SkmError, utils::load_keys::load_keys};

use super::CommandRunner;

#[derive(Parser, Debug)]
pub struct DisplayOptions {
    alias: Option<String>,
}

impl CommandRunner for DisplayOptions {
    fn execute(&self, cli_option: &super::SkmCliOptions) -> anyhow::Result<()> {
        let keys = load_keys(cli_option)?;
        if let Some(alisa) = &self.alias {
            if let Some(key) = keys.get(alisa) {
                println!("{}", std::fs::read_to_string(&key.public_key).unwrap());
            }
        } else {
            if let Some(key) = keys.iter().find_map(|(_, ssh_key)| {
                if ssh_key.is_default {
                    Some(ssh_key)
                } else {
                    None
                }
            }) {
                println!("{}", std::fs::read_to_string(&key.public_key).unwrap());
            } else {
                return Err(SkmError::OtherError.into());
            }
        }

        Ok(())
    }
}
