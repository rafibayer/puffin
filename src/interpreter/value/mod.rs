//! Author: Rafael Bayer (2021)
//! The value module defines types and functions relating to 
//! values in the `puffin` programming language. This includes the
//! `Value` enum, that acts as the underlying container for all types.

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

/// Value holds the data of `puffin` types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Puffin Null, implict return of functions and blocks
    Null,
    /// Puffin Number
    Num(f64),
    /// Puffin String
    String(String),
    /// Puffin Array
    Array(Rc<RefCell<Vec<Value>>>),
    /// Puffin Structure
    Structure(Rc<RefCell<HashMap<String, Value>>>),
    /// Puffin Closure
    Closure {
        kind: ClosureKind,
        args: Vec<String>,
        block: Block,
        environment: Rc<RefCell<Environment>>,
    },
    /// Puffin Builtin function
    Builtin(Builtin),
}

/// ClosureKind defines the type of a `puffin` closure
#[derive(Debug, Clone, PartialEq)]
pub enum ClosureKind {
    /// Anonymous, not assigned to a name
    Anonymous,
    /// Structure receiver, holds reference to structure which will be implicit first argument
    Receiver(Rc<RefCell<HashMap<String, Value>>>),
    /// Named function, name bound to closure in closures environment
    Named(String),
}

/// Circular refrence display
const CIRCULAR_REF: &str = "...";

/// Display of all `Puffin` Values
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Num(n) => write!(f, "{}", n),
            Value::String(s) => {
                if f.alternate() {
                    return write!(f, "{}", s);
                }
                write!(f, "\'{}\'", s)
            },
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
                    return write!(f, "<{} fn({})> ", name, argstr);
                } else if let ClosureKind::Receiver(_) = &kind {
                    return write!(f, "<(self) fn({})>", argstr);
                }
                // anonymous function/lambda
                write!(f, "<λ fn({})> ", argstr)
            }
            Value::Builtin(b) => {
                write!(f, "{:?}", b)
            }
        }
    }
}

/// safely stringify's the contents of a puffin `Array`.
/// checks for circular references, replacing them with a constant string
// todo: possible to make this generic and combine with method below?
pub fn stringify_array(array: &Rc<RefCell<Vec<Value>>>, seen: &mut HashSet<usize>) -> String {
    seen.insert(array.as_ptr() as usize);

    let result = array
        .borrow()
        .iter()
        .map(|element| match element {
            Value::Array(inner) => {
                if seen.contains(&(inner.as_ptr() as usize)) {
                    format!("[{}]", CIRCULAR_REF)
                } else {
                    stringify_array(inner, seen)
                }
            }
            Value::Structure(inner) => {
                if seen.contains(&(inner.as_ptr() as usize)) {
                    format!("{{{}}}", CIRCULAR_REF)
                } else {
                    stringify_struct(inner, seen)
                }
            }
            other => other.to_string(),
        })
        .collect::<Vec<String>>()
        .join(", ");

    format!("[{}]", result)
}

/// safely stringify's the contents of a puffin `Structure`.
/// checks for circular references, replacing them with a constant string
pub fn stringify_struct(
    structure: &Rc<RefCell<HashMap<String, Value>>>,
    seen: &mut HashSet<usize>,
) -> String {
    seen.insert(structure.as_ptr() as usize);

    let result = structure
        .borrow()
        .iter()
        .map(|(name, element)| match element {
            Value::Structure(inner) => {
                if seen.contains(&(inner.as_ptr() as usize)) {
                    format!("{}: {{{}}}", name, CIRCULAR_REF)
                } else {
                    format!("{}: {}", name, stringify_struct(inner, seen))
                }
            }
            Value::Array(inner) => {
                if seen.contains(&(inner.as_ptr() as usize)) {
                    format!("{}: [{}]", name, CIRCULAR_REF)
                } else {
                    format!("{}: {}", name, stringify_array(inner, seen))
                }
            }
            other => format!("{}: {}", name, other),
        })
        .collect::<Vec<String>>()
        .join(", ");

    format!("{{{}}}", result)
}

impl TryInto<f64> for Value {
    
    type Error = InterpreterError;
    
    /// Converts a Num `Value` into an f64.
    /// Returns an interpreter error of the value is not the correct variant.
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Num(n) => Ok(n),
            _ => Err(unexpected_type(self)),
        }
    }
}

impl TryInto<String> for Value {
    type Error = InterpreterError;

    /// Converts a String `Value` into an String.
    /// Returns an interpreter error of the value is not the correct variant.
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(unexpected_type(self)),
        }
    }
}

impl TryInto<Rc<RefCell<Vec<Value>>>> for Value {
    type Error = InterpreterError;

    /// Converts a Array `Value` into an String.
    /// Returns an interpreter error of the value is not the correct variant.
    fn try_into(self) -> Result<Rc<RefCell<Vec<Value>>>, Self::Error> {
        match self {
            Value::Array(arr) => Ok(arr),
            _ => Err(unexpected_type(self)),
        }
    }
}

impl From<f64> for Value {
    /// produces a Num `Value` from a f64
    fn from(v: f64) -> Self {
        Value::Num(v)
    }
}

impl From<String> for Value {
    /// produces a String `Value` from a String
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<Vec<Value>> for Value {
    /// produces a Array `Value` from a Vec<Value>
    fn from(v: Vec<Value>) -> Self {
        Value::Array(Rc::new(RefCell::new(v)))
    }
}

impl From<HashMap<String, Value>> for Value {
    /// produces a Structure `Value` from a HashMap<String, Value>
    fn from(v: HashMap<String, Value>) -> Self {
        Value::Structure(Rc::new(RefCell::new(v)))
    }
}

impl From<Builtin> for Value {
    /// Produces a Builtin `Value` from a Builtin struct
    fn from(v: Builtin) -> Self {
        Value::Builtin(v)
    }
}
