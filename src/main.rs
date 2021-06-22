use puffin::{Parser, PuffinParser, Rule, ast, interpreter, parser::utils::clean_print};
use std::fs;


fn main() {
    let input = fs::read_to_string("src.puf").unwrap();
   
    let pairs = PuffinParser::parse(Rule::program, &input).unwrap();
    // clean_print(pairs, 0);
    // dbg!(&pairs);
    let ast = ast::build_program(pairs.into_iter().next().unwrap()).unwrap();
    println!("{:#?}", ast);

    let result = interpreter::eval(ast);
    println!("{:?}", result.unwrap());    
}
