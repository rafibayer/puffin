use std::collections::HashMap;
use std::convert::TryInto;
use std::io;

use crate::interpreter::{InterpreterError, unexpected_type};
use cached::proc_macro::cached;

use super::Value;

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

#[cached]
pub fn get_builtins() -> HashMap<String, Value> {
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
            Value::Builtin(Builtin{
                name: "sin", body: |v| builtin_floatops(v, f64::sin)
            }),
        ),
        (
            "cos",
            Value::Builtin(Builtin{
                name: "cos", body: |v| builtin_floatops(v, f64::cos)
            }),
        ),(
            "tan",
            Value::Builtin(Builtin{
                name: "tan", body: |v| builtin_floatops(v, f64::tan)
            }),
        ),(
            "sqrt",
            Value::Builtin(Builtin{
                name: "sqrt", body: |v| builtin_floatops(v, f64::sqrt)
            }),
        ),(
            "abs",
            Value::Builtin(Builtin{
                name: "abs", body: |v| builtin_floatops(v, f64::abs)
            }),
        ),(
            "input_str",
            Value::Builtin(Builtin{
                name: "input_str", body: |v| builtin_input(v, InputType::String)
            })
        ),(
            "input_num",
            Value::Builtin(Builtin{
                name: "input_num", body: |v| builtin_input(v, InputType::Num)
            })
        ),
        (
            "push",
            Value::Builtin(Builtin{
                name: "push", body: builtin_push
            })
        ),
        (
            "pop",
            Value::Builtin(Builtin{
                name: "pop", body: builtin_pop
            })
        ),
        
    ];
    builtins
        .into_iter()
        .map(|kv| (kv.0.to_string(), kv.1))
        .collect()
}

fn builtin_len(v: Vec<Value>) -> Result<Value, InterpreterError> {
    let arg = get_one(v)?;
    match arg {
        Value::String(s) => Ok(Value::from(s.len() as f64)),
        Value::Array(a) => Ok(Value::from(a.len() as f64)),
        Value::Structure(s) => Ok(Value::from(s.len() as f64)),
        _ => Err(unexpected_type(arg.clone())),
    }
}

fn builtin_print(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| print!("{} ", e));
    Ok(Value::Null)
}

fn builtin_println(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| println!("{} ", e));
    Ok(Value::Null)
}

fn builtin_error(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| eprintln!("ERR: {} ", e));
    Err(InterpreterError::RuntimeError)
}

fn output<F>(v: Vec<Value>, f: F)
where
    F: Fn(&Value),
{
    // todo: fencepost
    for e in &v {
        f(e);
    }
}

#[inline]
fn builtin_floatops<F>(v: Vec<Value>, f: F) -> Result<Value, InterpreterError>
where F: Fn(f64) -> f64 {
    let arg = get_one(v)?;
    let float: f64 = arg.try_into()?;
    Ok(Value::from(f(float)))
}

enum InputType {
    String,
    Num
}

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
                return Err(InterpreterError::IOError("Failed to parse number".to_string()))
            };

            Value::Num(parsed)
        },
    })
}

fn builtin_push(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(2, &v)?;

    let value = v.pop().unwrap();
    let mut array: Vec<Value> = v.pop().unwrap().try_into()?;

    array.push(value);

    Ok(Value::Array(array))
}



fn builtin_pop(v: Vec<Value>) -> Result<Value, InterpreterError> {
    let mut array: Vec<Value> = get_one(v)?.try_into()?;

    let removed = array.pop().unwrap();

    Ok(Value::Structure(
        vec![("removed", removed), ("array", Value::Array(array))]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    ))
}

#[inline]
fn get_one(mut v: Vec<Value>) -> Result<Value, InterpreterError> {
    expect_args(1, &v)?;
    Ok(v.remove(0))
}

#[inline]
fn expect_args<T>(n: usize, v: &[T]) -> Result<(), InterpreterError> {
    if v.len() != n {
        return Err(InterpreterError::ArgMismatch{ expected: n, got: v.len() })
    }

    Ok(())
}