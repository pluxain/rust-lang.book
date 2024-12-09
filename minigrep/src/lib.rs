use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &file_content) {
        log::info!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_result() {
        let query = "Java";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected: Vec<&str> = Vec::new();
        assert_eq!(expected, search(query, content));
    }

    #[test]
    fn case_sensitive_one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_sensitive_two_results() {
        let query = "st";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        let expected = vec!["Rust:", "safe, fast, productive."];
        assert_eq!(expected, search(query, content));
    }

    #[test]
    fn case_insensitive_one_result() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        )
    }
}
