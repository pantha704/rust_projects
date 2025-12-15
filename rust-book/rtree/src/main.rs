use rtree::{run, Config};
use std::env;
use std::process;

// #![allow(dead_code)]
// This attribute globally disables the `dead_code` lint for the entire crate.
// The `dead_code` lint warns about unused functions, methods, structs, enums,
// and other items. Disabling it can be useful during development when
// code might temporarily be unused, or in libraries where not all public
// items are expected to be used by every consumer.

fn main() {
    let mut args = env::args();

    let config = Config::build(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Target directory: {}\n", config.target_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
