/*
-----------------------------------------------------------
Welcome to the Puffin Language Interpreter!
                            ,▄▄▄▄,
                        ▄██████████▄
                        ,███ ▀██ ▀██████▄
                        ███████████▓██████
                        ██████████▌ ▀▀▀▀▀▀
                        ████████████▄
                    ▄████████████████▄
                ▄▄████████████████████
            ,▄████████████████████████`
            ██████████████████████████▀
            ┌██████████████████████████`
            █████████████████████████▀
        ▄████████████████████████▀
        ,▄██████████████████████▀
        ▄█████████████████████▀
    ▀▀▀▀`         ███  ▐██▀
                    ▀▀▌  ▀▌
-----------------------------------------------------------
*/

#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod ast;
pub mod interpreter;
use std::{fs, process};

use interpreter::value::Value;
pub use parser::{Rule, PuffinParser};
pub use pest::Parser;

pub struct Config {
    pub filename: String,
    pub show_parse: bool,
    pub show_ast: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("Required Arguments: filename".to_string());
        }

        let filename = args[1].clone();
        let mut show_parse = false;
        let mut show_ast = false;
        
        for option in args.iter().skip(2) {
            match option.to_lowercase().as_str() {
                "-parse" => {
                    show_parse = true;
                },
                "-ast" => {
                    show_ast = true;
                },
                _ => return Err(format!("Unknown option: {}", option))
            }
        }

        Ok(Config {
            filename,
            show_parse,
            show_ast,
        })
    }
}

pub fn run(config: Config) -> Value {
    let contents = fs::read_to_string(config.filename.clone()).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {:#?}", err);
        process::exit(1);
    });
    
    let parsed = PuffinParser::parse(Rule::program, &contents).unwrap_or_else(|err| {
        eprintln!("Parser Error: {}", parser::line_col(err));
        process::exit(1);
    });
    if config.show_parse {
        println!("{} parse:\n{:#?}", config.filename, &parsed);
    }
    let program = ast::build_program(parsed.into_iter().next().unwrap()).unwrap_or_else(|err| {
        eprintln!("AST Error: {:#?}", err);
        process::exit(1);
    });
    if config.show_ast {
        println!("{} ast:\n{:#?}", config.filename, &program);
    }
    interpreter::eval(program).unwrap_or_else(|err| {
        eprintln!("Runtime Error: {:#?}", err);
        process::exit(1);
    })
}