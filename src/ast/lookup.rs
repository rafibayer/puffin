use super::node::*;
use super::ASTError;

pub fn infix(op: &str) -> Result<TermKind, ASTError> {
    Ok(match op {
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

pub fn unary(op: &str) -> Result<TermKind, ASTError> {

    Ok(match op {
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

pub fn is_keyword(name: &str) -> bool {
    matches!(name, "fn" | "if" | "else" | "return" | "for" | "while" | "null")
}
