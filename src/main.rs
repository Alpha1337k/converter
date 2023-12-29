use std::{collections::HashSet, ffi::{OsStr, OsString}, arch::x86_64};

use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use glob::{glob, Paths};

pub mod converters;
use crate::converters::get_converters;

fn select(options: &[&str]) -> usize {
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

fn get_target_files(raw_glob: String) -> Paths {
	let files = glob(&raw_glob).expect("Error: invalid glob pattern.");

	return files
}

fn get_extension(files: &mut Paths) -> Option<String> {
	let mut extensions = HashSet::new();

	for file in files {
		match file {
			Ok(f) => {
				let ext = f.extension().unwrap();
				extensions.insert(OsString::from(ext).into_string().expect("Failed conversion"));
			}
			Err(err) => {
				println!("Failed to get extension: {}", err);
			}
		}
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

	let extension = get_extension(&mut files).expect("WTF");
		// .get_or_insert(
		// 	basic_prompt("Could not automatically find extension. Enter extension: ")
		// ).to_string();

	println!("Extension used is {}.", extension);

	for file in files {
		match file {
			Ok(path) => println!("{:?}", path.display()),
			Err(e) => println!("{:?}", e)
		}
	}

	let selections = &[""];

    println!("Enjoy your {}!", selections[select(selections)]);
}