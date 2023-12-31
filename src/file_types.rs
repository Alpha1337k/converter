
use serde_json::{Value};

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

	let mut v = Vec::new();


	unsafe {
		if TYPES == Value::Null {
			TYPES = load_file_types();
		}
	
	if TYPES[ext].is_string() {

		v.push(TYPES[ext].as_str().unwrap());

		return v;
	} else {

		if (TYPES[ext].is_array()) {
			v = TYPES[ext]
				.as_array()
				.unwrap()
				.into_iter()
				.map(|f: &Value| f.as_str().expect("I"))
				.collect();
		}

		return v;
		}
	}

}

pub fn get_file_types_flat() -> Vec<String>
{
	static mut TYPES: Value = Value::Null;


	unsafe {
		if TYPES == Value::Null {
			TYPES = load_file_types();
		}
	let mut types: Vec<String> = Vec::new();

	for (key, value) in TYPES.as_object().unwrap() {
		if (value.is_string()) {
			types.push(String::from(value.as_str().unwrap()));
		} else if (value.is_array()) {
			for s in value.as_array().unwrap() {
				types.push(String::from(s.as_str().unwrap()));
			}
		}
	}

	return types;
	}
}