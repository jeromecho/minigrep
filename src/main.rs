use std::env;
use std::process;

use::minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error building config - {err}");
        process::exit(1);
    });

    if let Err(err) =  minigrep::run(config) {
        eprintln!("Error running function 'run' - {err}");
        process::exit(1);
    }
}


