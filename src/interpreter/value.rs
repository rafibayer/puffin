use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
use crate::Rule;

use crate::ast::node::*;
use super::InterpreterError;


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

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    String(String),
    Array(Vec<Value>),
    Structure(HashMap<String, Value>),
    Function{args: Vec<String>, block: Block, env: Environment}
}