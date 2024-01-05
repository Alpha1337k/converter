use std::{fs::{self}, process::{Command, ExitStatus, Stdio}, io::{self, Write, BufReader, BufRead}, thread::sleep, time};

use console::style;
use glob::glob;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct Converter {
	pub name: String,
	pub program_name: String,
	pub args: String,
	pub convert_from: serde_json::Value,
	pub convert_to: serde_json::Value,
}

pub fn get_converters() -> Vec<Converter>
{
	let mut converters = Vec::with_capacity(0);

	for item in glob("converters/*.json").expect("Error: No converter manifests found.") {
		match item {
			Ok(v) => {
				let file = fs::read_to_string(v.as_os_str()).expect("Error: Failed to open converter file.");
				let converter: Converter = serde_json::from_str(&file).expect("Failed to parse converter.");

				converters.push(converter);
			}
			Err(err) => {
				println!("{:?}", err)
			}
		}
	}

	return converters;
}

pub fn run_converter(converter: &Converter, args: &str, input: &str, output: &str, input_type: &str, output_type: &str) {
	let parsed_command = args
		.replace("%INFORM%", &format!("'{}'", input_type))
		.replace("%OUTFORM%", &format!("'{}'", output_type))
		.replace("%OUTFILE%", &format!("'{}'", output))
		.replace("%INFILE%", &format!("'{}'", input));

	let loading_chars = ["⠏","⠛","⠹","⠼","⠶","⠧"];
	let mut loading_char_idx = 0;

	let mut result = Command::new(&converter.program_name)
		.args(shlex::split(&parsed_command).unwrap())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("Failed to run program.");

	while result.try_wait().is_ok_and(|x| x == None) {
		print!("{}{} {} -> {}", ansi_escapes::EraseLines(1), 
			loading_chars[loading_char_idx % loading_chars.len()],
			input, 
			output);
		io::stdout().flush().unwrap();
		loading_char_idx += 1;
		sleep(time::Duration::from_millis(100));
	}


	if ExitStatus::success(&result.wait().unwrap()) == false {
		println!("{}{} {} -> {}", ansi_escapes::EraseLines(1), style("🞫").red().bold() , input, output);
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
	} else {
		println!("{}{} {} -> {}\t", ansi_escapes::EraseLines(1), style("✔").green().bold() , input, output);
	}
	
}