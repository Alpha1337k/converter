use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use std::fs;

fn load_file_types() -> Value
{
	let file = fs::read_to_string("filetypes.json").expect("Error: could not load filetypes.json");

	let v: Value = serde_json::from_str(&file).expect("Failed to parse filetypes");

	return v;
}

pub fn get_file_type(ext: &str) -> Vec<&str>
{
	static mut TYPES: Value = Value::Null;

	unsafe {
		if (TYPES == Value::Null) {
			TYPES = load_file_types();
		}
	
	if (TYPES[ext].is_string()) {
		let mut v = Vec::new();

		v.push(TYPES[ext].as_str().unwrap());

		return v;
	} else {
		let v: Vec<&str> = TYPES[ext]
			.as_array()
			.unwrap()
			.into_iter()
			.map(|f: &Value| f.as_str().expect("I"))
			.collect();
		
			return v;
		}
	}

}