//! Author: Rafael Bayer (2021)
//! Main Entrypoint of the Puffin Interpreter Binary

use puffin::{interpreter::value::Value, repl};
use std::{env, process};

/// Main with no arguments begins a Puffin REPL session.
/// If a filepath is passed, the interpreter will instead execute the contained program,
/// outputting a non-null top-level return to stdout. The interpreter program will terminate
/// when the program does.
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // if no additional arguments are passed, star the Puffin REPL
    if args.len() == 1 {
        repl::start_repl();
    }

    // otherwise, we parse the args into config, and run
    let config = puffin::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument Parsing Error: {}", err);
        process::exit(1);
    });

    let value = puffin::run(config);
    // if program value is null, we don't print it
    if let Value::Null = value {
        return;
    }
    println!("{}", value)

}
