//! Author: Rafael Bayer (2021)
//! 
//! This module contains helper lookup functions to convert between 
//! strings and operator AST nodes.

use cached::proc_macro::cached;
use super::node::*;
use super::ASTError;

/// Converts a string representation of an infix operator into
/// the appropriate Operater variant of TermKind
#[cached]
pub fn infix(op: String) -> Result<TermKind, ASTError> {
    Ok(match op.as_str() {
        "||" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Or),
            Associativity::Left,
            0,
        ),
        "&&" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::And),
            Associativity::Left,
            1,
        ),
        "==" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Eq),
            Associativity::Left,
            2,
        ),
        "!=" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Ne),
            Associativity::Left,
            2,
        ),
        "<" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Lt),
            Associativity::Left,
            3,
        ),
        "<=" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Le),
            Associativity::Left,
            3,
        ),
        ">" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Gt),
            Associativity::Left,
            3,
        ),
        ">=" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Ge),
            Associativity::Left,
            3,
        ),
        

        "-" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Minus),
            Associativity::Left,
            4,
        ),
        "+" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Plus),
            Associativity::Left,
            4,
        ),

        "/" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Div),
            Associativity::Left,
            5,
        ),
        "%" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Mod),
            Associativity::Left,
            5,
        ),
        "*" => TermKind::Operator (
            OperatorKind::Infix(InfixOp::Mul),
            Associativity::Left,
            5,
        ),

        _ => return Err(ASTError::InvalidOp(op.to_string())),
    })
}

/// Converts a string representation of an unary (prefix) operator into
/// the appropriate Operater variant of TermKind
#[cached]
pub fn unary(op: String) -> Result<TermKind, ASTError> {

    Ok(match op.as_str() {
        "!" => TermKind::Operator (
            OperatorKind::Unary(Unop::Not),
            Associativity::Right,
            6,
        ),
        "-" => TermKind::Operator (
            OperatorKind::Unary(Unop::Neg),
            Associativity::Right,
            6,
        ),
        _ => return Err(ASTError::InvalidOp(op.to_string())),
    })
}

/// Returns true if name is a reserved keyword
// not cached as name could be any variable name, not just limited subset
// of operators like other lookups
pub fn is_keyword(name: &str) -> bool {
    matches!(name, "fn" | "in" | "if" | "else" | "return" | "for" | "while" | "null")
}
