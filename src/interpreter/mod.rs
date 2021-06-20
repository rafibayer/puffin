use crate::{PuffinParser, Rule, Parser};

mod value;

pub struct InterpreterError {
    pub pos: usize,
    pub msg: &'static str,
}


// fn eval_num(num: &str, )