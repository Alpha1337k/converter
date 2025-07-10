use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader, Write},
    process::{Command, ExitStatus, Stdio},
    thread::sleep,
    time::{self, Duration, SystemTime},
};

use crate::{constants::LOADING_ANIMATION, converters::Converter};
use console::style;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertRunError {
    #[error("Failed to run program {program}: {e}")]
    CommandFailure {
        program: String,
        e: Box<dyn std::error::Error>,
    },

    #[error("{program} was not created.")]
    FileNotCreated { program: String },

    #[error("{program} was not overwritten. (last overwrite: {:?})", modified_time)]
    FileNotOverwritten {
        program: String,
        modified_time: SystemTime,
    },
}

fn dump_error_logs(
    converter: &Converter,
    input: &str,
    output: &str,
    parsed_command: String,
    mut result: std::process::Child,
) {
    println!("{} {} -> {}", style("🞫").red().bold(), input, output);
    println!("{}", style("---").dim());

    println!("> {} {}", converter.program_name, parsed_command);

    let stdout = result.stdout.take().unwrap();
    let stderr = result.stderr.take().unwrap();

    let lines_stdout = BufReader::new(stdout).lines();
    for line in lines_stdout {
        println!("<\t{}", line.unwrap());
    }

    let lines_stderr = BufReader::new(stderr).lines();
    for line in lines_stderr {
        println!("<2\t{}", line.unwrap());
    }

    println!("{}", style("---").dim());
}

pub fn validate_conversion(output: &str) -> Result<(), Box<dyn Error>> {
    let now = SystemTime::now();

    match fs::exists(output)? {
        true => {
            let metadata = fs::metadata(output)?;
            let modified_time = metadata.modified()?;

            if modified_time < now - Duration::from_secs(1) {
                Err(ConvertRunError::FileNotOverwritten {
                    program: output.into(),
                    modified_time,
                }
                .into())
            } else {
                Ok(())
            }
        }
        false => Err(ConvertRunError::FileNotCreated {
            program: output.into(),
        }
        .into()),
    }
}

pub fn run_converter(
    converter: &Converter,
    args: &str,
    input: &str,
    output: &str,
    input_type: &str,
    output_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_command = args
        .replace("%INFORM%", &format!("'{}'", input_type))
        .replace("%OUTFORM%", &format!("'{}'", output_type))
        .replace("%OUTFILE%", &format!("'{}'", output))
        .replace("%INFILE%", &format!("'{}'", input));

    let mut result = match Command::new(&converter.program_name)
        .args(shlex::split(&parsed_command).unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(ConvertRunError::CommandFailure {
                program: converter.program_name.clone(),
                e: e.into(),
            }
            .into())
        }
    };

    let mut loading_char_idx = 0;

    while result.try_wait().is_ok_and(|x| x.is_none()) {
        print!(
            "{}{} {} -> {}",
            ansi_escapes::EraseLines(1),
            LOADING_ANIMATION[loading_char_idx % LOADING_ANIMATION.len()],
            input,
            output
        );
        io::stdout().flush().unwrap();
        loading_char_idx += 1;
        sleep(time::Duration::from_millis(100));
    }

    println!("{}", ansi_escapes::EraseLines(1));

    if ExitStatus::success(&result.wait().unwrap()) {
        match validate_conversion(output) {
            Ok(_) => {}
            Err(e) => {
                println!("{} {} -> {}", style("🞫").red().bold(), input, output);

                eprintln!("Error: {}", e);
                return Err("Failed to convert file.".into());
            }
        }

        println!("{} {} -> {}\t", style("✔").green().bold(), input, output);
        Ok(())
    } else {
        dump_error_logs(converter, input, output, parsed_command, result);
        Err("Failed to convert file.".into())
    }
}
