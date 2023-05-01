use std::fs::{self};

use anyhow::{Ok, Result};
use clap::Parser;
use colored::Colorize;

use crate::{utils::load_keys::load_keys, CHECK_SYMBOL};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct ListOptions {}

impl super::CommandRunner for ListOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let keys = load_keys(cli_option)?;

        if keys.len() == 0 {
            println!("{}", format!("{} No SSH key found!", CHECK_SYMBOL).green());
            return Ok(());
        };

        println!(
            "{}",
            format!("\r\n{} Found {} SSH key(s)!\n", CHECK_SYMBOL, keys.len()).green()
        );

        for (key, ssh_key) in keys.iter() {
            let mut key_desc = "";
            let mut key_type = "";
            let key_str = fs::read_to_string(&ssh_key.public_key)?
                .splitn(3, " ")
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            if key_str.len() >= 3 {
                key_desc = &key_str[0];
                key_type = &key_str[2];
            }

            let key_type = format!("[{}]", key_type.replace("\n", ""));

            if ssh_key.is_default {
                println!(
                    "{}",
                    format!("->\t{:20}{:<20} \t[{}]", key, key_type, key_desc).green()
                );
            } else {
                println!(
                    "{}",
                    format!(" \t{:20}{:<20} \t[{}]", key, key_type, key_desc).blue()
                );
            }
        }
        Ok(())
    }
}
