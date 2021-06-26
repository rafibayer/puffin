use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::rc::Rc;

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
    Array(Rc<RefCell<Vec<Value>>>),
    Structure(Rc<RefCell<HashMap<String, Value>>>),
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
                    &v.borrow().iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                write!(f, "{}]", buf)
            }
            Value::Structure(s) => {
                let mut buf = String::from("{");
                buf.push_str(
                    &s.borrow().iter()
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


impl TryInto<Rc<RefCell<Vec<Value>>>> for Value {
    type Error = InterpreterError;

    fn try_into(self) -> Result<Rc<RefCell<Vec<Value>>>, Self::Error> {
        match self {
            Value::Array(arr) => Ok(arr),
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
        Value::Array(Rc::new(RefCell::new(v)))
    }
}

impl From<HashMap<String, Value>> for Value {

    fn from(v: HashMap<String, Value>) -> Self {
        Value::Structure(Rc::new(RefCell::new(v)))
    }
}

impl From<Builtin> for Value {
    fn from(v: Builtin) -> Self {
        Value::Builtin(v)
    }
}
