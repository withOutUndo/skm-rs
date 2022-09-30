use thiserror::Error;
use colored::Colorize;
use crate::CROSS_SYMBOL;

#[derive(Debug, Error)]
pub enum SkmError {
    #[error("kind")]
    IoError(#[from] std::io::Error),

    #[error("{} {}", CROSS_SYMBOL.red(),"SSH key store already exists.".red())]
    ExistError,
}