use minicat::Config;
use std::env;
use std::process;

fn main() {
    let mut config = Config {
        command: "-u".to_string(),
        filename: "\0".to_string(),
    };

    if env::args().len() > 2 {
        config = Config::new(env::args()).unwrap_or_else(|err| {
            eprintln!("{}", err);

            process::exit(1);
        });
    } else {
        config = Config::new_nocmd(env::args()).unwrap_or_else(|err| {
            eprintln!("{}", err);

            process::exit(1);
        });
    }

    if let Err(e) = minicat::run(config) {
        eprintln!("{}", e);

        process::exit(1);
    };
}
