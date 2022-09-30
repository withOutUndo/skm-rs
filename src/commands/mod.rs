use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;

use self::initialize::InitOptions;

pub mod initialize;

lazy_static! {
    static ref HOME: String = {
        if let Ok(e) = env::var("SKM_RS_TEST") {
            e
        } else {
            env!("HOME").to_string()
        }
    };
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct SkmCliOptions {
    #[clap(subcommand)]
    pub command: Command,

    /// Path where SKM should store its profiles
    #[clap(long, value_parser, value_name="STORE_PATH", default_value_t=format!("{}/{}", HOME.to_string(), ".skm-rs"))]
    store_path: String,

    /// Path to a .ssh folder
    #[clap(long, default_value_t=format!("{}/{}", HOME.to_string(), ".ssh"))]
    ssh_path: String,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize SSH keys store for the first time usage.
    Init(InitOptions),
    /// Create a new SSH key.
    Create,
    /// List all the available SSH keys
    Ls,
    /// Set specific SSH key as default by its a lias name
    Use,
    /// Delete specific SSH key by alias name
    Delete,
    /// Rename SSH key alias name to a new one
    Rename,
    /// Copy current SSH public key to a remote host
    Copy,
    /// Display the current SSH public key or specific SSH public key by alias name
    Display,
    /// Backup all SSH keys to an archive file
    Backup,
    /// Restore SSH keys from an existing archive file
    Restore,
    /// Add your SSH to SSH agent cache via alias name
    Cache,
    /// Shows a list of commands or help for one command
    Help,
}

pub trait CommandRunner {
    fn execute(&self, cli_option: &SkmCliOptions) -> Result<()>;
}
