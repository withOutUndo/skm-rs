use std::fs::{self};

use anyhow::{Ok, Result};
use clap::Parser;
use colored::Colorize;

use crate::{CHECK_SYMBOL, CROSS_SYMBOL};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct RenameOptions {
    /// SSH key alias
    alias: String,

    /// new alias
    new_alias: String,
}

impl super::CommandRunner for RenameOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let RenameOptions { alias, new_alias } = self;
        let store_path = &cli_option.store_path;
        if fs::rename(
            format!("{}/{}", store_path, alias),
            format!("{}/{}", store_path, new_alias),
        )
        .is_ok()
        {
            println!(
                "{}",
                format!(
                    "{} SSH key [{}] renamed to [{}]",
                    CHECK_SYMBOL, alias, new_alias
                )
                .green()
            );
        } else {
            println!(
                "{}",
                format!("{} Failed to rename SSH key!", CROSS_SYMBOL).red()
            );
        }
        Ok(())
    }
}
