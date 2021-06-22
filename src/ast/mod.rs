use crate::{PuffinParser, Rule};
use pest::iterators::{Pair, Pairs};
use std::usize;

use node::*;

mod lookup;
pub mod node;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum ASTError {
    ChildMismatch { got: usize, expected: usize },
    UnexpectedToken(String),
    InvalidNum(String),
    InvalidOp(String),
}

pub fn build_program(program: Pair<Rule>) -> Result<Program, ASTError> {
    let mut statements: Vec<Statement> = Vec::new();

    for statement in get_inner(program) {
        match statement.as_rule() {
            Rule::statement => statements.push(build_statement(statement)?),
            Rule::EOI => break,
            _ => return Err(unexpected_token(statement)),
        }
    }

    Ok(Program {
        program: statements,
    })
}

fn build_statement(statement: Pair<Rule>) -> Result<Statement, ASTError> {
    dbg!(&statement);

    let child = get_one(statement)?;

    match child.as_rule() {
        Rule::return_statment => build_return(child),
        Rule::assign_statment => build_assign(child),
        Rule::exp => Ok(Statement {
            statement: StatementKind::Exp(build_exp(child)?),
        }),
        Rule::nest => todo!(),
        _ => Err(unexpected_token(child)),
    }
}

fn build_return(return_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let exp = build_exp(get_one(return_statement)?)?;
    Ok(Statement {
        statement: StatementKind::Return(exp),
    })
}

fn build_assign(assign_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let mut inner = get_inner(assign_statement);
    expect_children(2, &inner)?;
    let lhs = build_assignable(inner.remove(0))?;
    let rhs = build_exp(inner.remove(0))?;

    Ok(Statement {
        statement: StatementKind::Assign { lhs, rhs },
    })
}

fn build_assignable(assignable: Pair<Rule>) -> Result<Assingnable, ASTError> {
    let mut inner = get_inner(assignable);

    let name = build_name(get_one(inner.remove(0))?)?;

    let mut assignable_vec = Vec::new();

    while !inner.is_empty() {
        let next = inner.remove(0);
        assignable_vec.push(match next.as_rule() {
            Rule::subscript => AssignableKind::ArrayIndex {
                index: build_exp(get_one(next)?)?,
            },
            Rule::dot => AssignableKind::StructureField {
                field: build_name(get_one(next)?)?,
            },
            _ => return Err(unexpected_token(next)),
        });
    }

    Ok(Assingnable {
        name: name,
        assignable: assignable_vec,
    })
}

fn build_name(name: Pair<Rule>) -> Result<String, ASTError> {
    match name.as_rule() {
        Rule::name => Ok(name.as_str().to_string()),
        _ => Err(unexpected_token(name)),
    }
}

fn build_exp(exp: Pair<Rule>) -> Result<Exp, ASTError> {
    let mut inner = get_inner(exp);
    let mut terms = Vec::new();

    while !inner.is_empty() {
        let next = inner.remove(0);
        // dbg!(&next);
        terms.push(match next.as_rule() {
            Rule::value => TermKind::Value(build_value(next)?),
            Rule::log_op | Rule::comp_op | Rule::sum_op | Rule::mul_op => {
                TermKind::Operator(OperatorKind::Infix(lookup::infix(next.as_str())?))
            }
            Rule::un_op => TermKind::Operator(OperatorKind::Unary(lookup::unary(next.as_str())?)),
            Rule::post_op => TermKind::Operator(OperatorKind::Postfix(build_postfix(next)?)),
            _ => return Err(unexpected_token(next)),
        });
    }

    Ok(Exp { exp: terms })
}

fn build_value(next: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let child = get_one(next)?;
    Ok(match child.as_rule() {
        Rule::paren => ValueKind::Paren(Box::new(build_exp(get_one(child)?)?)),
        Rule::structure => todo!(),
        Rule::function => todo!(),
        Rule::num => ValueKind::Num(build_num(child)?),
        Rule::string => build_string(child)?,
        Rule::array_init => ValueKind::ArrayInit(Box::new(build_exp(get_one(child)?)?)),
        Rule::name => ValueKind::Name(build_name(child)?),
        _ => return Err(unexpected_token(child)),
    })
}

fn build_num(num: Pair<Rule>) -> Result<f64, ASTError> {
    Ok(match num.as_str().parse() {
        Ok(n) => n,
        Err(e) => return Err(ASTError::InvalidNum(e.to_string())),
    })
}

fn build_string(string: Pair<Rule>) -> Result<ValueKind, ASTError> {
    match string.as_rule() {
        Rule::string => Ok(ValueKind::String(string.as_str().to_string())),
        _ => Err(unexpected_token(string)),
    }
}

fn build_postfix(postfix: Pair<Rule>) -> Result<PostOp, ASTError> {
    let inner = get_one(postfix)?;
    dbg!(&inner);

    Ok(match inner.as_rule() {
        Rule::subscript => PostOp::Subscript(Box::new(build_exp(get_one(inner)?)?)),
        Rule::call => {
            let mut exps = Vec::new();
            for actual in get_inner(inner) {
                exps.push(build_exp(actual)?)
            }

            PostOp::Call(exps)
        }
        Rule::dot => PostOp::Dot(build_name(get_one(inner)?)?),
        _ => return Err(unexpected_token(inner)),
    })
}

#[track_caller]
fn get_one(pair: Pair<Rule>) -> Result<Pair<Rule>, ASTError> {
    let mut children = get_inner(pair);
    if children.len() != 1 {
        // todo: replace with expect_children
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        eprintln!("expected one: src\\ast\\mod.rs:{}", caller_line_number);
        return Err(ASTError::ChildMismatch {
            got: 1,
            expected: children.len(),
        });
    }

    Ok(children.remove(0))
}

fn get_inner(pair: Pair<Rule>) -> Vec<Pair<Rule>> {
    pair.into_inner().collect()
}

// todo: remove annotation
#[track_caller]
fn expect_children(expected: usize, got: &Vec<Pair<Rule>>) -> Result<(), ASTError> {
    if expected != got.len() {
        // https://stackoverflow.com/a/60714285/9723960
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        eprintln!("child mismatch: src\\ast\\mod.rs:{}", caller_line_number);
        return Err(ASTError::ChildMismatch {
            expected,
            got: got.len(),
        });
    }

    Ok(())
}

// todo: remove annotation
#[track_caller]
fn unexpected_token(pair: Pair<Rule>) -> ASTError {
    // https://stackoverflow.com/a/60714285/9723960
    let caller_location = std::panic::Location::caller();
    let caller_line_number = caller_location.line();
    eprintln!("unexpected token: src\\ast\\mod.rs:{}", caller_line_number);
    ASTError::UnexpectedToken(format!(
        "{:?}: {}",
        pair.as_rule(),
        pair.as_str().to_string()
    ))
}
