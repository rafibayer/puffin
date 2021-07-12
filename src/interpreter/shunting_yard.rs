//! Author: Rafael Bayer (2021)
//! This module contains an implementation of the Shunting-Yard Algorithm.
//! Shunting-Yard takes an expression written in infix notation, and outputs
//! and expression in postfix (reverse-polish) notation.
//!
//! My implementation is based on the psuedo-code for the algorithm as described by
//! wikipedia, and contains several inline comments taken directly from the article
//! demonstrating correspondence between my code and the psuedo-code.


use std::collections::VecDeque;

use super::*;

/// shunting yard algorithm: [https://en.wikipedia.org/wiki/Shunting-yard_algorithm].
/// 
/// orders an infix expression (a list of terms) into reverse polish notation
/// according to associativity and operator precedence.
/// example: `1 + 2 * 3` => `1 2 3 * +`
/// Returns a queue of references to terms in reverse polish order. 
pub fn as_rpn_queue<'i>(exp: &'i Exp) -> VecDeque<&'i TermKind> {
    
    let mut op_stack: Vec<&TermKind> = Vec::new();
    let mut out_queue: VecDeque<&'i TermKind> = VecDeque::new();

    // while there are tokens to be read:
    for term in &exp.exp {
        //  if the token is:
        match term {
            // a value: put it into the output queue
            TermKind::Value(_) => out_queue.push_back(term),
            // an operator o1:
            TermKind::Operator (_, assoc, prec )=> {
                let mut o2 = op_stack.last();
                // o1 operator case, difficult to write cleanly as one while without `while let`.
                // consider the 2 nested if statements to all be part of the while loop condition
                /*
                while (
                    there is an operator o2 other than the left parenthesis at the top
                    of the operator stack, and (o2 has greater precedence than o1
                    or they have the same precedence and o1 is left-associative)
                */
                while o2.is_some() {
                    if let TermKind::Operator(_, _, o2_prec) = o2.unwrap() {
                        if o2_prec > prec || (o2_prec == prec && matches!(assoc, &Associativity::Left)) {
                            // pop o2 from the operator stack into the output queue
                            out_queue.push_back(op_stack.pop().unwrap());
                            o2 = op_stack.last();

                        
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // push o1 onto the operator stack
                op_stack.push(term);

            }
        }
    }

    // while there are tokens on the operator stack:
    while !op_stack.is_empty() {
        // pop the operator from the operator stack onto the output queue
        out_queue.push_back(op_stack.pop().unwrap());
    }

    out_queue
}