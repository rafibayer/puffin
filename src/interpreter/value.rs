// use std::{cell::RefCell, collections::HashMap};
// use pest::iterators::{Pair, Pairs};
// use crate::Rule;

// pub enum EnvironmentError {

// }

// pub struct Environment {
//     bindings: HashMap<String, RefCell<Value>>
// }

// impl Environment {
//     pub fn bind(name: &str, value: Value) {

//     }

// }

// pub enum Value {
//     Num(f64),
//     String(String),
//     Array(Vec<Value>),
//     Function{names: Vec<String>, body: Pair<Rule>}
// }