use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

use super::{InterpreterError, Value, builtin};



#[derive(Debug, Clone)]
pub struct Environment {
    // parent environment
    parent: Option<Rc<RefCell<Environment>>>,
    // local bindings
    bindings: HashMap<String, Value>,
    // builtin names, can't be rebound
    builtins: HashSet<String>,
}

impl Environment {
    pub fn new() -> Environment {
        // get_builtins and the builtins hashset should probably both be static/lazy & cached
        let bindings = builtin::get_builtins();
        let builtins = bindings.keys().cloned().collect();
        Environment {
            parent: None,
            bindings,
            builtins
        }
    }

    pub fn new_inner(outer: Rc<RefCell<Environment>>) -> Environment {
        let bindings = builtin::get_builtins();
        let builtins = bindings.keys().cloned().collect();
        Environment {
            parent: Some(outer),
            bindings,
            builtins
        }
    }

    pub fn bind(&mut self, name: String, value: &Value) -> Result<Value, InterpreterError> {
        if self.builtins.contains(&name) {
            return Err(InterpreterError::BuiltinRebinding(name));
        }
        self.bindings.insert(name, value.clone());

        Ok(Value::Null)
    }

    pub fn get(&self, name: String) -> Result<Value, InterpreterError> {
        match self.bindings.get(&name) {
            Some(value) => Ok(value.clone()),
            None => match self.parent {
                Some(ref env) => env.borrow().get(name),
                None => Err(InterpreterError::UnboundName(name)),
            },
        }
    }

}
