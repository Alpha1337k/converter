use std::{collections::HashSet, ffi::OsString, fmt::Display, path::{Path, PathBuf}, process::exit};

use console::{style};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, Confirm, Editor};
use glob::{glob};

mod converters;
mod file_types;
mod prompts;
mod constants;

use crate::{converters::{find_converter, get_converters, run_converter, Converter}, file_types::{get_file_type, get_file_types_flat}, prompts::{basic_prompt, confirm_prompt, select}};
use clap::Parser;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ArgError {
	#[error("Not enough arguments provided: {len}. Need at least two.")]
	InvalidArgumentCount { len: usize },

	#[error("Unrecognized file extension for '{file}'.")]
	UnrecognizedExtension { file: PathBuf },
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RunError {
	#[error("Could not find a possible conversion from {in_ext} to {out_ext}.")]
	NoConversionPossible {in_ext: String, out_ext: String}
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	input_file: PathBuf,
	output_file: PathBuf,

	#[arg(short, long)]
	edit: bool,

	// #[arg(short, long)]
	// yes: bool,

	// #[arg(short, long)]
	// input_type: Option<String>,

	// #[arg(short, long)]
	// output_type: Option<String>,
}

fn get_target_files(raw_glob: &str) -> Vec<PathBuf> {
	let files: Vec<PathBuf> = glob(&raw_glob)
		.expect("Error: invalid glob pattern.")
		.filter_map(Result::ok)
		.collect()
		;

	return files
}

fn get_extensions(files: &Vec<PathBuf>) -> Option<String> {
	let mut extensions = HashSet::new();

	for file in files {
		match file.extension() {
			Some(ext) => extensions.insert(OsString::from(ext).into_string().expect("Failed conversion")),
			None => extensions.insert("".to_string())
		};

	}

	if extensions.len() == 1 {
		let rv = extensions.iter().next().unwrap().clone();
		return Some(rv);
	}
	return None;
}

fn get_filetype(files: &Vec<PathBuf>) -> String {
	let _file_type = String::new();

	match get_extensions(&files) {
		Some(ext) => 'autoext: {
			let file_types = get_file_type(&ext);
			if file_types.len() == 0 {
				println!("Could not automatically find extension.");
				break 'autoext
			};

			let confirmed = confirm_prompt(&format!("Detected filetype '{}'. Is this correct?", file_types[0]));

			if confirmed == true {
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


fn populate_output_type(positionals: &Vec<String>) -> Result<String, ArgError> {
	if positionals.len() < 2 {
		return Err(ArgError::InvalidArgumentCount { len: positionals.len() });
	}
	
	let last = positionals.last().unwrap();

	let extension = match last.split(".").last() {
		Some(s) => Ok(s.to_string()),
		None => Err(ArgError::UnrecognizedExtension { file: last.to_string().into() })
	}?;


	Ok(extension)
}

fn main() {
	let args = Args::parse();

	dbg!(&args);

	let loaded_converters = match get_converters() {
		Ok(v) => v,
		Err(e) => {
			eprintln!("{:?}", e);
			exit(1)
		}
	};

	dbg!(&loaded_converters);

	let input_extension = match args.input_file.extension()
		.and_then(|v| v.to_str()) {
		Some(ext) => ext,
		None => {
			eprintln!("Error: {}", ArgError::UnrecognizedExtension { file: args.input_file });
			std::process::exit(1);
		}
	};

	let output_extension = match args.output_file.extension() 
		.and_then(|v| v.to_str()) {
		Some(ext) => ext,
		None => {
			eprintln!("Error: {}", ArgError::UnrecognizedExtension { file: args.output_file });
			std::process::exit(1);
		}
	};

	let selected_converter = match find_converter( &loaded_converters, input_extension, output_extension) {
		Some(c) => c,
		None => {
			eprintln!("Error: {}", RunError::NoConversionPossible { in_ext: input_extension.to_string(), out_ext: output_extension.to_string() });
			exit(1)
		}
	};

	let mut prompt = selected_converter.args.clone();

	if args.edit {
		prompt = Editor::new().edit(&prompt).unwrap().unwrap();
	}

	run_converter(&selected_converter, 
		&prompt,
		args.input_file.to_str().unwrap(),
		args.output_file.to_str().unwrap(),
		input_extension,
		output_extension);

	// let mut selected_converter_tmp: Option<Converter> = None;

	// for converter in converters {
	// 	if converter.convert_from[&file_type].is_string() {
	// 		selected_converter_tmp = Some(converter);
	// 		break;
	// 	}
	// }

	// let selected_converter = selected_converter_tmp.unwrap_or_else(|| {
	// 	println!("Failed to find converter for this filetype.");
	// 	exit(1);
	// });

	// let selections: Vec<&String> = selected_converter
	// 	.convert_to.as_object()
	// 	.unwrap()
	// 	.iter()
	// 	.map(|v| v.0)
	// 	.collect()	
	// 	;

	// let target_output = selections[select("Convert to:",&selections)];

	// let needs_cmd_edit = confirm_prompt("Do you want to add parameters?");

	// let mut prompt = selected_converter.args.clone();

	// if needs_cmd_edit {
	// 	prompt = Editor::new().edit(&prompt).unwrap().unwrap();
	// }

	// for file in &files {
	// 	run_converter(
	// 		&selected_converter,
	// 		&prompt,
	// 		file.as_path().to_str().unwrap(), 
	// 		&(String::new() + file.file_stem().unwrap().to_str().expect("") + "." + &target_output),
	// 		&file_type,
	// 		&target_output)
	// }


}