use std::fs;

use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
pub struct Converter {
	pub name: String,
	pub command: String,
	pub convert_from: serde_json::Value,
	pub convert_to: serde_json::Value,
}

pub fn get_converters() -> Vec<Converter>
{
	let mut converters = Vec::with_capacity(0);

	for item in glob("converters/*.json").expect("Error: No converter manifests found.") {
		match item {
			Ok(v) => {
				let file = fs::read_to_string(v.as_os_str()).expect("Error: Failed to open converter file.");
				let converter: Converter = serde_json::from_str(&file).expect("Failed to parse converter.");

				converters.push(converter);
			}
			Err(err) => {
				println!("{:?}", err)
			}
		}
	}

	return converters;
}