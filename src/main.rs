use converter::{
    constants::CONFIG_DIR,
    converters::{find_converter, get_converters},
    converting::run_converter,
    error_exit,
    extension::get_extension,
    file_types::FileTypes,
    prompts::confirm_prompt,
    setup::setup_user_files,
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
#[command(version, about = "One stop shop for converting filetypes with ease.")]
struct Args {
    /// Input file to convert
    #[arg(required_unless_present = "setup")]
    input_file: Option<PathBuf>,

    /// Output file to write
    #[arg(required_unless_present = "setup")]
    output_file: Option<PathBuf>,

    /// Edit parameters before the conversion
    #[arg(short, long)]
    edit: bool,

    /// Verbose debugging output
    #[arg(short)]
    verbose: bool,

    #[arg(short, long, default_value = "true")]
    use_default_formats: bool,

    #[arg(short, long)]
    yes: bool,

    #[arg(long)]
    setup: bool,
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

    if args.setup {
        match setup_user_files() {
            Ok(_) => {
                println!("Configuration installed successfully.");
                println!("You can add your own files at '~/{CONFIG_DIR}'.");
                exit(0)
            }
            Err(e) => error_exit(e, Some("Failed to install files: ")),
        }
    }

    if fs::exists(
        dirs::home_dir()
            .unwrap_or_else(|| error_exit("Failed to fetch Home directory", None))
            .join(CONFIG_DIR),
    )
    .is_ok_and(|x| !x)
    {
        error_exit(
            "Config directory does not exist. Run --setup to configure.",
            None,
        )
    }

    let loaded_converters = match get_converters() {
        Ok(v) => v,
        Err(e) => error_exit(e, Some("Failed to load converters: ")),
    };

    let file_types = match FileTypes::load() {
        Ok(f) => f,
        Err(e) => error_exit(e, Some("Failed to load filetypes table: ")),
    };

    let input_file = args.input_file.unwrap();
    let output_file = args.output_file.unwrap();

    // dbg!(&loaded_converters);

    let input_extension = get_extension(args.use_default_formats, &input_file, &file_types);
    let output_extension = get_extension(args.use_default_formats, &output_file, &file_types);

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

    if !args.yes && fs::exists(&output_file).is_ok_and(|x| x) {
        confirm_prompt(&format!(
            "{:?} already exists. Do you want to overwrite this file?",
            &output_file
        ));
    }

    match run_converter(
        selected_converter,
        &prompt,
        input_file.to_str().unwrap(),
        output_file.to_str().unwrap(),
        &input_extension,
        &output_extension,
    ) {
        Ok(_) => exit(0),
        Err(e) => error_exit(e, None),
    }
}
