use std::{fs, io::stdin};

use anyhow::{Ok, Result};
use clap::Parser;
use colored::Colorize;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::{utils::load_keys::load_keys, CHECK_SYMBOL};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct DeleteOptions {}

impl super::CommandRunner for DeleteOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let &SkmCliOptions {
            store_path,
            ssh_path,
            ..
        } = &cli_option;

        let keys = load_keys(cli_option)?;

        if keys.len() < 2 {
            println!("{}", "Cannot delete any one!".blue());
            return Ok(());
        }

        let alias = {
            let values = keys.values().collect::<Vec<_>>();
            let index = values.partition_point(|x| x.is_default);

            let keys = keys
                .into_iter()
                .filter_map(
                    |(s, ssh_key)| {
                        if ssh_key.is_default {
                            None
                        } else {
                            Some(s)
                        }
                    },
                )
                .collect::<Vec<_>>();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select one SSH key")
                .items(&keys)
                .interact()
                .unwrap();
            format!("{}", keys[selection])
        };

        let input = {
            println!(
                "{}",
                format!("Please confirm to delete SSH key [{}] [y/n]: ", alias).blue()
            );
            let mut s = "".to_string();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            s
        };

        if &input != "y" {
            return Ok(());
        }

        format!("{}/{}", store_path, alias);
        fs::remove_dir_all(format!("{}/{}", store_path, alias))?;

        println!(
            "{}",
            format!("{} SSH key [{}] deleted!", CHECK_SYMBOL, alias).green()
        );
        Ok(())
    }
}
