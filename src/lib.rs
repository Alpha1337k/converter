use std::{fmt::Display, process::exit};

use log::log_enabled;
use thiserror::Error;

pub mod constants;
pub mod converters;
pub mod converting;
pub mod extension;
pub mod file_types;
pub mod prompts;
pub mod setup;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ArgError {
    #[error("Not enough arguments provided: {len}. Need at least two.")]
    InvalidArgumentCount { len: usize },

    #[error("Unrecognized file extension for '{file}'.")]
    UnrecognizedExtension { file: std::path::PathBuf },

    #[error("Invalid filename.")]
    InvalidFileName,
}

pub fn error_exit<T: Display>(e: T, extra_message: Option<&str>) -> ! {
    eprintln!("Error: {}{}", extra_message.unwrap_or_default(), e);
    if !log_enabled!(log::Level::Error) {
        eprintln!("Hint: Try running the command again with -v for further debugging");
    }
    exit(1)
}
