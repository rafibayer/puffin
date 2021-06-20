use puffin::{PuffinParser, Parser, Rule};
use std::fs;


fn main() {
    let input = fs::read_to_string("src.puf").unwrap();
   
    let pairs = PuffinParser::parse(Rule::program, &input);
    match pairs {
        Ok(pairs) => println!("{:#?}", pairs),
        Err(e) => println!("{:#?}", e),
    }
}