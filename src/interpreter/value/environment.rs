use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{InterpreterError, Value};



#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    bindings: HashMap<String, Value>
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            parent: None,
            bindings: HashMap::new(),
        }
    }

    pub fn new_inner(outer: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            parent: Some(outer),
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, name: String, value: &Value) {
        self.bindings.insert(name, value.clone());
    }


    pub fn get(&self, name: String) -> Result<Value, InterpreterError> {
        match self.bindings.get(&name) {
            Some(value) => Ok(value.clone()),
            None => match self.parent {
                    Some(ref env) => {
                        env.borrow().get(name)
                    },
                    None => Err(InterpreterError::UnboundName(name)),
                }
        }
    }
}