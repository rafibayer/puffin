use value::{Environment, Value};

pub mod value;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    UnboundName(String),
    ArgMismatch {
        name: String,
        expected: usize,
        got: usize,
    },
    UnexpectedType(String),
    BuiltinRebinding(String)
}
