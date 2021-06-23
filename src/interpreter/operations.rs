use super::*;


pub fn infix(op: InfixOp, lhs: Value, rhs: Value) -> Result<Value, InterpreterError> {
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
            // numeric addition or string concat
            match lhs {
                Value::Num(lhs_float) => {
                    let rhs_float: f64 = rhs.try_into()?;
                    Value::Num(lhs_float + rhs_float)
                },
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
            Value::Num((lhs == rhs) as u32 as f64)
        },
        InfixOp::Ne => {
            if let Value::Num(bool) = infix(InfixOp::Eq, lhs, rhs)? {
                return Ok(Value::Num((bool == 0f64) as u32 as f64));
            }
            unreachable!();
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