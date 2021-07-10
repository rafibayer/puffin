//! Author: Rafael Bayer (2021)
//! The operation module defines the behavior of various operations
//! in the Puffin language. 
use super::*;

/// Evaluates the infix operator op for a given left and right value.
/// Returns an `InterpreterError::UnexpectedType` error if the op is not applicable
/// for the given types.
pub fn infix(op: &InfixOp, lhs: Value, rhs: Value) -> Result<Value, InterpreterError> {
    Ok(match op {
        InfixOp::Mul => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num(lhs_float * rhs_float)
        },
        InfixOp::Mod => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num(lhs_float % rhs_float)
        },
        InfixOp::Div => {
            // todo: div by 0 check or just allow inf?
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num(lhs_float / rhs_float)
        },
        InfixOp::Plus => {
            match lhs {
                // numeric addition
                Value::Num(lhs_float) => {
                    let rhs_float: f64 = rhs.try_into()?;
                    Value::Num(lhs_float + rhs_float)
                },
                // String concatenation
                Value::String(lhs_str) => {
                    let rhs_str: String = rhs.try_into()?;
                    Value::String(lhs_str + rhs_str.as_str())
                },
                _ => return Err(unexpected_type(lhs))
            }
        },
        InfixOp::Minus => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num(lhs_float - rhs_float)
        },
        InfixOp::Lt => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num((lhs_float < rhs_float) as u32 as f64)
        },
        InfixOp::Gt => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num((lhs_float > rhs_float) as u32 as f64)
        },
        InfixOp::Le => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num((lhs_float <= rhs_float) as u32 as f64)
        },
        InfixOp::Ge => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num((lhs_float >= rhs_float) as u32 as f64)
        },
        InfixOp::Eq => {
            // Value supports eq
            Value::Num((lhs == rhs) as u32 as f64)
        },
        InfixOp::Ne => {
            Value::Num((lhs != rhs) as u32 as f64)
        },
        InfixOp::And => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num(((lhs_float.abs() > EPSILON) && (rhs_float.abs() > EPSILON)) as u32 as f64)
        },
        InfixOp::Or => {
            let lhs_float: f64 = lhs.try_into()?;
            let rhs_float: f64 = rhs.try_into()?;
            Value::Num(((lhs_float.abs() > EPSILON) || (rhs_float.abs() > EPSILON)) as u32 as f64)
        },
    })
}


pub fn unary(unop: &Unop, value: Value) -> Result<Value, InterpreterError> {
    Ok(match unop {
        Unop::Not => {
            let float: f64 = value.try_into()?;
            let result = (float as i32 == 0) as i32 as f64;
            Value::Num(result)
        }
        Unop::Neg => {
            let result: f64 = -(value.try_into()?);
            Value::Num(result)
        }
    })
}

// some postfix operations are more complicated and require environment,
// these operations are left in mod.rs and are handled by eval_postfix()