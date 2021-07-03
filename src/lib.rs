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

extern crate cached;

pub mod parser;
pub mod ast;
pub mod interpreter;
pub mod repl;
use std::{fs, process};

use interpreter::value::Value;
pub use parser::{Rule, PuffinParser};
pub use pest::Parser;

/// Puffin Run Config.
/// Includes the filename of the program, as well as any optional flags
pub struct Config {
    pub filename: String,
    pub show_parse: bool,
    pub show_ast: bool,
}

impl Config {
    /// Create a Config `from std::env::args()`.
    /// Note: expects that first argument is the puffin interpreter executable.
    /// Example args: `["./puffin", "program.puf", "-ast", "-parse"]`
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("Required Arguments: filename".to_string());
        }

        let filename = args[1].clone();
        let mut show_parse = false;
        let mut show_ast = false;
        
        // parse optional flags
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

/// Runs a puffin program given a Config.
/// Returns the program final output if successful.
/// Prints errors to stderr and exits with non-zero exit code if errors are encountered.
pub fn run(config: Config) -> Value {
    let contents = fs::read_to_string(config.filename.clone()).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {:#?}", err);
        process::exit(1);
    });
    
    let parsed = PuffinParser::parse_program(&contents).unwrap_or_else(|err| {
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