use std::env;
use std::process;

use minigrep::Config;

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

    if let Err(e) = minigrep::run(config) {
        println!("!!! APPLICATION ERROR: {} !!!", e);
        process::exit(1);
    }
}
