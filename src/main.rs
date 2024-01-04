use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    //  Takes the arguments and turn them into a collection.
    //  We specify the type because collect wants to know what type of collection we want.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // we can exit the program without panicking
        // with unwrap_or_else we can specify a closure to call in case of error,
        // and if it is ok, we return the value stored in ok
        println!("!!! PROBLEM PARSING ARGUMENTS: {}", err);
        process::exit(1);
    });

    println!("SEARCHING FOR {}", config.query);
    println!("IN FILE {}", config.filename);

    if let Err(e) = run(config) {
        println!("!!! APPLICATION ERROR: {} !!!", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("WITH TEXT:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // we retrun a Result because we want to return an error if the number of arguments is not correct
    // we use the &str type because we don't need to own the data
    // we don't need to return a panic! because is a user error, not an application error
    fn new(args: &[String]) -> Result<Config, &str> {
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
