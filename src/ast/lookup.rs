use super::node::*;
use super::ASTError;


pub fn infix(op: &str) -> Result<InfixOp, ASTError> {
    Ok(match op {
        "<" => InfixOp::Lt,
        "<=" => InfixOp::Le,
        ">" => InfixOp::Gt,
        ">=" => InfixOp::Ge,
        "==" => InfixOp::Eq,
        "!=" => InfixOp::Ne,

        "-" => InfixOp::Minus,
        "+" => InfixOp::Plus,

        "/" => InfixOp::Div,
        "%" => InfixOp::Mod,
        "*" => InfixOp::Mul,

        _ => return Err(ASTError::InvalidOp(op.to_string()))
    })
}

pub fn unary(op: &str) -> Result<Unop, ASTError> {
    Ok(match op {
        "!" => Unop::Not,
        "-" => Unop::Neg,
        _ => return Err(ASTError::InvalidOp(op.to_string()))
    })
}

pub fn is_keyword(name: &str) -> bool {
    match name {
        "fn" => true,
        "if" => true,
        "else" => true,
        "return" => true,
        "for" => true,
        "while" => true,
        _ => false
    }
}