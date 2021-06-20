// todo: remove
#![feature(track_caller)]


use std::usize;
use pest::iterators::{Pair, Pairs};
use crate::{PuffinParser, Rule};


use node::*;

pub mod node;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum ASTError<'i> {
    UnexpectedPair(Pair<'i, Rule>),
    ChildMismatch{ expected: usize, got: usize},
    InvalidOperator(Pair<'i, Rule>),
    InvalidNum(Pair<'i, Rule>)
}

// Rule: Program
pub fn ast(program: Pair<Rule>) -> Result<Program, ASTError> {
    match program.as_rule() {
        Rule::program => build_program(program),
        _ => Err(unexpected_pair(program))
    }
}

// Rule: Program
fn build_program(program: Pair<Rule>) -> Result<Program, ASTError> {
    let mut statements = Program {
        program: Vec::new(),
    };

    // Statements
    let children = program.into_inner();

    for pair in children {
        match pair.as_rule() {
            Rule::statement => statements.program.push(build_statement(pair)?),
            Rule::EOI => break,
            _ => return Err(unexpected_pair(pair))
        }
    }

    Ok(statements)
}

// Rule: Statement
fn build_statement(statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let mut children: Vec<Pair<Rule>> = statement.into_inner().into_iter().collect();    

    // check if num children is not exactly 1
    expect_children(1, children.len())?;

    // all statement variants should have exactly 1 child
    let child = children.remove(0);

    match child.as_rule() {
        Rule::return_statment => build_return(child),
        Rule::assign_statment => build_assign(child),
        Rule::exp => {
            Ok(Statement { statement: StatementKind::Exp(build_exp(child)?) })
        },
        Rule::nest => todo!(),
        _ => Err(unexpected_pair(child))
    }
}

// Rule:: return_statement
fn build_return(return_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let mut children: Vec<Pair<Rule>> = return_statement.into_inner().collect();
    expect_children(1, children.len())?;

    let child = children.remove(0);

    match child.as_rule() {
        Rule::exp => Ok(Statement { statement: StatementKind::Return(build_exp(child)?) }),
        _ => Err(unexpected_pair(child))
    }
}

fn build_assign(assign_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let mut children: Vec<Pair<Rule>> = assign_statement.into_inner().collect();
    expect_children(2, children.len())?;

    let lhs = build_exp(children.remove(0))?;
    let rhs = build_exp(children.remove(0))?;

    Ok(Statement{ statement: StatementKind::Assign { lhs, rhs } })
}

// Rule: exp
fn build_exp(exp: Pair<Rule>) -> Result<Exp, ASTError> {
    let mut children: Vec<Pair<Rule>> = exp.into_inner().collect();
    expect_children(1, children.len())?;

    let child = children.remove(0);
    match child.as_rule() {
        Rule::exp => build_exp(child),
        Rule::infix => build_infix(child),
        Rule::term => {
            Ok(Exp { exp: ExpKind::Term( build_term(child)?) })
        },
        _ => Err(unexpected_pair(child))
    }
}



// Rule: infix
fn build_infix(infix: Pair<Rule>) -> Result<Exp, ASTError> {
    let mut children: Vec<Pair<Rule>> = infix.into_inner().collect();

    let term = build_term(children.remove(0))?;

    let mut op_terms: Vec<(OpKind, Term)> = Vec::new();

    while !children.is_empty() {
        let next_op = get_op(children.remove(0))?;
        let next_term = build_term(children.remove(0))?;
        op_terms.push((next_op, next_term))
    }

    Ok(Exp { exp: ExpKind::Infix { term, op_terms } })

}

// Rule: term
fn build_term(term: Pair<Rule>) -> Result<Term, ASTError> {
    let mut children: Vec<Pair<Rule>> = term.into_inner().collect();
    expect_children(1, children.len())?;
    let child = children.remove(0);

    match child.as_rule() {
        Rule::exp => {
            Ok(Term { term: TermKind::Paren(Box::new(build_exp(child)?)) })
        },
        Rule::unop_use => build_unop(child),
        Rule::function => build_function(child),
        Rule::function_call => build_function_call(child),
        Rule::array_index => build_array_index(child),
        Rule::array_init => build_array_init(child),
        Rule::name => build_name(child),
        Rule::num => build_num(child),
        Rule::string => build_string(child),
        _ => Err(unexpected_pair(child))
    }
}

fn build_unop(unop: Pair<Rule>) -> Result<Term, ASTError> {
    let mut children: Vec<Pair<Rule>> = unop.into_inner().collect();
    expect_children(2, children.len())?;

    let unop_op = get_unop(children.remove(0))?;
    let unop_term = build_term(children.remove(0))?;

    Ok(Term { term: TermKind::UnopUse{ unop: unop_op, term: Box::new(unop_term) } })
}

fn build_function(function: Pair<Rule>) -> Result<Term, ASTError> {
    let mut children: Vec<Pair<Rule>> = function.into_inner().collect();
    expect_children(2, children.len())?;
    let child = children.remove(0);

    todo!()
}

fn build_function_call(function_call: Pair<Rule>) -> Result<Term, ASTError> {
    let mut children: Vec<Pair<Rule>> = function_call.into_inner().collect();
    expect_children(1, children.len())?;
    let child = children.remove(0);

    todo!()
}

fn build_array_index(array_index: Pair<Rule>) -> Result<Term, ASTError> {
    let mut children: Vec<Pair<Rule>> = array_index.into_inner().collect();
    expect_children(2, children.len())?;
    let child = children.remove(0);

    todo!()
}

fn build_array_init(array_init: Pair<Rule>) -> Result<Term, ASTError> {
    let mut children: Vec<Pair<Rule>> = array_init.into_inner().collect();
    expect_children(1, children.len())?;
    let child = children.remove(0);

    todo!()
}

fn build_name(name: Pair<Rule>) -> Result<Term, ASTError> {
    Ok(Term {term: TermKind::Name(name.as_str().to_string())})
}

fn build_num(term: Pair<Rule>) -> Result<Term, ASTError> {
    let num: Result<f64, _> = term.as_str().parse();
    if num.is_err() {
        return Err(ASTError::InvalidNum(term));
    }

    Ok(Term { term: TermKind::Num(num.unwrap())})
}

fn build_string(term: Pair<Rule>) -> Result<Term, ASTError> {
    Ok(Term {term: TermKind::String(term.as_str().to_string())})
}

// Rule: op
fn get_op(op: Pair<Rule>) -> Result<OpKind, ASTError> {
    Ok(match op.as_str() {
        "*" => OpKind::Mul,
        "%" => OpKind::Mod,
        "/" => OpKind::Div,
        "+" => OpKind::Plus,
        "-" => OpKind::Minus,
        "<" => OpKind::Lt,
        ">" => OpKind::Gt,
        "<=" => OpKind::Le,
        ">=" => OpKind::Ge,
        "==" => OpKind::Eq,
        "!=" => OpKind::Ne,
        "&&" => OpKind::And,
        "||" => OpKind::Or,
        _ => return Err(ASTError::InvalidOperator(op))
    })
}

fn get_unop(unop: Pair<Rule>) -> Result<UnopKind, ASTError> {
    Ok(match unop.as_str() {
        "!" => UnopKind::Not,
        "-" => UnopKind::Neg,
        _ => return Err(ASTError::InvalidOperator(unop))
    })
}

// todo: remove annotation
#[track_caller]
fn expect_children(expected: usize, got: usize) -> Result<(), ASTError<'static>> {
    if expected != got {
        // https://stackoverflow.com/a/60714285/9723960
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        eprintln!("child mismatch: ln:{}", caller_line_number);
        return Err(ASTError::ChildMismatch{ expected, got })
    }

    Ok(())
}

#[track_caller]
fn unexpected_pair(pair: Pair<Rule>) -> ASTError {
    let caller_location = std::panic::Location::caller();
    let caller_line_number = caller_location.line();
    eprintln!("unexpected pair: ln:{}", caller_line_number);
    ASTError::UnexpectedPair(pair)
}