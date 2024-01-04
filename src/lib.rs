use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("WITH TEXT:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // we retrun a Result because we want to return an error if the number of arguments is not correct
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
