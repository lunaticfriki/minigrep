use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // we return a Result because we want to return an error if the number of arguments is not correct
    // we use the &str type because we don't need to own the data
    // we don't need to return a panic! because is a user error, not an application error
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // first, we check if the number or args is correct
        if args.len() < 3 {
            return Err(" NOT ENOUGH ARGUMENTS !!!");
        }

        // we don't care about the first index [0], which is the binary path
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
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

#[cfg(test)]
mod tests {
    use super::*; // we bring the parent scope into the current scope

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
