use std::error::Error;
use std::fs;
use std::env;
use std::io::{self, Write, BufWriter};

pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool,
}

impl Config {
	pub fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguements!\nUsage: minigrep <key> <file_path>")
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		let ignore_case = env::var("IGNORE_CASE").is_ok();

		Ok(Config {query, file_path, ignore_case})
	}
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	let query = query.to_lowercase(); //returns String

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) { //which is why we use & ref here
			results.push(line);
		}
	}

	results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;

	let greplines = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};

	let stdout = io::stdout();
    let mut buf_writer = BufWriter::new(stdout.lock());

	for line in greplines {
		buf_writer.write(line.as_bytes()).unwrap();
		buf_writer.write(b"\n").unwrap();
	}
	buf_writer.flush().unwrap();

    Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive_search() {
		let query = "duct";
		let text = "\
Rust:
safe, fast, productive
Pick three.
Duct tape.
";
		assert_eq!(
			vec!["safe, fast, productive"], 
			search(query, text));
	}

	#[test]
	fn case_insensitive_search() {
		let query = "RuSt";
		let text = "\
Rust:
safe, fast, productive
Pick three
Trust me.
";		
		assert_eq!(
			vec!["Rust:", "Trust me."], 
			search_case_insensitive(query, text));
	}
}