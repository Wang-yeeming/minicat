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
        //		"-b" | "--number-nonblock" => ,
        // Display $ at end of each line
        "-E" | "--show-ends" => contents.lines().map(|line| format!("{}$", line)).collect(),
        // Number all output lines
        "-n" | "--number" => {
            let mut count: u64 = 0;
            let mut collecter: Vec<String> = Vec::new();

            loop {
                let arg = match contents.lines().next() {
                    Some(l) => l,
                    None => return collecter,
                };

                count += 1;

                collecter.push(
                    contents
                        .lines()
                        .map(|_| format!("{:>6}  {}", count, arg))
                        .collecter(),
                );
            }
        }
        // Display TAB characters as ^I
        //		"-T" | "--show-tabs" => vec!["ok"],
        // Display this help and exit
        //		"-h" | "--help" => vec!["ok"],
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
                "#include <iostream>$",
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
    fn use_number_cmd() {
        let text = String::from(
            "\
#include <iostream>

int main() {
	std::cout << \"Hello, world!\" << std::endl;

	return 0;
}",
        );

        assert_eq!(
            show("-n", &text),
            vec![
                "     1  #include <iostream>",
                "     2  ",
                "     3  int main() {",
                "     4		std::cout << \"Hello, world\" << std::endl;",
                "     5  ",
                "     6		return 0;",
                "     7  }"
            ]
        )
    }
}
