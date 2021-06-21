use puffin::{PuffinParser, Parser, Rule};
use std::fs;


fn main() {
    let input = fs::read_to_string("src.puf").unwrap();
   
    let pairs = PuffinParser::parse(Rule::program, &input).unwrap();
    println!("{:#?}", pairs.clone().into_iter().nth(0));
    // let ast = ast::ast(pairs.into_iter().nth(0).unwrap()).unwrap();
    // println!("{:#?}", ast);

    
}