use std::{
    fs::{self, File},
    io::{self, ErrorKind},
};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use crate::{utils::is_empty, CHECK_SYMBOL, PUBLIC_KEY, PRIVATE_KEY};

use super::SkmCliOptions;

#[derive(Parser, Debug)]
pub struct InitOptions {}

impl super::CommandRunner for InitOptions {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()> {
        let &SkmCliOptions {
            store_path,
            ssh_path,
            ..
        } = &cli_option;

        is_empty(store_path)?;

        fs::create_dir(store_path)?;

        fs::rename(&format!("{}/{}", ssh_path, PRIVATE_KEY), &format!("{}/{}", store_path, PRIVATE_KEY))?;
        fs::rename(&format!("{}/{}", ssh_path, PUBLIC_KEY), &format!("{}/{}", store_path, PUBLIC_KEY))?;

        println!(
            "{}",
            format!("{} {}", CHECK_SYMBOL, "SSH key store initialized!").green()
        );
        println!("Key store location is: {}", store_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{any, f32::consts::E};

    use super::*;
    use crate::{
        commands::{initialize, Command, CommandRunner},
        i_error::SkmError,
    };
    use anyhow::Error;

    #[test]
    fn its_exist() {
        let initOptions = InitOptions {};
        let options = SkmCliOptions {
            command: Command::Init(initOptions),
            store_path: "/home/iboom/workspace/Rust/skm-rs/test-dir".into(),
            ssh_path: "ss".into(),
        };

        match &options.command {
            Command::Init(init) => {
                let a = init.execute(&options);
                match a.err().unwrap().downcast_ref::<crate::i_error::SkmError>() {
                    Some(&SkmError::ExistError) => {
                        assert!(true)
                    }
                    _ => assert!(false),
                }
            }
            _ => {
                assert!(false)
            }
        }
    }
}
