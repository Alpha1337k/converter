use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
    io::{self},
    path::PathBuf,
};

use glob::glob;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use which::which;

use crate::constants::CONVERTER_CONFIG_DIR;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConverterError {
    #[error("'{program_name}' was not found in path.")]
    ConverterNotFound { program_name: String },

    #[error("Directory '{directory}' does not exist")]
    DirectoryNotFound { directory: String },

    #[error("No conversion manifests were found or valid.")]
    NoConvertersLoaded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Converter {
    pub name: String,
    pub program_name: String,
    pub args: String,
    pub convert_from: HashMap<String, String>,
    pub convert_to: HashMap<String, String>,
}

impl Converter {
    pub fn validate_program_existence(&self) -> Result<PathBuf, ConverterError> {
        which(&self.program_name).map_err(|_| ConverterError::ConverterNotFound {
            program_name: self.program_name.clone(),
        })
    }
}

pub fn get_converters() -> Result<Vec<Converter>, Box<dyn Error>> {
    let mut converters = Vec::new();

    let root = format!("{CONVERTER_CONFIG_DIR}/converters");

    if fs::exists(&root)? == false {
        return Err(ConverterError::DirectoryNotFound { directory: root }.into());
    }

    let converter_dir = format!("{CONVERTER_CONFIG_DIR}/converters/*.json");

    debug!("Loading all manifests using pattern '{}'", &converter_dir);

    let iter = match glob(&converter_dir) {
        Ok(i) => i,
        Err(e) => return Err(e.into()),
    };

    for entry in iter.flatten() {
        match fs::read_to_string(&entry).and_then(|file| {
            serde_json::from_str::<Converter>(&file)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }) {
            Ok(converter) => {
                if let Err(e) = converter.validate_program_existence() {
                    warn!("Failed to load {:?}: {}", entry, e);
                } else {
                    converters.push(converter)
                }
            }
            Err(e) => eprintln!("Failed to load {:?}: {}", entry, e),
        }
    }

    if converters.len() == 0 {
        Err(ConverterError::NoConvertersLoaded.into())
    } else {
        Ok(converters)
    }
}

pub fn find_converter<'a>(
    converters: &'a Vec<Converter>,
    input_extension: &str,
    output_extension: &str,
) -> Option<&'a Converter> {
    for converter in converters {
        if converter
            .convert_from
            .get(input_extension)
            .is_some_and(|_| converter.convert_to.contains_key(output_extension))
        {
            return Some(converter);
        }
    }

    None
}
