use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments. Provide a query string and a file path.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("Query: {}", config.query);
    println!("With text: {}", contents);
    Ok(())
}
