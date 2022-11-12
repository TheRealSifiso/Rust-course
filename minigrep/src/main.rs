use ::std::env; //env::args() function will return an iterator of the
                //command line arguments passed into minigrep

use std::process;

use minigrep::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

//eprintln! macro - prints to the standard error stream
//println! marco - prints to the standard output stream
