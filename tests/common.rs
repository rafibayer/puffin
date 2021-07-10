//! Author: Rafael Bayer (2021)
//! This module contains common test code for Puffin Integration tests. 
//!
//! `run_program` is used to easily run a program from a passed str.

pub use puffin::{
    ast::{self, node::*},
    interpreter::{self, value::Environment, Value},
    parser, Parser,
};

/// run_program executes a Puffin program in a given str,
/// returning the resulting value.
/// Panics if the parser, AST generator, or interpreter encounter any error.
pub fn run_program(program: &str) -> Value {
    let parsed = parser::PuffinParser::parse_program(program)
        .unwrap()
        .next()
        .unwrap();
    let ast = ast::build_program(parsed).unwrap();
    interpreter::eval(&ast).unwrap()
}
