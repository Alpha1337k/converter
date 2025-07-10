use log::debug;
use serde_json::{Map, Value};
use thiserror::Error;

use std::{
    error::Error,
    fs::{self},
    io,
};

use crate::constants::CONFIG_DIR;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FileTypeError {
    #[error("Invalid value(s) for {key} found. Please check the filetypes file.")]
    InvalidValueFound { key: String },
}

pub struct FileTypes {
    data: Map<String, Value>,
}

impl FileTypes {
    pub fn load() -> Result<FileTypes, Box<dyn Error>> {
        let path = dirs::home_dir()
            .ok_or("Failed to load home directory")?
            .join(CONFIG_DIR)
            .join("filetypes.json");

        debug!("Loading filetypes from '{:?}'", &path);

        let map = fs::read_to_string(path).and_then(|f| {
            serde_json::from_str::<Map<String, Value>>(&f)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        })?;

        Ok(FileTypes { data: map })
    }

    pub fn get_file_types(&self, ext: String) -> Result<Vec<String>, FileTypeError> {
        let val = match self.data.get(&ext) {
            Some(v) => v,
            None => return Ok(vec![ext.clone()]),
        };

        match val {
            serde_json::Value::String(v) => Ok(vec![v.clone()]),
            serde_json::Value::Array(arr) => {
                let possible_types: Vec<String> = arr
                    .iter()
                    .map_while(|f| f.as_str())
                    .map(|f| f.to_string())
                    .collect();

                if possible_types.len() != arr.len() {
                    return Err(FileTypeError::InvalidValueFound { key: ext });
                }

                Ok(possible_types)
            }
            _ => Err(FileTypeError::InvalidValueFound { key: ext }),
        }
    }
}
