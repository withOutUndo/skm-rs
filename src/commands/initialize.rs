use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
pub struct InitOptions {}

impl super::CommandRunner for InitOptions {
    fn execute(&self) {
        println!("{}", env!("HOME").green());
    }
}
