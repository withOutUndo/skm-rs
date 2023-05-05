use std::{os::unix::process::CommandExt, process::Command};

use anyhow::{Ok, Result};
use clap::Parser;
use colored::Colorize;

use crate::{utils::load_single_key::parsed_path, CHECK_SYMBOL, PUBLIC_KEY};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct CopyOptions {
    /// remote host
    host: String,
    /// SSH port
    #[clap(long, short('p'), default_value = "22")]
    port: String,
}

impl super::CommandRunner for CopyOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let CopyOptions { port, host } = self;
        let SkmCliOptions { ssh_path, .. } = &cli_option;
        let key_path = parsed_path(format!("{}/{}", ssh_path, PUBLIC_KEY))?;

        let args: Vec<&str> = vec!["-p", port, "-i", &key_path, host];

        Command::new("ssh-copy-id").args(&args).exec();
        println!(
            "{}",
            format!(
                "{} Current SSH key already copied to remote host",
                CHECK_SYMBOL
            )
            .green()
        );
        Ok(())
    }
}
