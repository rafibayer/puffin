use std::{cell::RefCell, rc::Rc};

use crate::ast::node::Statement;

use super::{InterpreterError, Value, value::Environment};


/// Repl maintains an environment for repeated evaluation of statements in the same environment
pub struct Repl {
    environment: Rc<RefCell<Environment>>
}

#[allow(clippy::new_without_default)]
impl Repl {
    pub fn new() -> Repl {
        Repl {
            environment: Rc::new(RefCell::new(Environment::new()))
        }
    }

    // evaluates a statement in the current repl environment
    pub fn repl_statement(&self, statement: &Statement) -> Result<Option<Value>, InterpreterError> {
        super::eval_repl_statement(&statement, &self.environment)
    }
}