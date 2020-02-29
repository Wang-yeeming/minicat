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
		match line {
			Some(txt) => {println!("{}", txt);},
			None => (),
		}
    }

    Ok(())
}

pub fn show(cmd: &str, contents: &str) -> Vec<Option<String>> {
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

                Some(format!("{}$", tmp))
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
                        Some(format!("{:>6}  {}", count, line))
                    } else {
                        Some(String::from(" "))
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

                Some(format!("{}$", tmp))
            })
            .collect(),
        // Display $ at end of each line
        "-E" | "--show-ends" => contents.lines().map(|line| Some(format!("{}$", line))).collect(),
        // Number all output lines
        "-n" | "--number" => {
            let mut count: u64 = 0;

            contents
                .lines()
                .map(|line| {
                    count += 1;
                    Some(format!("{:>6}  {}", count, line))
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
                            Some(line.to_string())
                        } else {
                            if FLAG {
								None
                            } else {
                                FLAG = true;
                                Some(String::from(" "))
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

                Some(tmp)
            })
            .collect(),
        // Display TAB characters as ^I
        "-T" | "--show-tabs" => contents
            .lines()
            .map(|line| {
                let tmp = line.replace("\t", "^I");
                Some(tmp)
            })
            .collect(),
        // Ignore
        "-u" => contents.lines().map(|line| Some(line.to_string())).collect(),
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

                Some(tmp)
            })
            .collect(),
        // Display this help and exit
        "--help" => vec![Some(String::from(
            "\
name:    minicat
version: 1.1.0
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
        ))],
        // Output version information and exit
        "--version" => vec![Some("minicat 1.1.0".to_string())],
        // Unidentified command
        _ => vec![Some(format!(
            "Cannot found command named: {}.
Please press '--help' for help.
If you wanna output directly, try: minicat -u [FILE]",
            cmd
        ))],
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
                Some("     1  #include <iostream>".to_string()),
                Some(" ".to_string()),
                Some("     2  int main() {".to_string()),
                Some("     3  \tstd::cout << \"Hello, world!\" << std::endl;".to_string()),
                Some(" ".to_string()),
                Some("     4  \treturn 0;".to_string()),
                Some("     5  }".to_string())
            ]
        )
    }

    #[test]
    fn use_show_ends_cmd() {
        let text = generate_text();

        assert_eq!(
            show("-E", &text),
            vec![
                Some("#include <iostream>$".to_string()),
                Some("$".to_string()),
                Some("int main() {$".to_string()),
                Some("	std::cout << \"Hello, world!\" << std::endl;$".to_string()),
                Some("$".to_string()),
                Some("	return 0;$".to_string()),
                Some("}$".to_string())
            ]
        )
    }

    #[test]
    fn use_number_cmd() {
        let text = generate_text();

        assert_eq!(
            show("-n", &text),
            vec![
                Some("     1  #include <iostream>".to_string()),
                Some("     2  ".to_string()),
                Some("     3  int main() {".to_string()),
                Some("     4  \tstd::cout << \"Hello, world!\" << std::endl;".to_string()),
                Some("     5  ".to_string()),
                Some("     6  \treturn 0;".to_string()),
                Some("     7  }".to_string())
            ]
        )
    }

    #[test]
    fn use_unidentified_cmd() {
        let text = generate_text();

        assert_eq!(
            show("foobar", &text),
            vec![
                Some("Cannot found command named: foobar.
Please press '--help' for help.
If you wanna output directly, try: minicat -u [FILE]".to_string())
            ]
        )
    }

    #[test]
    fn use_directly_cmd() {
        let text = generate_text();

        assert_eq!(
            show("-u", &text),
            vec![
                Some("#include <iostream>".to_string()),
                Some("".to_string()),
                Some("int main() {".to_string()),
                Some("	std::cout << \"Hello, world!\" << std::endl;".to_string()),
                Some("".to_string()),
                Some("	return 0;".to_string()),
                Some("}".to_string())
            ]
        )
    }

    #[test]
    fn use_show_tabs_cmd() {
        let text = generate_text();

        assert_eq!(
            show("-T", &text),
            vec![
                Some("#include <iostream>".to_string()),
                Some("".to_string()),
                Some("int main() {".to_string()),
                Some("^Istd::cout << \"Hello, world!\" << std::endl;".to_string()),
                Some("".to_string()),
                Some("^Ireturn 0;".to_string()),
                Some("}".to_string())
            ]
        )
    }

    #[test]
    fn use_show_all_cmd() {
        let text = String::from("\t\t我中国人、\n日本语了解不能。");

        assert_eq!(show("--show-all", &text), vec![Some("^I^I@@@@@$".to_string()), Some("@@@@@@@@$".to_string())])
    }

    #[test]
    fn use_show_nonascii_cmd() {
        let text = String::from("你好世界\nHello world");

        assert_eq!(show("-v", &text), vec![Some("@@@@".to_string()), Some("Hello world".to_string())])
    }

    #[test]
    fn use_t_cmd() {
        let text = String::from("你好\t世界\n\t\tHello world");

        assert_eq!(show("-t", &text), vec![Some("@@^I@@".to_string()), Some("^I^IHello world".to_string())])
    }

    #[test]
    fn use_e_cmd() {
        let text = String::from("你好\n\t世界");

        assert_eq!(show("-e", &text), vec![Some("@@$".to_string()), Some("\t@@$".to_string())])
    }

    #[test]
    fn use_squeeze_blank_cmd() {
        let text = String::from("\n\n\n\n\n");

        assert_eq!(show("-s", &text), vec![Some(" ".to_string()), None, None, None, None])
    }
}
