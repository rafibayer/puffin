use std::{error, fmt::Display};

use crate::Rule;
use pest::iterators::Pair;

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
    InvalidName(String),
    DuplicateArg(String),
}

impl error::Error for ASTError {}

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
    let child = get_one(statement)?;

    match child.as_rule() {
        Rule::return_statment => build_return(child),
        Rule::assign_statment => build_assign(child),
        Rule::exp => Ok(Statement {
            statement: StatementKind::Exp(build_exp(child)?),
        }),
        Rule::nest => build_nest(child),
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
    // value
    let mut inner = get_inner(assignable);

    // parser allows for this to be a parenthesized assignable, if this is the case,
    // we could unwrap it to allow for this, but that's a big pain in the ass right now
    let name = build_name(get_one(inner.remove(0))?)?;

    let mut assignable_vec = Vec::with_capacity(inner.len());

    while !inner.is_empty() {
        let next = get_one(inner.remove(0))?;

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
        name,
        assignable: assignable_vec,
    })
}

fn build_nest(nest_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let inner = get_one(nest_statement)?;

    Ok(match inner.as_rule() {
        Rule::condnest => Statement {
            statement: StatementKind::Nest(NestKind::CondNest(build_condnest(inner)?)),
        },
        Rule::loopnest => Statement {
            statement: StatementKind::Nest(NestKind::LoopNest(build_loopnest(inner)?)),
        },
        _ => return Err(unexpected_token(inner)),
    })
}

fn build_condnest(condnest: Pair<Rule>) -> Result<CondNestKind, ASTError> {
    let inner = get_one(condnest)?;
    match inner.as_rule() {
        Rule::if_block | Rule::if_else_block => {
            // to reduce redundancy, we handle if and ifelse the same, just parsing the else
            // block and returning IfElse if we have to. Downside: we can't expect_children, since
            // we need only 2 for if, and 3 for if else
            let has_else = matches!(inner.as_rule(), Rule::if_else_block);
            let mut if_parts = get_inner(inner);
            let cond = build_exp(if_parts.remove(0))?;
            let then = build_block(if_parts.remove(0))?;

            if has_else {
                let or_else = build_block(if_parts.remove(0))?;
                return Ok(CondNestKind::IfElse {
                    cond,
                    then,
                    or_else,
                });
            }

            Ok(CondNestKind::If { cond, then })
        }
        _ => Err(unexpected_token(inner)),
    }
}

fn build_loopnest(loopnest: Pair<Rule>) -> Result<LoopNestKind, ASTError> {
    let inner = get_one(loopnest)?;
    match inner.as_rule() {
        Rule::while_block => {
            let mut while_parts = get_inner(inner);
            expect_children(2, &while_parts)?;
            let cond = build_exp(while_parts.remove(0))?;
            let block = build_block(while_parts.remove(0))?;

            Ok(LoopNestKind::While { cond, block })
        }
        Rule::for_block => {
            let mut for_parts = get_inner(inner);
            expect_children(4, &for_parts)?;
            let init = build_statement(for_parts.remove(0))?;
            let cond = build_exp(for_parts.remove(0))?;

            // "adv" (usually the i++ part) of the loop can be either an expression, or an assignment.
            // either way, we wrap it in a statement
            let adv = match for_parts[0].as_rule() {
                Rule::assign_statment => build_assign(for_parts.remove(0))?,
                Rule::exp => Statement {
                    statement: StatementKind::Exp(build_exp(for_parts.remove(0))?),
                },
                _ => return Err(unexpected_token(for_parts.remove(0))),
            };

            let block = build_block(for_parts.remove(0))?;

            Ok(LoopNestKind::For {
                init: Box::new(init),
                cond,
                adv: Box::new(adv),
                block,
            })
        }
        _ => Err(unexpected_token(inner)),
    }
}

fn build_name(name: Pair<Rule>) -> Result<String, ASTError> {
    match name.as_rule() {
        Rule::name => {
            let val = name.as_str().to_string();
            if lookup::is_keyword(&val) {
                return Err(ASTError::InvalidName(val));
            }
            Ok(val)
        }
        _ => Err(unexpected_token(name)),
    }
}

