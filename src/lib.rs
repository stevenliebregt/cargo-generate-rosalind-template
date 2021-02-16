use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct Config {
    pub file_name: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Remove first (calling script) argument.

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file argument"),
        };

        Ok(Config { file_name })
    }
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    Ok(echo(&contents))
}

pub fn echo(input: &String) -> String {
    String::from(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            "Hello, world!",
            echo(&String::from("Hello, world!"))
        )
    }
}
