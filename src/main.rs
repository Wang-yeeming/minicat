use minicat::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);

        process::exit(1);
    });

    if let Err(e) = minicat::run(config) {
        println!("{}", e);

        process::exit(1);
    };
}
