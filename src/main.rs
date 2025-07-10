use converter::{
    converters::{find_converter, get_converters},
    converting::run_converter,
    error_exit,
    extension::get_extension,
    file_types::FileTypes,
    prompts::confirm_prompt,
};
use dialoguer::Editor;
use env_logger::Builder;
use std::{fmt::Debug, fs, path::PathBuf, process::exit};

use clap::Parser;
use thiserror::Error;

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

                if value.is_empty() {
                    eprintln!("Using default prompt");
                    value = prompt.clone()
                }

                value
            }
            Err(e) => error_exit(e, None),
        }
    }

    if !args.yes && fs::exists(&args.output_file).is_ok_and(|x| x) {
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
