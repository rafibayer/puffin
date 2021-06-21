use crate::ast::node;
use value::{Value, Environment};

pub mod value;

pub enum InterpreterError {
    UnboundName(String)
}


pub fn eval(program: node::Program) -> Result<Value, InterpreterError> {
    todo!()
}
