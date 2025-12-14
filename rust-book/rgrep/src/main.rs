use std::env;
use std::fs;
use std::process;

use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // &args[0] is the name of the program (target/debug/rgrep)
    // &args[1] is the query
    // &args[2] is the file path
    // let query = &args[1];
    // let file_path = &args[2];

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
