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

use thiserror::Error;

use crate::constants::{CONFIG_DIR, CONVERTERS_DIR};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConverterError {
    #[error("'{program_name}' was not found in path.")]
    ConverterNotFound { program_name: String },

    #[error("Directory '{directory}' does not exist")]
    DirectoryNotFound { directory: PathBuf },

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

    let mut root = dirs::home_dir()
        .ok_or("Failed to load home directory")?
        .join(CONFIG_DIR)
        .join(CONVERTERS_DIR);

    if !fs::exists(&root)? {
        return Err(ConverterError::DirectoryNotFound { directory: root }.into());
    }

    root = root.join("*.json");
    let converter_dir = root.to_str().ok_or("Failed to convert PathBuf to String")?;

    debug!("Loading all manifests using pattern '{}'", &converter_dir);

    let iter = match glob(converter_dir) {
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

    if converters.is_empty() {
        Err(ConverterError::NoConvertersLoaded.into())
    } else {
        Ok(converters)
    }
}

pub fn find_converter<'a>(
    converters: &'a [Converter],
    input_extension: &str,
    output_extension: &str,
) -> Option<&'a Converter> {
    converters.iter().find(|&converter| {
        converter
            .convert_from
            .get(input_extension)
            .is_some_and(|_| converter.convert_to.contains_key(output_extension))
    })
}
