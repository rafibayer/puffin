use value::{Value, Environment};

pub mod value;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    UnboundName(String)
}


