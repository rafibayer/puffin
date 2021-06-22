use std::{convert::TryInto, f64::EPSILON, usize};

use crate::ast::node::*;
use value::{Environment, Value};

mod shunting_yard;
pub mod value;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    UnboundName(String),
    ArgMismatch {
        name: String,
        expected: usize,
        got: usize,
    },
    UnexpectedType(String),
    BuiltinRebinding(String),
    UnexepectedOperator(String),
}

pub fn eval(program: Program) -> Result<Value, InterpreterError> {
    eval_env(program, &mut Environment::new())
}

fn eval_env(program: Program, env: &mut Environment) -> Result<Value, InterpreterError> {
    for statement in program.program {
        match eval_statement(statement, env)? {
            // if the statement results in a value, we stop executing and return in
            Some(return_val) => return Ok(return_val),
            // otherwise, we continue
            None => {}
        }
    }

    // if the program terminates without encountering a return in the body,
    // the program implicitly returns null
    Ok(Value::Null)
}

fn eval_statement(
    statement: Statement,
    env: &mut Environment,
) -> Result<Option<Value>, InterpreterError> {
    match statement.statement {
        StatementKind::Return(exp) => return Ok(Some(eval_exp(exp, env)?)),
        StatementKind::Assign { lhs, rhs } => eval_assign(lhs, rhs, env),
        StatementKind::Exp(exp) => eval_exp(exp, env),
        StatementKind::Nest(nest) => match eval_nest(nest, env)? {
            Some(return_value) => return Ok(Some(return_value)),
            None => return Ok(None),
        },
    }?;

    Ok(None)
}

fn eval_exp(exp: Exp, env: &mut Environment) -> Result<Value, InterpreterError> {
    let mut rpn_queue = shunting_yard::to_rpn(exp);
    dbg!(&rpn_queue);

    let mut stack: Vec<TermKind> = Vec::new();

    // evaluate rpn
    while !rpn_queue.is_empty() {
        let top = rpn_queue.remove(0);
        match top {
            TermKind::Operator(op, _, _) => match op {
                OperatorKind::Unary(unop) => {
                    let next = eval_value(stack.remove(0).try_into()?, env)?;
                    
                    match unop {
                        Unop::Not => {
                            let float: f64 = next.try_into()?;
                            let result = ValueKind::Num(!(float as i32 != 0) as i32 as f64);
                            stack.push(TermKind::Value(result));
                        }
                        Unop::Neg => {
                           
                            let result = ValueKind::Num(-next.try_into()?);
                            stack.push(TermKind::Value(result));
                        }
                    }
                },
                OperatorKind::Infix(infix) => {
                    // convert the top 2 values on the stack into numbers.
                    // This serves as a type check, to make sure that the operators can be applied
                    // to the given values

                    let left: f64 = eval_value(stack.remove(0).try_into()?, env)?.try_into()?;
                    let right: f64 = eval_value(stack.remove(0).try_into()?, env)?.try_into()?;

                   
                    stack.push(TermKind::Value(ValueKind::Num(match infix {
                        InfixOp::Mul => left * right,
                        InfixOp::Mod => left % right,
                        InfixOp::Div => left / right,
                        InfixOp::Plus => left + right,
                        InfixOp::Minus => left - right,
                        InfixOp::Lt => (left < right) as u32 as f64,
                        InfixOp::Gt => (left > right) as u32 as f64,
                        InfixOp::Le => (left <= right) as u32 as f64,
                        InfixOp::Ge => (left >= right) as u32 as f64,
                        InfixOp::Eq => ((left - right) < EPSILON) as u32 as f64,
                        InfixOp::Ne => ((left - right) > EPSILON) as u32 as f64,
                        InfixOp::And => ((left > EPSILON) && (right > EPSILON)) as u32 as f64,
                        InfixOp::Or => ((left > EPSILON) || (right > EPSILON)) as u32 as f64,
                    })));
                }
                OperatorKind::Postfix(postop) => {
                    let left: f64 = eval_value(stack.remove(0).try_into()?, env)?.try_into()?;
                    match postop {
                        PostOp::Subscript(exp) => todo!(),
                        PostOp::Call(args) => todo!(),
                        PostOp::Dot(name) => todo!(),
                    }
                },
            },
            TermKind::Value(_) => {
                stack.push(top);
            }
        }
    }

    let result = stack.remove(0);
    match result {
        TermKind::Operator(op, _, _) => Err(unexpected_operator(op)),
        TermKind::Value(v) => eval_value(v, env),
    }
}

fn eval_value(value: ValueKind, env: &mut Environment) -> Result<Value, InterpreterError> {
    match value {
        ValueKind::Paren(exp) => eval_exp(*exp, env),
        ValueKind::Structure(_) => todo!(),
        ValueKind::Function { args, block } => todo!(),
        ValueKind::Num(n) => Ok(Value::Num(n)),
        ValueKind::String(string) => Ok(Value::String(string)),
        ValueKind::ArrayInit(size_exp) => {
            let size_float: f64 = eval_exp(*size_exp, env)?.try_into()?;
            let size = size_float as usize;
            Ok(Value::Array(vec![Value::Num(0f64); size]))
        },
        ValueKind::Name(name) => {
            env.get(name)
        },
        ValueKind::Null => Ok(Value::Null),
    }
}

fn eval_assign(
    lhs: Assingnable,
    rhs: Exp,
    env: &mut Environment,
) -> Result<Value, InterpreterError> {
    let name = lhs.name;
    let subassignment = lhs.assignable;

    // simple assignment to name (a = something),
    // no subassignment (like a[5], or a.b)
    if subassignment.is_empty() {
        // we cannot re-borrow env for the exp eval, so we clone it first,
        // then we bind the name
        let mut env_clone = env.clone();
        return env.bind(name, &eval_exp(rhs, &mut env_clone)?);
    }

    let mut bound = env.get(name.clone())?;
    bound = assign_drilldown(bound, subassignment, rhs, env)?;

    env.bind(name, &bound)
}

fn assign_drilldown(assign_to: Value, mut assignments: Vec<AssignableKind>, rhs: Exp, env: &mut Environment) -> Result<Value, InterpreterError> {
    if assignments.is_empty() {
        return eval_exp(rhs, env);
    }
    let next = assignments.remove(0);
    match next {
        AssignableKind::ArrayIndex { index } => {
            if let Value::Array(mut arr) = assign_to {
                let index_val: f64 = eval_exp(index, env)?.try_into()?;
                let new_arr = arr.clone();
                arr[index_val as usize] = assign_drilldown(
                    new_arr[index_val as usize].clone(),
                    assignments,
                    rhs,
                    env,
                )?;

                return Ok(Value::Array(arr));
            }
            Err(unexpected_type(assign_to))
        }
        AssignableKind::StructureField { field } => todo!(),
    }
}

fn eval_nest(nest: NestKind, env: &mut Environment) -> Result<Option<Value>, InterpreterError> {
    todo!()
}

fn unexpected_type(value: Value) -> InterpreterError {
    InterpreterError::UnexpectedType(format!("{:?}", value))
}

fn unexpected_operator(op: OperatorKind) -> InterpreterError {
    InterpreterError::UnexepectedOperator(format!("{:?}", op))
}
