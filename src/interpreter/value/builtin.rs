//! Author: Rafael Bayer (2021)
//! The builtin module defines builtin functions and values in Puffin

use rand;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::rc::Rc;

use crate::interpreter::{unexpected_type, InterpreterError};
use super::Value;

/// Builtin wraps a name and a builtin function body
pub struct Builtin {
    name: &'static str,
    pub body: fn(Vec<Value>) -> Result<Value, InterpreterError>,
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Builtin Function: {}>", self.name)
    }
}

impl Clone for Builtin {
    fn clone(&self) -> Self {
        Builtin {
            name: self.name,
            body: self.body,
        }
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        // all builtins have unique names
        self.name == other.name
    }
}

/// returns all puffin Builtins.
/// Expensive, should only be called once for the global environment.
pub fn get_builtins() -> HashMap<String, Value> {
    // List of builtin functions and constants
    let builtins = vec![
        ("PI", Value::from(std::f64::consts::PI)),
        ("true", Value::from(1f64)),
        ("false", Value::from(0f64)),
        ("EPSILON", Value::from(std::f64::EPSILON)),
        (
            "len",
            Value::from(Builtin {
                name: "len",
                body: builtin_len,
            }),
        ),
        (
            "str",
            Value::from(Builtin {
                name: "str",
                body: builtin_str,
            }),
        ),
        (
            "print",
            Value::from(Builtin {
                name: "print",
                body: builtin_print,
            }),
        ),
        (
            "println",
            Value::from(Builtin {
                name: "println",
                body: builtin_println,
            }),
        ),
        (
            "error",
            Value::from(Builtin {
                name: "error",
                body: builtin_error,
            }),
        ),
        (
            "sin",
            Value::Builtin(Builtin {
                name: "sin",
                body: |v| builtin_floatops(v, f64::sin),
            }),
        ),
        (
            "cos",
            Value::Builtin(Builtin {
                name: "cos",
                body: |v| builtin_floatops(v, f64::cos),
            }),
        ),
        (
            "tan",
            Value::Builtin(Builtin {
                name: "tan",
                body: |v| builtin_floatops(v, f64::tan),
            }),
        ),
        (
            "sqrt",
            Value::Builtin(Builtin {
                name: "sqrt",
                body: |v| builtin_floatops(v, f64::sqrt),
            }),
        ),
        (
            "abs",
            Value::Builtin(Builtin {
                name: "abs",
                body: |v| builtin_floatops(v, f64::abs),
            }),
        ),
        (
            "round",
            Value::Builtin(Builtin {
                name: "round",
                body: |v| builtin_floatops(v, f64::round),
            }),
        ),
        (
            "pow",
            Value::Builtin(Builtin {
                name: "pow",
                body: builtin_pow
            })
        ),
        (
            "input_str",
            Value::Builtin(Builtin {
                name: "input_str",
                body: |v| builtin_input(v, InputType::String),
            }),
        ),
        (
            "input_num",
            Value::Builtin(Builtin {
                name: "input_num",
                body: |v| builtin_input(v, InputType::Num),
            }),
        ),
        (
            "push",
            Value::Builtin(Builtin {
                name: "push",
                body: builtin_push,
            }),
        ),
        (
            "pop",
            Value::Builtin(Builtin {
                name: "pop",
                body: builtin_pop,
            }),
        ),
        (
            "remove",
            Value::Builtin(Builtin{
                name: "remove",
                body: builtin_remove,
            })
        ),
        (
            "insert",
            Value::Builtin(Builtin{
                name: "insert",
                body: builtin_insert,
            })
        ),
        (
            "rand",
            Value::Builtin(Builtin {
                name: "rand",
                body: builtin_rand,
            }),
        ),
    ];
    builtins
        .into_iter()
        .map(|kv| (kv.0.to_string(), kv.1))
        .collect()
}

/// converts `a` into a string
fn builtin_str(v: Vec<Value>) -> Result<Value, InterpreterError> {
    let arg = get_one(v)?;
    Ok(Value::String(arg.to_string()))
}

/// Returns the length of a string, array, or structure
fn builtin_len(v: Vec<Value>) -> Result<Value, InterpreterError> {
    let arg = get_one(v)?;
    match arg {
        Value::String(s) => Ok(Value::from(s.len() as f64)),
        Value::Array(a) => Ok(Value::from(a.borrow().len() as f64)),
        Value::Structure(s) => Ok(Value::from(s.borrow().len() as f64)),
        _ => Err(unexpected_type(arg.clone())),
    }
}

