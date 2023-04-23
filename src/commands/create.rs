use std::{
    fs::{self, File},
    io::{self, ErrorKind},
    os::unix::process::CommandExt,
};

use anyhow::{Ok, Result};
use clap::{Command, Parser, Subcommand};
use colored::Colorize;

use crate::{
    models::key_type::{self, SUPPORTED_KEY_TYPES},
    utils::{is_empty, load_keys::load_keys},
    CHECK_SYMBOL, CROSS_SYMBOL, DEFAULT_KEY, PRIVATE_KEY, PUBLIC_KEY,
};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct CreateOptions {
    /// SSH key alias
    pub alias: String,

    #[clap(short('b'))]
    pub bits: Option<String>,

    #[clap(short('C'))]
    pub comment: Option<String>,

    #[clap(short('t'))]
    /// SSH key type
    pub key_type: Option<String>,
}

impl super::CommandRunner for CreateOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let &SkmCliOptions {
            store_path,
            ssh_path,
            ..
        } = &cli_option;

        let keys = load_keys(cli_option)?;

        for (key, ssh_key) in keys.iter() {
            if key == &self.alias {
                println!(
                    "{}",
                    format!(
                        "{}{}",
                        CROSS_SYMBOL, "SSH key alias already exists, please choose another one!"
                    )
                    .red()
                );
                return Ok(());
            }
        }

        let dir_path = format!("{}/{}", store_path, self.alias);

        // Remove existing empty alias directory if exists
        if let io::Result::Ok(_) = fs::metadata(&dir_path) {
            fs::remove_dir_all(&dir_path)?;
        }

        // Create alias directory
        fs::create_dir(&dir_path)?;

        let key_type = self.key_type.clone().unwrap_or("rsa".to_string());

        let support_key = SUPPORTED_KEY_TYPES.get_key_value(&key_type);

        if support_key.is_none() {
            println!(
                "{}",
                format!("{} is not a supported key type.", &key_type).red()
            );
            return Ok(());
        }
        let (_, ssh_key_setting) = support_key.unwrap();

        let mut args = vec![];
        args.push("-t".to_string());
        args.push(key_type);

        args.push("-f".to_string());
        args.push(format!("{}/{}", dir_path, &ssh_key_setting.key_base_name));

        if ssh_key_setting.support_variable_bit_size && self.bits.is_some() {
            args.push("-b".to_string());
            args.push(self.bits.clone().unwrap());
        }

        if let Some(comment) = self.comment.clone() {
            args.push("-C".to_string());
            args.push(comment);
        }

        std::process::Command::new("ssh-keygen").args(&args).exec();
        // .spawn()
        // .expect("Generate sshkey failed!");

        println!(
            "{}",
            format!("{} SSH key [{}] created!", CHECK_SYMBOL, &self.alias).green()
        );

        Ok(())
    }
}
