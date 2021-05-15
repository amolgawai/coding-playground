use std::{env, process};
use minigrep::Config;

fn main() {

    let config = Config::new(env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments - {}", err);
        process::exit(1);
    } );

    println!("finding {}, in file {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error - {}", e);
        process::exit(1);
    }

}
