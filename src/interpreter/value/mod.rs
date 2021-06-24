use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Debug, Display};

use super::InterpreterError;
use crate::ast::node::*;
use crate::interpreter::unexpected_type;

mod builtin;
pub mod environment;
pub use environment::Environment;

use builtin::Builtin;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Num(f64),
    String(String),
    Array(Vec<Value>),
    Structure(HashMap<String, Value>),
    Closure {
        self_name: Option<String>,
        args: Vec<String>,
        block: Block,
        environment: Environment,
    },
    Builtin(Builtin),
}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Num(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(v) => {
                let mut buf = String::from("[");
                buf.push_str(
                    &v.iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                write!(f, "{}]", buf)
            }
            Value::Structure(s) => {
                let mut buf = String::from("{");
                buf.push_str(
                    &s.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                write!(f, "{}}}", buf)
            }
            Value::Closure { args, self_name, .. } => {
                let argstr = args.join(", ");
                if let Some(name) = self_name {
                    return write!(f, "<{} fn({})> ", name, argstr)

                }
                write!(f, "<λ fn({})> ", argstr)
            },
            Value::Builtin(b) => {
                write!(f, "{:?}", b)
            }
        }
    }
}

impl TryInto<f64> for Value {
    type Error = InterpreterError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Num(n) => Ok(n),
            _=> Err(unexpected_type(self))
        }
    }
}

impl TryInto<String> for Value {
    type Error = InterpreterError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            _=> Err(unexpected_type(self))
        }
    }
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
