use std::env;
use std::process;

use minigrep_macano953 as minigrep;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}