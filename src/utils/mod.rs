use anyhow::Result;
use std::{
    error::Error,
    fs::{self, File, DirEntry},
    path::Path,
};

use crate::i_error::SkmError;

pub mod load_keys;

pub mod load_single_key;

pub fn is_empty(path: &str) -> Result<bool> {
    match fs::read_dir(path) {
        Ok(dirs) => {
            dirs.for_each(|x| {
                let y: DirEntry = x.unwrap();
                let meta = y.metadata().unwrap();
                
                println!("{:?}, {}", meta.file_type(), meta.is_dir(),);
            });
            Err(SkmError::ExistError.into())
        },
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Ok(true),
            _ => Err(err.into()),
        },
    }
}
