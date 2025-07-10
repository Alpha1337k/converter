use std::{path::Path, process::exit};

use log::debug;

use crate::{
    error_exit,
    file_types::{FileTypeError, FileTypes},
    prompts::select,
    ArgError,
};

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

pub fn get_extension(use_default_format: bool, target: &Path, file_types: &FileTypes) -> String {
    let prompt = format!(
        "Multiple filetypes available. Please specify type for {:?}",
        &target.as_os_str()
    );

    match target
        .extension()
        .and_then(|v| v.to_str())
        .map(|v| v.to_lowercase())
        .map(|v| get_correct_extension(use_default_format, target, file_types, v, prompt))
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
