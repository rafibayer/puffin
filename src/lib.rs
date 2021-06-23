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
use std::fs;

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
    let contents = fs::read_to_string(config.filename)
        .expect("Read failed.");
    let parsed = PuffinParser::parse(Rule::program, &contents)
        .expect("Parse failed.");
    let program = ast::build_program(parsed.into_iter().next().unwrap())
        .expect("AST failed.");
    interpreter::eval(program).expect("exec failed.")
}