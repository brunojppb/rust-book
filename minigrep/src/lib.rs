//! # minigrep
//!
//! `minigrep` is a collection of utilities to search for text
//! within the given file. It's a super minimal version of the `grep`
//! CLI tool.
//! 
use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // Skip binary path arg
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Undefined query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Undefined file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
///
/// Search for the given query in the given content string.
/// 
/// # Examples
/// 
/// ```
/// use minigrep;
/// 
/// let contents = "\
/// Rust, the programming language
/// powerful and fast
/// Rust shines without a sweat.
/// ";
/// let query = "Rust";
/// let result = minigrep::search(query, contents);
/// 
/// assert_eq!(
///   vec![
///     "Rust, the programming language",
///     "Rust shines without a sweat."
///   ],
///  result);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
      .lines()
      .filter(|line| line.contains(query))
      .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut search_results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            search_results.push(line);
        }
    }

    search_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "RusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
