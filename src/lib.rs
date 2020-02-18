use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
	pub index: bool,
	pub command: String,
	pub filename: String,
}

impl Config {
	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		args.next();

		let index: bool;
		let command = match args.next() {
			Some(arg) => {
				index = true;
				arg
			},
			None => {
				index = false;
				"default"
			}.to_string(),
		};
		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Cannot found the file, \
please check what you just pressed."),
		};

		Ok(Config {index, command, filename})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	Ok(())
}
