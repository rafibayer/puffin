

pub use puffin::{
    ast::{self, node::*},
    interpreter::{self, value::Environment, Value},
    parser, Parser,
};

pub fn run_program(program: &str) -> Value {
    let parsed = parser::PuffinParser::parse(puffin::Rule::program, program)
        .unwrap()
        .next()
        .unwrap();
    let ast = ast::build_program(parsed).unwrap();
    interpreter::eval(ast).unwrap()
}
