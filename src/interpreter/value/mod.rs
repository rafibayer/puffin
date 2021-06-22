use std::fmt::{Debug};
use std::{collections::HashMap};

use crate::ast::node::*;
use super::InterpreterError;

mod builtin;
pub mod environment;
pub use environment::Environment;

pub struct Builtin(fn(Vec<Value>) -> Result<Value, InterpreterError>);

impl Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Builtin Function>")
    }
}

impl Clone for Builtin {
    fn clone(&self) -> Self {
        Builtin(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Num(f64),
    String(String),
    Array(Vec<Value>),
    Structure(HashMap<String, Value>),
    Function{args: Vec<String>, block: Block, env: Environment},
    Builtin(Builtin),
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Num(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}

impl From<Builtin> for Value {
    fn from(v: Builtin) -> Self {
        Value::Builtin(v)
    }
}