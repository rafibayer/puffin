use puffin::{interpreter::value::Value, repl};
use std::{env, process};

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
