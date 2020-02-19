use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub command: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Cannot found the command, \
please check what you just pressed.")
            }
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Cannot found the file, \
please check what you just pressed.")
            }
        };

        Ok(Config { command, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let cmd = config.command;

    for line in show(&cmd, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn show(cmd: &str, contents: &str) -> Vec<String> {
    match cmd {
        // Number nonempty output lines
        //		"-B" | "--number-nonblock" => ,
        // Display $ at end of each line
        "-E" | "--show-ends" => contents.lines().map(|line| format!("{}$", line)).collect(),
        // Number all output lines
        //"-N" | "--number" => {}
        // Display TAB characters as ^I
        //		"-T" | "--show-tabs" => vec!["ok"],
        // Display this help and exit
        //		"-H" | "--help" => vec!["ok"],
        // Output version information and exit
        //		"-V" | "--version" => vec!["ok"],
        // Default
        _ => vec!["ok".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_show_ends_cmd() {
        let text = String::from(
            "\
#include <iostream>

int main() {
	std::cout << \"Hello, world!\" << std::endl;

	return 0;
}",
        );

        assert_eq!(
            show("-E", &text),
            vec![
                "\
#include <iostream>$",
                "$",
                "int main() {$",
                "	std::cout << \"Hello, world!\" << std::endl;$",
                "$",
                "	return 0;$",
                "}$"
            ]
        )
    }

    #[test]
    fn use_number_cmd() {}
}
