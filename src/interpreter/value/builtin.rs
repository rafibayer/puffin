use std::collections::HashMap;
use std::process;

use crate::interpreter::InterpreterError;

use super::Value;

pub struct Builtin(fn(Vec<Value>) -> Result<Value, InterpreterError>);

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Builtin Function>")
    }
}

impl Clone for Builtin {
    fn clone(&self) -> Self {
        Builtin(self.0.clone())
    }
}

// todo: replace with static or lazy-static
pub fn get_builtins() -> HashMap<String, Value> {
    let builtins = vec![
        ("PI", Value::from(std::f64::consts::PI)),
        ("len", Value::from(Builtin(builtin_len))),
        ("print", Value::from(Builtin(builtin_print))),
        ("println", Value::from(Builtin(builtin_println))),
        ("error", Value::from(Builtin(builtin_error))),
    ];
    builtins
        .into_iter()
        .map(|kv| (kv.0.to_string(), kv.1))
        .collect()
}

fn builtin_len(v: Vec<Value>) -> Result<Value, InterpreterError> {
    if v.len() != 1 {
        return Err(InterpreterError::ArgMismatch {
            name: "len".to_string(),
            expected: 1,
            got: v.len(),
        });
    }

    let arg = &v[0];
    match arg {
        Value::String(s) => Ok(Value::from(s.len() as f64)),
        Value::Array(a) => Ok(Value::from(a.len() as f64)),
        Value::Structure(s) => Ok(Value::from(s.len() as f64)),
        _ => Err(InterpreterError::UnexpectedType(format!("{:?}", arg))),
    }
}

fn builtin_print(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| print!("{:?} ", e));
    Ok(Value::Null)
}

fn builtin_println(v: Vec<Value>) -> Result<Value, InterpreterError> {
    output(v, |e| println!("{:?} ", e));
    Ok(Value::Null)
}

fn builtin_error(v: Vec<Value>) -> Result<Value, InterpreterError> {
    eprintln!("Fatal Error: ");
    output(v, |e| eprintln!("{:?} ", e));
    process::exit(1);
}

fn output<F>(v: Vec<Value>, f: F) where F: Fn(&Value) {
    // todo: fencepost
    for e in &v {
        f(e);
    }
}
