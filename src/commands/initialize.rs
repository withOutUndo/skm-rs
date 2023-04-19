use std::{
    fs::{self, File},
    io::{self, ErrorKind},
};

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use crate::{utils::is_empty, CHECK_SYMBOL, DEFAULT_KEY, PRIVATE_KEY, PUBLIC_KEY};

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

        fs::create_dir(format!("{}/{}", store_path, DEFAULT_KEY))?;

        let store_private_key_path = format!("{}/{}/{}", store_path, DEFAULT_KEY, PRIVATE_KEY);
        let store_public_key_path = format!("{}/{}/{}", store_path, DEFAULT_KEY, PUBLIC_KEY);
        let link_private_key_path = format!("{}/{}", ssh_path, PRIVATE_KEY);
        let link_public_key_path = format!("{}/{}", ssh_path, PUBLIC_KEY);

        fs::rename(&link_private_key_path, &store_private_key_path)?;
        fs::rename(&link_public_key_path, &store_public_key_path)?;

        // TODO
        fs::soft_link(store_private_key_path, link_private_key_path)?;
        fs::soft_link(store_public_key_path, link_public_key_path)?;

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
            store_path: "./test-dira".into(),
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
