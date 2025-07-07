use std::{collections::HashMap, error::Error, fs::{self}, io::{self, BufRead, BufReader, Write}, process::{Command, ExitStatus, Stdio}, thread::sleep, time};

use console::style;
use glob::glob;
use serde::{Deserialize, Serialize};

use crate::constants::{CONVERTER_CONFIG_DIR, LOADING_ANIMATION};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Converter {
	pub name: String,
	pub program_name: String,
	pub args: String,
	pub convert_from: HashMap<String, String>,
	pub convert_to: HashMap<String, String>,
}

pub fn get_converters() -> Result<Vec<Converter>, Box<dyn Error>>
{
	let mut converters = Vec::new();

	let converter_dir = format!("{CONVERTER_CONFIG_DIR}/converters/*.json");

	let iter = match glob(&converter_dir) {
		Ok(i) => i,
		Err(e) => return Err(e.into())
	};

	for entry in iter.flatten() {
		match fs::read_to_string(&entry)
			.and_then(
				|file| serde_json::from_str::<Converter>(&file)
				.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
			)
		{
			Ok(converter) => converters.push(converter),
			Err(e) => eprintln!("Failed to load {:?}: {}", entry, e),
		}
	}

	return Ok(converters);
}

pub fn find_converter<'a>(converters: &'a Vec<Converter>, input_extension: &str, output_extension: &str) -> Option<&'a Converter> {
	for converter in converters {
		if converter.convert_from.get(input_extension)
			.is_some_and(|_| converter.convert_to.get(output_extension).is_some()) {
				return Some(converter)
			}
	}

	None
}

fn dump_error_logs(converter: &Converter, input: &str, output: &str, parsed_command: String, mut result: std::process::Child) {
	println!("{} {} -> {}", style("🞫").red().bold() , input, output);
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

pub fn run_converter(converter: &Converter, args: &str, input: &str, output: &str, input_type: &str, output_type: &str) {
	let parsed_command = args
		.replace("%INFORM%", &format!("'{}'", input_type))
		.replace("%OUTFORM%", &format!("'{}'", output_type))
		.replace("%OUTFILE%", &format!("'{}'", output))
		.replace("%INFILE%", &format!("'{}'", input));

	let mut result = Command::new(&converter.program_name)
		.args(shlex::split(&parsed_command).unwrap())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("Failed to run program.");

	let mut loading_char_idx = 0;

	while result.try_wait().is_ok_and(|x| x == None) {
		print!("{}{} {} -> {}", ansi_escapes::EraseLines(1), 
			LOADING_ANIMATION[loading_char_idx % LOADING_ANIMATION.len()],
			input, 
			output);
		io::stdout().flush().unwrap();
		loading_char_idx += 1;
		sleep(time::Duration::from_millis(100));
	}

	println!("{}", ansi_escapes::EraseLines(1));

	if ExitStatus::success(&result.wait().unwrap()) {
		println!("{} {} -> {}\t", style("✔").green().bold() , input, output);
	} else {
		dump_error_logs(converter, input, output, parsed_command, result);
	}
	
}
