use puffin::interpreter::value::Value;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = puffin::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument Parsing Error: {}", err);
        process::exit(1);
    });

    let value =puffin::run(config);
    if let Value::Null = value {
        return;
    }
    println!("{}", value)

    // match puffin::run(config) {
        
        // Ok(value) => {
            
        // }
        // Err(e) => {
        //     eprintln!("Error: {}", e.to_string());
        //     process::exit(1);
        // }
    // }
}


#[test]
fn run_src() {
    let cfg = puffin::Config {
        filename: "src.puf".to_string(),
        show_parse: false,
        show_ast: false,
    };

    let res = puffin::run(cfg);
    println!("{}", res);
}