/// prints args
fn builtin_print(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| print!("{} ", e));
    Ok(Value::Null)
}

/// v: 
fn builtin_println(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| println!("{}", e));
    Ok(Value::Null)
}

/// printlns args to stderr, and returns an InterpreterError
fn builtin_error(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| eprintln!("ERR: {} ", e));
    Err(InterpreterError::Error)
}

fn output<F>(v: Vec<Value>, f: F)
where
    F: Fn(String),
{
    f(v.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" "));
}

/// Used to create single argument math builtins:
#[inline]
fn builtin_floatops<F>(v: Vec<Value>, f: F) -> Result<Value, InterpreterError>
where
    F: Fn(f64) -> f64,
{
    let arg = get_one(v)?;
    let float: f64 = arg.try_into()?;
    Ok(Value::from(f(float)))
}

fn builtin_pow(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(2, &v)?;
    let exp: f64 = v.pop().unwrap().try_into()?;
    let base: f64 = v.pop().unwrap().try_into()?;

    Ok(Value::from(base.powf(exp)))
}

enum InputType {
    String,
    Num,
}

/// Used to create builtins
fn builtin_input(v: Vec<Value>, input_type: InputType) -> Result<Value, InterpreterError> {
    // print any args as a prompt
    builtin_print(v)?;
    // flush stdout so prompt appears first
    io::Write::flush(&mut io::stdout())?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf = buf.trim_end().to_string();

    Ok(match input_type {
        InputType::String => Value::String(buf),
        InputType::Num => {
            let parsed: f64 = if let Ok(n) = buf.parse() {
                n
            } else {
                
                return Err(InterpreterError::IOError(
                    "Failed to parse number".to_string(),
                ));
            };

            Value::Num(parsed)
        }
    })
} 

/// Push `b` onto array `a`
fn builtin_push(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(2, &v)?;

    let value = v.pop().unwrap();
    let array: Rc<RefCell<Vec<Value>>> = v.pop().unwrap().try_into()?;
    array.borrow_mut().push(value);
    Ok(Value::Array(array))
}

/// Pop from array `a`
fn builtin_pop(v: Vec<Value>) -> Result<Value, InterpreterError> {
    let array: Rc<RefCell<Vec<Value>>> = get_one(v)?.try_into()?;
    if array.borrow().len() == 0 {
        return Err(InterpreterError::BoundsError{ index: 0, size: 0 })
    }
    let removed = array.borrow_mut().pop().unwrap();
    Ok(removed)
}

/// Remove from array `a` at index `i`
fn builtin_remove(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(2, &v)?;

    let index_float: f64 = v.pop().unwrap().try_into()?;
    let index = index_float as usize;
    let array: Rc<RefCell<Vec<Value>>> = v.pop().unwrap().try_into()?;
    if index >= array.borrow().len()  {
        return Err(InterpreterError::BoundsError { index, size: array.borrow().len() })
    }

    let removed = array.borrow_mut().remove(index);
    Ok(removed)
}

/// inserts element `v` at index `i` in array `a`
fn builtin_insert(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(3, &v)?;

    let value = v.pop().unwrap();

    let index_float: f64 = v.pop().unwrap().try_into()?;
    let index = index_float as usize;

    let array: Rc<RefCell<Vec<Value>>> = v.pop().unwrap().try_into()?;

    if index > array.borrow().len() {
        return Err(InterpreterError::BoundsError { index, size: array.borrow().len() })
    }

    array.borrow_mut().insert(index, value);

    Ok(Value::Null)
}


/// Return a random number in [0, 1)
fn builtin_rand(v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(0, &v)?;
    Ok(Value::Num(rand::random()))
}

/// Gets exactly 1 argument from v
#[inline]
fn get_one(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(1, &v)?;
    Ok(v.pop().unwrap())
}


/// Checks that v has exactly the expect number of elements, returning
/// and InterpreterError otherwise
#[inline]
fn expect_args<T>(n: usize, v: &[T]) -> Result<(), InterpreterError> {
    if v.len() != n {
        return Err(InterpreterError::ArgMismatch {
            expected: n,
            got: v.len(),
        });
    }

    Ok(())
}
