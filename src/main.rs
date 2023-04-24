use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use commands::{Command, SkmCliOptions};

use crate::commands::CommandRunner;

#[macro_use]
extern crate lazy_static;


mod commands;

mod utils;

mod i_error;

mod models;

/// CheckSymbol is the code for check symbol
const CHECK_SYMBOL: &str = "\u{2714}";
/// CrossSymbol is the code for check symbol
const CROSS_SYMBOL: &str = "\u{2716}";

/// PublicKey is the default name of SSH public key
const PUBLIC_KEY: &str = "id_rsa.pub";
/// PrivateKey is the default name of SSH private key
const PRIVATE_KEY: &str = "id_rsa";
/// DefaultKey is the default alias name of SSH key
const DEFAULT_KEY: &str = "default";

fn main() -> Result<()> {
    let skm_cli_option = SkmCliOptions::parse();

    let command: &Command = &skm_cli_option.command;

    match command {
        Command::Init(init_option) => {
            init_option.execute(&skm_cli_option)
        },
        Command::Create(create_options) => {
            create_options.execute(&skm_cli_option)
        },
        Command::Ls(list_option) => {
            list_option.execute(&skm_cli_option)
        },
        Command::Use => todo!(),
        Command::Delete => todo!(),
        Command::Rename => todo!(),
        Command::Copy => todo!(),
        Command::Display(option) => {
            option.execute(&skm_cli_option)
        },
        Command::Backup => todo!(),
        Command::Restore => todo!(),
        Command::Cache => todo!(),
        Command::Help => todo!(),
    }
}
