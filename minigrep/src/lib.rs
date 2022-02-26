use std::error::Error;
use std::{env, fs};

pub mod hello;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn new_(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first argument
        args.next();

        let qry = match args.next() {
            None => Err("no query string"),
            Some(arg) => Ok(arg),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => panic!("no filename"),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            query: qry.unwrap(),
            filename,
            case_sensitive,
        })
    }
}

pub fn run(con: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(con.filename).expect("Error reading file");

    let results = if con.case_sensitive {
        search(&con.query, &contents)
    } else {
        search_case_insensitive(&con.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
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

pub fn search_<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// put in lib
#[cfg(test)]
mod tests {
    // use all definition in library
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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
