use std::{error::Error, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
	pub fn new<'a, I>(mut args: I) -> Result<Config, &'a str>
	where I: Iterator<Item = String>,
	{
		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Missing argument - query string"),
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Missing argument - filename"),
		};

		Ok(Config{query, filename})

	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect::<_>()
}
