use puffin::{Parser, PuffinParser, Rule, ast, interpreter};
use std::fs;


fn main() {
    let input = fs::read_to_string("src.puf").unwrap();
   
    let parsed = PuffinParser::parse(Rule::program, &input).unwrap();
    let ast = ast::build_program(parsed.into_iter().next().unwrap()).unwrap();
    let result = interpreter::eval(ast);
    match result.unwrap() {
        interpreter::value::Value::Null => {},
        value => println!("{}", value)
    }    
}
