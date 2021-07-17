// need this to read from command line
use std::fs;
use std::error::Error;
use std::env;


// We were still splitting tuple as soon as we parsed configs
// This is a sign that we can have a better structure for grouping!
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    query: String,
    file_name: String,
    case_insensitive: bool,
}

// implementing the parse function on Config to structure better
impl Config{
    // used to be parse_cfg: we're just structuring code together!
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // Want to let main know reason for failure
           return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        // is_err() true if this is unset; so we want case sensitive search, if it's set
        // then we want case_insensitive search
        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_insensitive,
        })
    }
}

// We want everything other than setting up and error handling to be here
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    // Read contents of the text file argument; make use of ? for Result handling
    let contents = fs::read_to_string(cfg.file_name)?;
    let results = if cfg.case_insensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

// We want the string slice(s) lifetime and return content to be connected, because our result's
// refs only make sense if the actual contents are still alive
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // to store the slices to return

    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str
    )-> Vec<&'a str>{
    // To store lines that we wish to return
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_test_config() -> Config{
        Config {
            query: String::from("test"),
            file_name: String::from("poem.txt"),
        }
    }

    #[test]
    fn test_config_constructor_correct() {
        let args = vec!["".to_string(), "test".to_string(), "poem.txt".to_string()];
        let cfg = match Config::new(&args) {
            Ok(config) => config,
            Err(_) => Config{query:"".to_string(), file_name: "".to_string()}
        };

        assert_eq!(cfg, gen_test_config());
    }

    #[test]
    fn test_config_constructor_incorrect() {
        let args = Vec::new();
        if let Err(s) = Config::new(&args) {
            assert!(s.contains("enough arguments"));
        }
    }

    // TDD -- define failing function first
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
            vec!("Rust:", "Trust me."),
            search_case_insensitive(query, contents)
        );
    }
}
