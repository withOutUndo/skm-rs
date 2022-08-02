use clap::Parser;
use colored::Colorize;
use commands::{Command, SkmCliOptions};

use crate::commands::CommandRunner;

mod commands;

fn main() {
    let skm_cli_option = SkmCliOptions::parse();

    println!("skm_cli_options, {:#?}", skm_cli_option);

    let command: Command = skm_cli_option.command;

    match command {
        Command::Init(init_option) => {
            init_option.execute();
            println!("Init, {:#?}", init_option);
        }
        Command::Create => todo!(),
        Command::Ls => todo!(),
        Command::Use => todo!(),
        Command::Delete => todo!(),
        Command::Rename => todo!(),
        Command::Copy => todo!(),
        Command::Display => todo!(),
        Command::Backup => todo!(),
        Command::Restore => todo!(),
        Command::Cache => todo!(),
        Command::Help => todo!(),
    }
}
