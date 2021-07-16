// need this to read from command line
use std::fs;
use std::error::Error;


// We were still splitting tuple as soon as we parsed configs
// This is a sign that we can have a better structure for grouping!
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    query: String,
    file_name: String
}

// implementing the parse function on Config to structure better
impl Config{
    // used to be parse_cfg: we're just structuring code together!
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // Want to let main know reason for failure
           return Err("Not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

// We want everything other than setting up and error handling to be here
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    // Read contents of the text file argument; make use of ? for Result handling
    let contents = fs::read_to_string(cfg.file_name)?;

    println!("Text we read:\n {}", contents);

    Ok(())
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
}
