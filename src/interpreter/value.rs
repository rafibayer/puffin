use std::{collections::HashMap, rc::Rc};
use super::node;
use super::InterpreterError;


pub struct Environment {
    bindings: HashMap<String, Value>
}

impl Environment {
    pub fn bind(&mut self, name: &str, value: Value) {
        self.bindings.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<Value, InterpreterError> {
        match self.bindings.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(InterpreterError::UnboundName(name.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    String(String),
    Array(Vec<Value>),
    Function{names: Vec<String>, body: node::Block}
}