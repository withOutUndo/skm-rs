use clap::{Parser, Subcommand};

use self::initialize::InitOptions;

pub mod initialize;

static HOME: &str = env!("HOME");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct SkmCliOptions {
    #[clap(subcommand)]
    pub command: Command,

    /// Path where SKM should store its profiles
    #[clap(long, value_parser, value_name="STORE_PATH", default_value_t=format!("{}/{}", HOME, ".skm"))]
    store_path: String,

    /// Path to a .ssh folder
    #[clap(long, default_value_t=format!("{}/{}", HOME, ".ssh"))]
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
    fn execute(&self) -> ();
}
