use converter::{
    converters::{find_converter, get_converters},
    converting::run_converter,
    file_types::{FileTypeError, FileTypes},
    prompts::{confirm_prompt, select},
};
use dialoguer::Editor;
use env_logger::Builder;
use log::{debug, log_enabled, Log};
use std::{
    fmt::{Debug, Display},
    fs,
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

    #[arg(short)]
    verbose: bool,

    #[arg(short, long, default_value = "true")]
    use_default_formats: bool,

    #[arg(short, long)]
    yes: bool,
    // #[arg(short, long)]
    // input_type: Option<String>,

    // #[arg(short, long)]
    // output_type: Option<String>,
}

fn error_exit<T: Display>(e: T, extra_message: Option<&str>) -> ! {
    eprintln!("Error: {}{}", extra_message.unwrap_or_default(), e);
    if !log_enabled!(log::Level::Error) {
        eprintln!("Hint: Try running the command again with -v for further debugging");
    }
    exit(1)
}

fn get_correct_extension(
    use_default_format: bool,
    target: &Path,
    file_types: &FileTypes,
    ext: String,
    prompt: String,
) -> Result<String, FileTypeError> {
    let types = file_types.get_file_types(ext)?;

    if types.len() == 1 {
        return Ok(types[0].clone());
    }

    if use_default_format {
        debug!("Using default format for {:?}", target);
        return Ok(types[0].clone());
    }

    let idx = select(&prompt, &types);

    Ok(types[idx].clone())
}

fn get_extension(use_default_format: bool, target: &Path, file_types: &FileTypes) -> String {
    let prompt = format!(
        "Multiple filetypes available. Please specify type for {:?}",
        &target.as_os_str()
    );

    match target
        .extension()
        .and_then(|v| v.to_str())
        .map(|v| v.to_lowercase())
        .map(|v| get_correct_extension(use_default_format, &target, file_types, v, prompt))
    {
        Some(ext) => match ext {
            Ok(e) => e,
            Err(e) => error_exit(e, None),
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

    Builder::new()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Off
        })
        .init();

    let loaded_converters = match get_converters() {
        Ok(v) => v,
        Err(e) => error_exit(e, Some("Failed to load converters: ")),
    };

    let file_types = match FileTypes::load() {
        Ok(f) => f,
        Err(e) => error_exit(e, Some("Failed to load filetypes table: ")),
    };

    // dbg!(&loaded_converters);

    let input_extension = get_extension(args.use_default_formats, &args.input_file, &file_types);
    let output_extension = get_extension(args.use_default_formats, &args.output_file, &file_types);

    let selected_converter =
        match find_converter(&loaded_converters, &input_extension, &output_extension) {
            Some(c) => c,
            None => error_exit(
                RunError::NoConversionPossible {
                    in_ext: input_extension.to_string(),
                    out_ext: output_extension.to_string(),
                },
                None,
            ),
        };

    let mut prompt = selected_converter.args.clone();

    if args.edit {
        prompt = match Editor::new().edit(&prompt) {
            Ok(v) => {
                let mut value = v.unwrap_or_default();

                if value == "" {
                    eprintln!("Using default prompt");
                    value = prompt.clone()
                }

                value
            }
            Err(e) => error_exit(e, None),
        }
    }

    if !args.yes && fs::exists(&args.output_file).is_ok_and(|x| x == true) {
        confirm_prompt(&format!(
            "{:?} already exists. Do you want to overwrite this file?",
            &args.output_file
        ));
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
        Err(e) => error_exit(e, None),
    }
}