fn build_exp(exp: Pair<Rule>) -> Result<Exp, ASTError> {
    let mut inner = get_inner(exp);
    let mut terms = Vec::with_capacity(inner.len());

    while !inner.is_empty() {
        let next = inner.remove(0);
        terms.push(match next.as_rule() {
            Rule::value => TermKind::Value(build_value(next)?),
            Rule::log_op | Rule::comp_op | Rule::sum_op | Rule::mul_op => {
                lookup::infix(next.as_str())?
            }
            Rule::un_op => lookup::unary(next.as_str())?,
            Rule::post_op => TermKind::Operator(
                OperatorKind::Postfix(build_postfix(next)?),
                Associativity::Left,
                7, // highest precedence, rest are in lookup.rs
            ),
            _ => return Err(unexpected_token(next)),
        });
    }

    Ok(Exp { exp: terms })
}

fn build_value(next: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let child = get_one(next)?;
    Ok(match child.as_rule() {
        Rule::paren => ValueKind::Paren(Box::new(build_exp(get_one(child)?)?)),
        Rule::structure => build_structure(child)?,
        Rule::function => build_function(child)?,
        Rule::num => ValueKind::Num(build_num(child)?),
        Rule::string => build_string(child)?,
        Rule::array_init => ValueKind::ArrayInit(Box::new(build_exp(get_one(child)?)?)),
        Rule::name => ValueKind::Name(build_name(child)?),
        Rule::null => ValueKind::Null,
        _ => return Err(unexpected_token(child)),
    })
}

fn build_structure(structure: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let inner = get_inner(structure);

    let mut fields = Vec::with_capacity(inner.len());

    for field in inner {
        let mut contents = get_inner(field);
        expect_children(2, &contents)?;
        fields.push(Field {
            name: build_name(contents.remove(0))?,
            exp: build_exp(contents.remove(0))?,
        })
    }

    Ok(ValueKind::Structure(fields))
}

fn build_function(function: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let mut inner = get_inner(function);
    // last child is block, everything else is an arg
    let mut args = Vec::with_capacity(inner.len() - 1);
    while inner.len() > 1 {
        let next = build_name(inner.remove(0))?;
        // check for duplicate args
        if args.contains(&next) {
            return Err(ASTError::DuplicateArg(next));
        }
        args.push(next);
    }

    let block = build_block(inner.remove(0))?;

    Ok(ValueKind::Function { args, block })
}

fn build_block(statements: Pair<Rule>) -> Result<Block, ASTError> {
    let inner = get_inner(statements);
    let mut block = Vec::with_capacity(inner.len());

    for statement in inner {
        block.push(build_statement(statement)?);
    }

    Ok(Block { block })
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

    Ok(match inner.as_rule() {
        Rule::subscript => PostOp::Subscript(Box::new(build_exp(get_one(inner)?)?)),
        Rule::call => {
            let actuals = get_inner(inner);
            let mut exps = Vec::with_capacity(actuals.len());
            for actual in actuals {
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
            got: children.len(),
            expected: 1,
        });
    }

    Ok(children.remove(0))
}

fn get_inner(pair: Pair<Rule>) -> Vec<Pair<Rule>> {
    pair.into_inner().collect()
}

// todo: remove annotation
#[track_caller]
fn expect_children(expected: usize, got: &[Pair<Rule>]) -> Result<(), ASTError> {
    if expected != got.len() {
        // https://stackoverflow.com/a/60714285/9723960
        let caller_line_number = std::panic::Location::caller().line();
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
    let caller_line_number = std::panic::Location::caller().line();
    eprintln!("unexpected token: src\\ast\\mod.rs:{}", caller_line_number);
    ASTError::UnexpectedToken(format!(
        "{:?}: {}",
        pair.as_rule(),
        pair.as_str().to_string()
    ))
}

impl Display for ASTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self)
    }
}
