use std::{collections::HashSet, ffi::{OsStr, OsString}, arch::x86_64, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use glob::{glob, Paths};

pub mod converters;
pub mod file_types;

use crate::{converters::{get_converters, Converter, run_converter}, file_types::get_file_type};

fn select(options: &Vec<&String>) -> usize {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Convert to: ")
		.max_length(5_usize)
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

	return selection;
}

fn basic_prompt(query: &str) -> String {
	return Input::with_theme(&ColorfulTheme::default())
        .with_prompt(query)
        .interact_text()
        .unwrap();
}

fn get_target_files(raw_glob: String) -> Vec<PathBuf> {
	let files: Vec<PathBuf> = glob(&raw_glob)
		.expect("Error: invalid glob pattern.")
		.filter_map(Result::ok)
		.collect()
		;

	return files
}

fn get_extension(files: &Vec<PathBuf>) -> Option<String> {
	let mut extensions = HashSet::new();

	for file in files {
		let ext = file.extension().unwrap();
		extensions.insert(OsString::from(ext).into_string().expect("Failed conversion"));
		// match file {
		// 	Ok(f) => {

		// 	}
		// 	Err(err) => {
		// 		println!("Failed to get extension: {}", err);
		// 	}
		// }
	}

	if (extensions.len() == 1) {
		let rv = extensions.iter().next().unwrap().clone();
		println!("RV: {}", rv);
		return Some(rv);
	}
	return Some(String::new() + "AA");
}

fn main() {
	let converters = get_converters();


	let target_glob = basic_prompt("[GLOB] Select target files: ");

	let mut files = get_target_files(target_glob);

	let extension = get_extension(&files)
		.unwrap_or_else(|| basic_prompt("Could not automatically find extension. Enter extension: ")
		).to_string();

	let file_types = get_file_type(&extension);

	println!("Extension used is {} ({}).", extension, file_types[0]);

	let file_type = file_types[0];

	for file in &files {
		println!("{}", file.display());
	}

	let mut selectedConverterTmp: Option<Converter> = None;

	for converter in converters {
		if (converter.convert_from[&file_type].is_string()) {
			selectedConverterTmp = Some(converter);
			break;
		}
	}

	let selected_converter = selectedConverterTmp.unwrap();

	println!("Converter: {}", selected_converter.name);

	let selections: Vec<&String> = selected_converter
		.convert_to.as_object()
		.unwrap()
		.iter()
		.map(|v| v.0)
		.collect()	
		;

	let target_output = selections[select(&selections)];

	for file in &files {
		run_converter(&selected_converter, 
			file.as_path().to_str().expect("AA"), 
			 &(String::new() + file.file_stem().expect("A").clone().to_str().expect("") + "." + &target_output),
			file_type,
			&target_output)
	}


}