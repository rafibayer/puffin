use pest::iterators::Pairs;

use super::*;

pub fn clean_print(pairs: Pairs<Rule>, depth: usize) {

    let tabs = "\t".repeat(depth);
    for pair in pairs {
        println!("{}{:#?}: {}", tabs, pair.as_rule(), pair.as_str());
        clean_print(pair.into_inner(), depth + 1);
    }
}