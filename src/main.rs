use std::{collections::HashSet, ffi::{OsString}, path::{PathBuf}, fmt::Display};

use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, Confirm};
use glob::{glob};

pub mod converters;
pub mod file_types;

use crate::{converters::{get_converters, Converter, run_converter}, file_types::{get_file_type, get_file_types_flat}};

fn select<T: Display>(prompt: &str, options: &Vec<T>) -> usize {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
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
		match file.extension() {
			Some(ext) => extensions.insert(OsString::from(ext).into_string().expect("Failed conversion")),
			None => extensions.insert("".to_string())
		};

	}

	if extensions.len() == 1 {
		let rv = extensions.iter().next().unwrap().clone();
		println!("RV: {}", rv);
		return Some(rv);
	}
	return None;
}

fn get_filetype(files: &Vec<PathBuf>) -> String {
	let mut file_type = String::new();

	match get_extension(&files) {
		Some(ext) => 'autoext: {
			let file_types = get_file_type(&ext);
			if (file_types.len() == 0) {
				println!("Could not automatically find extension.");
				break 'autoext
			};

			if (file_types.len() >= 2) {
				println!("Multiple extensions found. Requires manual input.");
				break 'autoext
			};

				let confirmed = Confirm::new()
					.with_prompt(format!("Detected filetype '{}'. Is this correct?", file_types[0]))
					.interact()
					.unwrap();
				if (confirmed == true) {
					return file_types[0].to_string();
				}
		},
		None => {
			println!("Extension not specified.");
		}
	};

	let types = get_file_types_flat();

	let type_idx = select("Select filetype", &types);
	
	return types[type_idx].clone();
}

fn main() {
	let converters = get_converters();
	let target_glob = basic_prompt("[GLOB] Select target files: ");
	let files = get_target_files(target_glob);

	println!("{}",format!("Found {} files.", files.len()));
	let file_type = get_filetype(&files);

	let mut selected_converter_tmp: Option<Converter> = None;

	for converter in converters {
		if converter.convert_from[&file_type].is_string() {
			selected_converter_tmp = Some(converter);
			break;
		}
	}

	let selected_converter = selected_converter_tmp.unwrap();

	println!("Converter: {}", selected_converter.name);

	let selections: Vec<&String> = selected_converter
		.convert_to.as_object()
		.unwrap()
		.iter()
		.map(|v| v.0)
		.collect()	
		;

	let target_output = selections[select("Convert to:",&selections)];

	for file in &files {
		run_converter(&selected_converter, 
			file.as_path().to_str().unwrap(), 
			&(String::new() + file.file_stem().unwrap().clone().to_str().expect("") + "." + &target_output),
			&file_type,
			&target_output)
	}


}