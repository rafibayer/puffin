//! Author: Rafael Bayer (2021)
//! The environment module defines the environment structure.
//! This structure binds names to values in the Puffin language.

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{builtin, InterpreterError, Value};

/// Environment maps between names and values
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    // parent environment, global environment has None parent
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
    // used as a placeholder for type const builtins
    pub fn empty() -> Environment {
        Environment {
            parent: None,
            bindings: HashMap::new(),
            builtins: HashSet::new(),
        }
    }

    /// Returns a new Environment, filling it with Builtin values
    pub fn new() -> Environment {
        // get_builtins and the builtins hashset should probably both be static/lazy & cached
        let bindings = builtin::get_builtins();
        let builtins = bindings.keys().cloned().collect();
        Environment {
            parent: None,
            bindings,
            builtins,
        }
    }

    /// Returns a new Environment with a given parent Environment
    pub fn new_sub(parent: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            parent: Some(parent.clone()),
            bindings: HashMap::new(),
            builtins: HashSet::new(),
        }
    }

    /// Binds a name to a value.
    /// Returns InterpreterError::BuiltinRebinding if name is used by a Builtin.
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

    /// Returns the value for a name in this Environment, or the
    /// nearest parent to define it.
    /// Returns an InterpreterError::UnboundName if name is unbound.
    pub fn get(&self, name: &str) -> Result<Value, InterpreterError> {
        match self.bindings.get(name) {
            Some(value) => Ok(value.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(name),
                None => Err(InterpreterError::UnboundName(name.to_string())),
            },
        }
    }
}
