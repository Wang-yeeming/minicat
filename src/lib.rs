use std::env;
use std::error::Error;
use std::fs;

static mut FLAG: bool = false;

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
        "-A" | "--show-all" | "-vET" | "-vTE" | "-EvT" | "-TvE" | "-ETv" | "-TEv" => contents
            .lines()
            .map(|line| {
                let mut collecter = Vec::<String>::new();

                for mut ch in line.chars() {
                    if !ch.is_ascii() {
                        ch = '@';
                    }

                    collecter.push(ch.to_string());
                }

                let mut tmp = String::new();

                for st in collecter.iter() {
                    tmp = format!("{}{}", tmp, st);
                }

                tmp = tmp.replace("\t", "^I");

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

                    if index {
                        // nonempty
                        count += 1;
                        format!("{:>6}  {}", count, line)
                    } else {
                        String::from(" ")
                    }
                })
                .collect()
        }
        // -vE
        "-e" | "-vE" | "-Ev" => contents
            .lines()
            .map(|line| {
                let mut collecter = Vec::<String>::new();

                for mut ch in line.chars() {
                    if !ch.is_ascii() {
                        ch = '@';
                    }

                    collecter.push(ch.to_string());
                }

                let mut tmp = String::new();

                for st in collecter.iter() {
                    tmp = format!("{}{}", tmp, st);
                }

                format!("{}$", tmp)
            })
            .collect(),
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
        // Suppress repeated empty output lines
        "-s" | "--squeeze-blank" => {
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

                    unsafe {
                        if index {
                            // nonempty
                            FLAG = false;
                            line.to_string()
                        } else {
                            if FLAG {
                                String::from("\u{8}") // unsupplied in Rust
                            } else {
                                FLAG = true;
                                String::from(" ")
                            }
                        }
                    }
                })
                .collect()
        }
        // -vT
        "-t" | "-vT" | "-Tv" => contents
            .lines()
            .map(|line| {
                let mut collecter = Vec::<String>::new();

                for mut ch in line.chars() {
                    if !ch.is_ascii() {
                        ch = '@';
                    }

                    collecter.push(ch.to_string());
                }

                let mut tmp = String::new();

                for st in collecter.iter() {
                    tmp = format!("{}{}", tmp, st);
                }

                tmp = tmp.replace("\t", "^I");

                tmp
            })
            .collect(),
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
        // Use ^M notation, except for LFD and TAB
        "-v" | "--show-nonascii" => contents
            .lines()
            .map(|line| {
                let mut collecter = Vec::<String>::new();

                for mut ch in line.chars() {
                    if !ch.is_ascii() {
                        ch = '@';
                    }

                    collecter.push(ch.to_string());
                }

                let mut tmp = String::new();

                for st in collecter.iter() {
                    tmp = format!("{}{}", tmp, st);
                }

                tmp
            })
            .collect(),
        // Display this help and exit
        "--help" => vec![String::from(
            "\
name:    minicat
version: 1.0.0
author:  Wang-yeeming <yeeming0771@foxmail.com>
about:   A self-made CLI tool, simple implement of the cat.

USAGE

	minicat [OPTIONS] [FILE]

OPTIONS

	-A, --show-all           equivalent to -vET
	-b, --number-nonblank    number nonempty output lines, overrides -n
	-e                       equivalent to -vE
	-E, --show-ends          display $ at end of each line
	-n, --number             number all output lines
	-s, --squeeze-blank      suppress repeated empty output lines
	-t                       equivalent to -vT
	-T, --show-tabs          display TAB characters as ^I
	-u                       (ignore)
	-v, --show-nonascii      display non-ASCII characters as @
	    --help               display this help and exit
	    --version            output version information and exit

Have a try!",
        )],
        // Output version information and exit
        "--version" => vec!["minicat 1.0.0".to_string()],
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
        let text = String::from("\t\t我中国人、\n日本语了解不能。");

        assert_eq!(show("--show-all", &text), vec!["^I^I@@@@@$", "@@@@@@@@$"])
    }

    #[test]
    fn use_show_nonascii_cmd() {
        let text = String::from("你好世界\nHello world");

        assert_eq!(show("-v", &text), vec!["@@@@", "Hello world"])
    }

    #[test]
    fn use_t_cmd() {
        let text = String::from("你好\t世界\n\t\tHello world");

        assert_eq!(show("-t", &text), vec!["@@^I@@", "^I^IHello world"])
    }

    #[test]
    fn use_e_cmd() {
        let text = String::from("你好\n\t世界");

        assert_eq!(show("-e", &text), vec!["@@$", "\t@@$"])
    }

    #[test]
    fn use_squeeze_blank_cmd() {
        let text = String::from("\n\n\n\n\n");

        assert_eq!(show("-s", &text), vec!["\n"])
    }
}
