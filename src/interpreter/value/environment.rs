use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

use super::{builtin, InterpreterError, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
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
        Environment { parent: None, bindings, builtins }
    }

    pub fn new_sub(parent: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            parent: Some(parent.clone()),
            bindings: HashMap::new(),
            builtins: HashSet::new(),
            
        }
    }

    pub fn bind(&mut self, name: &str, value: Value) -> Result<Value, InterpreterError> {
        if self.builtins.contains(name) {
            return Err(InterpreterError::BuiltinRebinding(name.to_string()));
        }

        // bind or rebind
        if let Some(val) = self.bindings.get_mut(name) {
            *val = value
        } else {
            self.bindings.insert(name.to_string(), value);
        }

        Ok(Value::Null)
    }

    pub fn get(&self, name: &str) -> Result<Value, InterpreterError> {
        match self.bindings.get(name) {
            Some(value) => Ok(value.clone()),
            None => {
                match &self.parent {
                    Some(parent) => parent.borrow().get(name),
                    None => Err(InterpreterError::UnboundName(name.to_string())),
                }
            },
        }
    }
}
