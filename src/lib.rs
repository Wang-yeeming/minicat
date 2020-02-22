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
please check what you just pressed.");
            }
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Cannot found the file, \
please check what you just pressed.");
            }
        };

        Ok(Config { command, filename })
    }

    pub fn new_nocmd(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let mut command = String::from("-u");
        let mut filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Cannot found the file, \
please check what you just pressed.");
            }
        };

        if filename == "--help".to_string() {
            command = "--help".to_string();
            filename = "\0".to_string();
        } else if filename == "--version".to_string() {
            command = "--version".to_string();
            filename = "\0".to_string();
        }

        Ok(Config { command, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match config.filename.as_str() {
        "\0" => "\0".to_string(),
        _ => fs::read_to_string(config.filename)?,
    };
    let cmd = config.command;

    for line in show(&cmd, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn show(cmd: &str, contents: &str) -> Vec<String> {
    match cmd {
        // Show all
        "-A" | "--show-all" | "-ET" | "-TE" => contents
            .lines()
            .map(|line| {
                let tmp = line.replace("\t", "^I");
                format!("{}$", tmp)
            })
            .collect(),
        // Number nonindex output lines
        "-b" | "--number-nonblank" => {
            let mut count: u64 = 0;

            contents
                .lines()
                .map(|line| {
                    let mut index = false;

                    for ch in line.bytes() {
                        if ch != b' ' | b'\t' {
                            index = true;
                            break;
                        }
                    }

                    if index == true {
                        count += 1;
                        format!("{:>6}  {}", count, line)
                    } else {
                        String::from(" ")
                    }
                })
                .collect()
        }
        // Display $ at end of each line
        "-E" | "--show-ends" => contents.lines().map(|line| format!("{}$", line)).collect(),
        // Number all output lines
        "-n" | "--number" => {
            let mut count: u64 = 0;

            contents
                .lines()
                .map(|line| {
                    count += 1;
                    format!("{:>6}  {}", count, line)
                })
                .collect()
        }
        // Display TAB characters as ^I
        "-T" | "--show-tabs" => contents
            .lines()
            .map(|line| {
                let tmp = line.replace("\t", "^I");
                tmp
            })
            .collect(),
        // Ignore
        "-u" => contents.lines().map(|line| line.to_string()).collect(),
        // Display this help and exit
        "--help" => vec![String::from(
            "\
name:    minicat
version: 0.2.0
author:  Wang-yeeming <yeeming0771@foxmail.com>
about:   A self-made CLI tool, simply implement of the cat.

USAGE

	minicat [OPTIONS] [FILE]

OPTIONS

	-A, --show-all           equivalent to -ET
	-b, --number-nonblank    number nonempty output lines, overrides -n
	-E, --show-ends          display $ at end of each line
	-n, --number             number all output lines
	-T, --show-tabs          display TAB characters as ^I
	-u                       (ignore)
	    --help               display this help and exit
	    --version            output version information and exit
	
Have a try!",
        )],
        // Output version information and exit
        "--version" => vec!["minicat 0.2.0".to_string()],
        // Unidentified command
        _ => vec![format!(
            "Cannot found command named: {}.
Please press '--help' for help.
If you wanna output directly, try: minicat -u [FILE]",
            cmd
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_text() -> String {
        let text = String::from(
            "\
#include <iostream>

int main() {
	std::cout << \"Hello, world!\" << std::endl;

	return 0;
}",
        );
        text
    }

    #[test]
    fn use_number_nonempty_cmd() {
        let text = generate_text();

        assert_eq!(
            show("--number-nonblank", &text),
            vec![
                "     1  #include <iostream>",
                " ",
                "     2  int main() {",
                "     3  \tstd::cout << \"Hello, world!\" << std::endl;",
                " ",
                "     4  \treturn 0;",
                "     5  }"
            ]
        )
    }

    #[test]
    fn use_show_ends_cmd() {
        let text = generate_text();

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
        let text = generate_text();

        assert_eq!(
            show("-n", &text),
            vec![
                "     1  #include <iostream>",
                "     2  ",
                "     3  int main() {",
                "     4  \tstd::cout << \"Hello, world!\" << std::endl;",
                "     5  ",
                "     6  \treturn 0;",
                "     7  }"
            ]
        )
    }

    #[test]
    fn use_unidentified_cmd() {
        let text = generate_text();

        assert_eq!(
            show("foobar", &text),
            vec![
                "Cannot found command named: foobar.
Please press '--help' for help.
If you wanna output directly, try: minicat -u [FILE]"
            ]
        )
    }

    #[test]
    fn use_directly_cmd() {
        let text = generate_text();

        assert_eq!(
            show("-u", &text),
            vec![
                "#include <iostream>",
                "",
                "int main() {",
                "	std::cout << \"Hello, world!\" << std::endl;",
                "",
                "	return 0;",
                "}"
            ]
        )
    }

    #[test]
    fn use_show_tabs_cmd() {
        let text = generate_text();

        assert_eq!(
            show("-T", &text),
            vec![
                "#include <iostream>",
                "",
                "int main() {",
                "^Istd::cout << \"Hello, world!\" << std::endl;",
                "",
                "^Ireturn 0;",
                "}"
            ]
        )
    }

    #[test]
    fn use_show_all_cmd() {
        let text = generate_text();

        assert_eq!(
            show("--show-all", &text),
            vec![
                "#include <iostream>$",
                "$",
                "int main() {$",
                "^Istd::cout << \"Hello, world!\" << std::endl;$",
                "$",
                "^Ireturn 0;$",
                "}$"
            ]
        )
    }
}
