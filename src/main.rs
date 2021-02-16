use app::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match app::run(config) {
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
        Ok(result) => println!("{}", result),
    }
}
