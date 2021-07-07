use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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
        kind: ClosureKind,
        args: Vec<String>,
        block: Block,
        environment: Rc<RefCell<Environment>>,
    },
    Builtin(Builtin),
}


#[derive(Debug, Clone, PartialEq)]
pub enum ClosureKind {
    Anonymous,
    Reciever(Rc<RefCell<HashMap<String, Value>>>),
    Named(String),
}

const CIRCULAR_REF: &str = "...";

// Display of all Puffin Types
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Num(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\'{}\'", s),
            Value::Array(v) => {
                write!(f, "{}", stringify_array(v, &mut HashSet::new()))
            }
            Value::Structure(s) => {
                write!(f, "{}", stringify_struct(s, &mut HashSet::new()))

            }
            Value::Closure { args, kind, .. } => {
                let argstr = args.join(", ");
                if let ClosureKind::Named(name) = &kind {
                    // named function
                    return write!(f, "<{} fn({})> ", name, argstr)
                } else if let ClosureKind::Reciever(_) = &kind {
                    return write!(f, "<(self) fn({})>", argstr)
                }
                // anonymous function/lambda 
                write!(f, "<λ fn({})> ", argstr)
            },
            Value::Builtin(b) => {
                write!(f, "{:?}", b)
            }
        }
    }
}


// safely stringify's the contents of a puffin array.
// checks for circular refrences, replacing them with a constant string
// todo: possible to make this generic and combine with method below?
pub fn stringify_array(array: &Rc<RefCell<Vec<Value>>>, seen: &mut HashSet<usize>) -> String {
    seen.insert(array.as_ptr() as usize);

    let result = array
        .borrow()
        .iter()
        .map(|element| {
            match element {
                Value::Array(inner) => {
                    if seen.contains(&(inner.as_ptr() as usize)) {
                        format!("[{}]", CIRCULAR_REF)
                    } else {
                        stringify_array(inner, seen)
                    }
                },
                Value::Structure(inner) => {
                    if seen.contains(&(inner.as_ptr() as usize)) {
                        format!("{{{}}}", CIRCULAR_REF)
                    } else {
                        stringify_struct(inner, seen)
                    }
                }
                other => other.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    format!("[{}]", result)
}

// safely stringify's the contents of a puffin structure.
// checks for circular refrences, replacing them with a constant string
pub fn stringify_struct(structure: &Rc<RefCell<HashMap<String, Value>>>, seen: &mut HashSet<usize>) -> String {
    seen.insert(structure.as_ptr() as usize);

    let result = structure
        .borrow()
        .iter()
        .map(|(name, element)| {
            match element {
                Value::Structure(inner) => {
                    if seen.contains(&(inner.as_ptr() as usize)) {
                        format!("{}: {{{}}}", name, CIRCULAR_REF)
                    } else {
                        format!("{}: {}", name, stringify_struct(inner, seen))
                    }
                },
                Value::Array(inner) => {
                    if seen.contains(&(inner.as_ptr() as usize)) {
                        format!("{}: [{}]", name, CIRCULAR_REF)
                    } else {
                        format!("{}: {}", name, stringify_array(inner, seen))
                    }
                }
                other => format!("{}: {}", name, other)
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    format!("{{{}}}", result)
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
