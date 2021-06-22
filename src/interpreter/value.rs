use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
use pest::iterators::{Pair, Pairs};
use crate::Rule;

use super::InterpreterError;


pub struct Environment<'a> {
    parent: Option<Rc<RefCell<Environment<'a>>>>,
    bindings: HashMap<String, Value<'a>>
}

impl<'a> Environment<'a> {

    pub fn new() -> Environment<'a> {
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

    pub fn bind(&mut self, name: String, value: &'a Value) {
        self.bindings.insert(name, value.clone());
    }


    pub fn get(&mut self, name: String) -> Result<Value, InterpreterError> {
        match self.bindings.get(&name) {
            Some(value) => Ok(value.clone()),
            None => match self.parent {
                    Some(env) => {
                        (&env.borrow()).get(name).clone()
                    },
                    None => Err(InterpreterError::UnboundName(name)),
                }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Num(f64),
    String(String),
    Array(Vec<Value<'a>>),
    Function{names: Vec<String>, body: Pair<'a, Rule>}
}