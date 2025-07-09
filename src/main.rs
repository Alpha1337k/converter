use converter::{
    converters::{find_converter, get_converters},
    converting::run_converter,
    file_types::{FileTypeError, FileTypes},
    prompts::select,
};
use dialoguer::Editor;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ArgError {
    #[error("Not enough arguments provided: {len}. Need at least two.")]
    InvalidArgumentCount { len: usize },

    #[error("Unrecognized file extension for '{file}'.")]
    UnrecognizedExtension { file: PathBuf },

    #[error("Invalid filename.")]
    InvalidFileName,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RunError {
    #[error("Could not find a possible conversion from {in_ext} to {out_ext}.")]
    NoConversionPossible { in_ext: String, out_ext: String },
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

fn get_correct_extension(
    file_types: &FileTypes,
    ext: String,
    prompt: String,
) -> Result<String, FileTypeError> {
    let types = file_types.get_file_types(ext)?;

    if types.len() == 1 {
        return Ok(types[0].clone());
    }

    let idx = select(&prompt, &types);

    Ok(types[idx].clone())
}

fn get_extension(target: &Path, file_types: &FileTypes) -> String {
    let prompt = format!(
        "Multiple filetypes available. Please specify type for {:?}",
        &target.as_os_str()
    );

    match target
        .extension()
        .and_then(|v| v.to_str())
        .map(|v| v.to_lowercase())
        .map(|v| get_correct_extension(file_types, v, prompt))
    {
        Some(ext) => match ext {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1)
            }
        },
        None => {
            eprintln!(
                "{}",
                ArgError::UnrecognizedExtension {
                    file: target.into()
                }
            );
            exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();

    let loaded_converters = match get_converters() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
    };

    let file_types = match FileTypes::load() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
    };

    // dbg!(&loaded_converters);

    let input_extension = get_extension(&args.input_file, &file_types);
    let output_extension = get_extension(&args.output_file, &file_types);

    let selected_converter =
        match find_converter(&loaded_converters, &input_extension, &output_extension) {
            Some(c) => c,
            None => {
                eprintln!(
                    "Error: {}",
                    RunError::NoConversionPossible {
                        in_ext: input_extension.to_string(),
                        out_ext: output_extension.to_string()
                    }
                );
                exit(1)
            }
        };

    let mut prompt = selected_converter.args.clone();

    if args.edit {
        prompt = Editor::new().edit(&prompt).unwrap().unwrap();
    }

    match run_converter(
        selected_converter,
        &prompt,
        args.input_file.to_str().unwrap(),
        args.output_file.to_str().unwrap(),
        &input_extension,
        &output_extension,
    ) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1)
        }
    }
}
