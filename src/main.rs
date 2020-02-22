use minicat::Config;
use std::env;
use std::process;

fn main() {
    if env::args().len() > 2 {
        let config = Config::new(env::args()).unwrap_or_else(|err| {
            eprintln!("{}", err);

            process::exit(1);
        });

        if let Err(e) = minicat::run(config) {
            eprintln!("{}", e);

            process::exit(1);
        };
    } else {
        let config = Config::new_nocmd(env::args()).unwrap_or_else(|err| {
            eprintln!("{}", err);

            process::exit(1);
        });

        if let Err(e) = minicat::run_nocmd(config) {
            eprintln!("{}", e);

            process::exit(1);
        }
    }
}
