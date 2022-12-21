use std::error::Error;
use std::env;
use std::fs;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    for filename in config.filenames {
        let contents = fs::read_to_string(&filename)?;

        let results = search(&config.query, &contents, config.ignore_case);

        if results.len() > 0 {
            println!("{}:", filename);
        }

        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filenames: Vec<String>,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let mut filenames: Vec<String> = vec![
            match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            },
        ];

        loop {
            match args.next() {
                Some(arg) => filenames.push(arg),
                None => break,
            }
        };

        let ignore_case = env::var("IGNORE_CASE").unwrap_or_else(|_| {
            String::from("0")
        });
        let ignore_case = ignore_case == String::from("1");

        Ok(Config { query, filenames, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| (if case { line.to_lowercase() } else { line.to_string() }).contains(query))
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
