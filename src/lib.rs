//! # minigrep
//!
//! `minigrep` is a cut-down version of the Linux 'grep' tool written for fun

use std::{env,fs};
use std::error::Error;

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
	args.next();

        let query = match args.next() {
		Some(arg) => arg,
		None => return Err("Didn't get a query string"),
	};

	let filename = match args.next() {
		Some(arg) => arg,
		None => return Err("Didn't get a file name"),
	};
	let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
	
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
	    search(&config.query, &contents)
	} else {
	    search_case_insensitive(&config.query, &contents)
	};
    
	for line in results {
	    println!("{}", line);
	}
    
	Ok(())
}

/// Search the term 'query' through the contents and return all matches
/// 
/// ## Example
/// 
/// ```
/// use minigrep_macano953 as minigrep;
/// let query = "rust";
/// let contents = "rust is great";
/// let result = minigrep::search(query, contents);
/// assert_eq!(vec!["rust is great"], result);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}

/// Case-insensitive search the term 'query' through the contents and return all matches
/// 
/// ## Example
/// 
/// ```
/// use minigrep_macano953 as minigrep;
/// let query = "rUsT";
/// let contents = "Rust is great";
/// let result = minigrep::search_case_insensitive(query, contents);
/// assert_eq!(vec!["Rust is great"], result);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let q = query.to_lowercase();
	contents.lines()
		.filter(|line| line.to_lowercase().contains(&q))
		.collect()
}

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
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}