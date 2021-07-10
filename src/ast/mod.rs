//! Author: Rafael Bayer (2021)
//! This module contains the Puffin AST generator.
//! `build_program` converts the programs parse tree into the programs AST.
//! 
//! ast is also responsible for parsing literals, and expanding certain syntactic sugars
//! into their full representation within the AST.

use std::{fmt::Display, vec};

use crate::Rule;
use pest::iterators::Pair;

use node::*;

mod lookup;
pub mod node;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub enum ASTError {
    // encountered unexpected number of tokens for AST node
    ChildMismatch { got: usize, expected: usize },
    // encountered unexpected token for AST Node
    UnexpectedToken(String),
    // failed to parse a number literal
    InvalidNum(String),
    // encountered unknown operator
    InvalidOp(String),
    // encountered illegal name (was a keyword)
    InvalidName(String),
    DuplicateArg(String),
}

// builds a program.
/// `Rule: Program`
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

// builds a statement
/// `Rule: Statement`
pub fn build_statement(statement: Pair<Rule>) -> Result<Statement, ASTError> {
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

/// `Rule: return_statement`
fn build_return(return_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let exp = build_exp(get_one(return_statement)?)?;
    Ok(Statement {
        statement: StatementKind::Return(exp),
    })
}

/// `Rule: assign_statment`
fn build_assign(assign_statement: Pair<Rule>) -> Result<Statement, ASTError> {
    let line = assign_statement.as_span().start_pos().line_col().0;

    let mut inner = get_inner(assign_statement);
    // regular assigmnet statements have 2 children:
    //      c1 = c2
    // augmented assigment statements have 3 children:
    //      c1 c2= c3
    match inner.len() {
        // regular assignment (a = b)
        2 => {
            let lhs = build_assignable(inner.remove(0))?;
            let rhs = build_exp(inner.remove(0))?;
            Ok(Statement {
                statement: StatementKind::Assign { lhs, rhs },
            })
        }
        // augmented assignment (a op= b)
        3 => {
            let assign_to = inner.remove(0);
            let lhs = build_assignable(assign_to.clone())?;
            let aug = lookup::infix(inner.remove(0).as_str().to_string())?;
            // preserve right hand expression by wrapping in parens
            let mut rhs = Exp {
                exp: vec![TermKind::Value(ValueKind::Paren(Box::new(build_exp(
                    inner.remove(0),
                )?)))],
                line,
            };

            rhs.exp.insert(0, aug);
            rhs.exp.insert(
                0,
                TermKind::Value(ValueKind::Paren(Box::new(build_exp(assign_to)?))),
            );

            // final statement expands
            // from: a op= b;
            // to:   a = (a) op (b);
            Ok(Statement {
                statement: StatementKind::Assign { lhs, rhs },
            })
        }
        e => Err(ASTError::ChildMismatch {
            got: e,
            expected: 2,
        }),
    }
}

/// `Rule: exp`
// alternate ast generation for expressions.
// takes an expression of the form:
// <name> <assignable>*
// where assignable is either a structure field access ( e.g. ".name") or array access ( e.g. "[5]")
fn build_assignable(assignable: Pair<Rule>) -> Result<Assingnable, ASTError> {
    // value
    let mut inner = get_inner(assignable);

    // base name being assigned to:
    // a[5][7] => a
    let name = build_name(get_one(inner.remove(0))?)?;

    // vector containing all the subassignments
    // a[5][7] => [[5], [7]]
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

/// `rule: nest`
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

/// `rule: condnest`
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

/// `rule: loopnest`
fn build_loopnest(loopnest: Pair<Rule>) -> Result<LoopNestKind, ASTError> {
    let inner = get_one(loopnest)?;
    match inner.as_rule() {
        Rule::while_block => {
            let mut while_parts = get_inner(inner);
            expect_children(2, &while_parts)?;
            let cond = build_exp(while_parts.remove(0))?;
            let block = build_block(while_parts.remove(0))?;

            Ok(LoopNestKind::While { cond, block })
        },
        Rule::for_in_block => {
            let mut for_parts = get_inner(inner);
            expect_children(3, &for_parts)?;
            let name = build_name(for_parts.remove(0))?;
            let array = build_exp(for_parts.remove(0))?;
            let block = build_block(for_parts.remove(0))?;

            Ok(LoopNestKind::ForIn{
                name,
                array,
                block,
            })
        },
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

/// `rule: name`
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

/// `rule: exp`
/// expressions are just a list of terms in the AST.
/// a term is either a value, or some kind of operator.
/// we use the lookup module to convert from an operators string, to
/// the appropriate TermKind, which in turn contains the Operator enum
/// with data about the operators kind, associativity, and precedence
fn build_exp(exp: Pair<Rule>) -> Result<Exp, ASTError> {
    let line = exp.as_span().start_pos().line_col().0;
    let mut inner = get_inner(exp);
    let mut terms = Vec::with_capacity(inner.len());

    while !inner.is_empty() {
        let next = inner.remove(0);
        terms.push(match next.as_rule() {
            Rule::value => TermKind::Value(build_value(next)?),
            Rule::log_op | Rule::comp_op | Rule::sum_op | Rule::mul_op => {
                lookup::infix(next.as_str().to_string())?
            }
            Rule::un_op => lookup::unary(next.as_str().to_string())?,
            // postfix operators contain additional parts, for example the index or fieldname,
            // we must parse these further instead of just looking them up.
            Rule::post_op => TermKind::Operator(
                OperatorKind::Postfix(build_postfix(next)?),
                Associativity::Left,
                7, // highest precedence, rest are in lookup.rs
            ),
            _ => return Err(unexpected_token(next)),
        });
    }

    Ok(Exp { exp: terms, line })
}

/// `rule: value`
fn build_value(next: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let child = get_one(next)?;
    Ok(match child.as_rule() {
        Rule::paren => ValueKind::Paren(Box::new(build_exp(get_one(child)?)?)),
        Rule::structure => build_structure(child)?,
        Rule::function => build_function(child)?,
        Rule::num => ValueKind::Num(build_num(child)?),
        Rule::string => build_string(child)?,
        Rule::array_init => build_array_init(child)?,
        Rule::name => ValueKind::Name(build_name(child)?),
        Rule::null => ValueKind::Null,
        _ => return Err(unexpected_token(child)),
    })
}

/// `rule: structure`
fn build_structure(structure: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let inner = get_inner(structure);

    let mut fields = Vec::with_capacity(inner.len());

    // build all struct fields
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

/// `rule: function`
fn build_function(function: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let mut inner = get_inner(function);

    // last child is block, everything else is an arg
    let mut args = Vec::with_capacity(inner.len() - 1);

    // consume all
    while inner.len() > 1 {
        let next = build_name(inner.remove(0))?;
        // check for duplicate args
        if args.contains(&next) {
            return Err(ASTError::DuplicateArg(next));
        }
        args.push(next);
    }

    // consuming the args leaves us with the last token, the function body
    let function_body = inner.remove(0);

    // we either parse the block as is, or if the function is a lambda
    // (e.g. fn() => x) we expand it to a block
    let block = match function_body.as_rule() {
        Rule::block => build_block(function_body)?,
        // expand lambda expression into block returning expression value
        Rule::lambda => Block {
            block: vec![build_return(function_body)?],
        },
        _ => return Err(unexpected_token(function_body)),
    };

    Ok(ValueKind::FunctionDef { args, block })
}

/// `rule: block`
fn build_block(statements: Pair<Rule>) -> Result<Block, ASTError> {
    let inner = get_inner(statements);
    let mut block = Vec::with_capacity(inner.len());

    for statement in inner {
        block.push(build_statement(statement)?);
    }

    Ok(Block { block })
}

/// `rule: num`
/// here we parse number literals
fn build_num(num: Pair<Rule>) -> Result<f64, ASTError> {
    Ok(match num.as_str().parse() {
        Ok(n) => n,
        Err(e) => return Err(ASTError::InvalidNum(e.to_string())),
    })
}

/// `rule: string`
/// here we parse string literals
fn build_string(string: Pair<Rule>) -> Result<ValueKind, ASTError> {
    match string.as_rule() {
        Rule::string => {
            let len = string.as_str().len();
            // trim the quote literals
            Ok(ValueKind::String(string.as_str()[1..len - 1].to_string()))
        }
        _ => Err(unexpected_token(string)),
    }
}

fn build_array_init(array_init: Pair<Rule>) -> Result<ValueKind, ASTError> {
    let inner = get_one(array_init)?;

    Ok(ValueKind::ArrayInit(match inner.as_rule() {
        Rule::sized_init => {
            ArrayInitKind::Sized(Box::new(build_exp(get_one(inner)?)?))
            
        },
        Rule::range_init => {
            let mut init_inner = get_inner(inner);
            let from = build_exp(init_inner.remove(0))?;
            let to = build_exp(init_inner.remove(0))?;
            ArrayInitKind::Range(Box::new(from), Box::new(to))
        },
        _ => return Err(unexpected_token(inner))
    }))
}

/// `rule: postfix`
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
        let caller_location = std::panic::Location::caller();
        let caller_line_number = caller_location.line();
        eprintln!("AST expected one: src\\ast\\mod.rs:{}", caller_line_number);
        return Err(ASTError::ChildMismatch {
            got: children.len(),
            expected: 1,
        });
    }

    Ok(children.remove(0))
}

/// helper function to retrieve inner pairs of a pair
#[inline]
fn get_inner(pair: Pair<Rule>) -> Vec<Pair<Rule>> {
    pair.into_inner().collect()
}

impl Display for ASTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self)
    }
}

/****************** Error Helpers ******************/

#[track_caller]
fn expect_children(expected: usize, got: &[Pair<Rule>]) -> Result<(), ASTError> {
    if expected != got.len() {
        // https://stackoverflow.com/a/60714285/9723960
        let caller_line_number = std::panic::Location::caller().line();
        eprintln!(
            "AST child mismatch: src\\ast\\mod.rs:{}",
            caller_line_number
        );
        return Err(ASTError::ChildMismatch {
            expected,
            got: got.len(),
        });
    }

    Ok(())
}

#[track_caller]
fn unexpected_token(pair: Pair<Rule>) -> ASTError {
    // https://stackoverflow.com/a/60714285/9723960
    let caller_line_number = std::panic::Location::caller().line();
    eprintln!(
        "AST unexpected token: src\\ast\\mod.rs:{}",
        caller_line_number
    );
    ASTError::UnexpectedToken(format!(
        "{:?}: {}",
        pair.as_rule(),
        pair.as_str().to_string()
    ))
}
