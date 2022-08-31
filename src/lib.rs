use std::fs; 
use std::error::Error;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive
Pick three. 
        ";

        assert_eq!(search(&query, &contents), vec!["safe, fast, productive"]);
    }

    #[test]
    fn different_result() {
        let query = "three";
        let contents = "\
Rust: 
safe, fast, productive
Pick three.
Duct tape.";

        assert_eq!(search(&query, &contents), vec!["Pick three."]);
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
            search_case_insensitive(&query, &contents)
            );
    }
}

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str
    ) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
} 

pub struct Config {
    pub query: String, 
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(s) => s, 
            None => return Err("No query string specified")
        };
        let file_path = match args.next() {
            Some(fp) => fp,
            None => return Err("No file path specified")
        };
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        match args.next() {
            Some(ic) => {
                let ic = &ic[..];
                match ic {
                    "true" => ignore_case = true,
                    "false" => ignore_case = false,
                    other => return Err("Only 2 valid options true/false
                        for ignoring case")
                }
            },
            None => (),
        }

        Ok(
            Config {
                query, 
                file_path,
                ignore_case
            })
     }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }
    Ok(())
}
