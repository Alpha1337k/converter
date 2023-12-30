use std::{fs, process::Command};

use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize)]
pub struct Converter {
	pub name: String,
	pub program_name: String,
	pub args: String,
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

pub fn run_converter(converter: &Converter, input: &str, output: &str, input_type: &str, output_type: &str) {
	let parsed_command = converter.args
		.replace("%INFORM%", input_type)
		.replace("%OUTFORM%", output_type)
		.replace("%OUTFILE%", output)
		.replace("%INFILE%", input);

	println!("> {} {}", converter.program_name, parsed_command);

	let result = Command::new(&converter.program_name)
		.args(parsed_command.split(" "))
		.output()
		.expect("failed to run program.");
	
	if (result.stdout.len() != 0) {
		println!("< {}", String::from_utf8(result.stdout).unwrap());
	}
	if (result.stderr.len() != 0) {
		println!("< {}", String::from_utf8(result.stderr).unwrap());
	}
}