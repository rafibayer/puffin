use std::collections::{HashMap, HashSet};

use super::{builtin, InterpreterError, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    // local bindings
    bindings: HashMap<String, Value>,
    // builtin names, can't be rebound
    builtins: HashSet<String>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Environment {
        // get_builtins and the builtins hashset should probably both be static/lazy & cached
        let bindings = builtin::get_builtins();
        let builtins = bindings.keys().cloned().collect();
        Environment { bindings, builtins }
    }

    pub fn bind(&mut self, name: String, value: Value) -> Result<Value, InterpreterError> {
        if self.builtins.contains(&name) {
            return Err(InterpreterError::BuiltinRebinding(name));
        }
        self.bindings.insert(name, value);
        Ok(Value::Null)
    }

    pub fn get(&self, name: &str) -> Result<Value, InterpreterError> {
        match self.bindings.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(InterpreterError::UnboundName(name.to_string())),
        }
    }
}
