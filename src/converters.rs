use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
    io::{self, BufRead, BufReader, Write},
    process::{Command, ExitStatus, Stdio},
    thread::sleep,
    time,
};

use console::style;
use glob::glob;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{CONVERTER_CONFIG_DIR, LOADING_ANIMATION},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Converter {
    pub name: String,
    pub program_name: String,
    pub args: String,
    pub convert_from: HashMap<String, String>,
    pub convert_to: HashMap<String, String>,
}

pub fn get_converters() -> Result<Vec<Converter>, Box<dyn Error>> {
    let mut converters = Vec::new();

    let converter_dir = format!("{CONVERTER_CONFIG_DIR}/converters/*.json");

    let iter = match glob(&converter_dir) {
        Ok(i) => i,
        Err(e) => return Err(e.into()),
    };

    for entry in iter.flatten() {
        match fs::read_to_string(&entry).and_then(|file| {
            serde_json::from_str::<Converter>(&file)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }) {
            Ok(converter) => converters.push(converter),
            Err(e) => eprintln!("Failed to load {:?}: {}", entry, e),
        }
    }

    Ok(converters)
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
