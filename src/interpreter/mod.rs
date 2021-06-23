use std::{collections::HashMap, convert::TryInto, f64::EPSILON, usize};

use crate::ast::node::*;
use value::{Environment, Value};

mod shunting_yard;
pub mod value;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    UnboundName(String),
    ArgMismatch {
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

    let mut stack: Vec<Value> = Vec::new();

    // evaluate rpn
    while !rpn_queue.is_empty() {
        let top = rpn_queue.remove(0);
        match top {
            TermKind::Operator(op, _, _) => match op {
                OperatorKind::Unary(unop) => {
                    let next = stack.remove(0);
                    
                    match unop {
                        Unop::Not => {
                            let float: f64 = next.try_into()?;
                            let result = !(float as i32 != 0) as i32 as f64;
                            stack.push(Value::Num(result));
                        }
                        Unop::Neg => {
                           
                            let result: f64 = -next.try_into()?;
                            stack.push(Value::Num(result));
                        }
                    }
                },
                OperatorKind::Infix(infix) => {
                    // convert the top 2 values on the stack into numbers.
                    // This serves as a type check, to make sure that the operators can be applied
                    // to the given values

                    let left: f64 = stack.remove(0).try_into()?;
                    let right: f64 = stack.remove(0).try_into()?;
                   
                    stack.push(Value::Num(match infix {
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
                    }));
                }
                OperatorKind::Postfix(postop) => {
                    let next = stack.remove(0);
                    match postop {
                        PostOp::Subscript(exp) => {
                            match next {
                                Value::Array(arr) => {
                                    let index: f64 = eval_exp(*exp, env)?.try_into()?;
                                    stack.push(arr[index as usize].clone());
                                },
                                _ => return Err(unexpected_type(next))
                            }
                        },
                        PostOp::Call(exps) => {
                            match next {
                                Value::Function { args, block, mut closure } => {
                                    if exps.len() != args.len() {
                                        return Err(InterpreterError::ArgMismatch{ expected: args.len(), got: exps.len() })
                                    }

                                    for i in 0..args.len() {
                                        closure.bind(args[i].clone(), &eval_exp(exps[i].clone(), env)?)?;
                                    }

                                    let result = eval_block(block, &mut closure)?.unwrap_or(Value::Null);
                                    stack.push(result);
                                },
                                Value::Builtin(f) => {
                                    let mut actuals = Vec::with_capacity(exps.len());
                                    for actual in exps {
                                        actuals.push(eval_exp(actual, env)?);
                                    }

                                    let result = (f.body)(actuals)?;
                                    stack.push(result);
                                },
                                _ => return Err(unexpected_type(next))
                            }
                        },
                        PostOp::Dot(name) => {
                            match next {
                                Value::Structure(map) => {

                                }
                                _ => return Err(unexpected_type(next))
                            } 
                        },
                    }
                },
            },
            TermKind::Value(v) => {
                stack.push(eval_value(v, env)?);
            }
        }
    }

    Ok(stack.remove(0))
}

fn eval_value(value: ValueKind, env: &mut Environment) -> Result<Value, InterpreterError> {
    match value {
        ValueKind::Paren(exp) => eval_exp(*exp, env),
        ValueKind::Structure(fields) => {
            let mut map = HashMap::new();
            for field in fields {
                map.insert(field.name, eval_exp(field.exp, env)?);
            }
            Ok(Value::Structure(map))
        },
        ValueKind::Function { args, block } => {
            Ok(Value::Function{ args, block, closure: env.clone() })
        },
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

fn eval_block(block: Block, env: &mut Environment) -> Result<Option<Value>, InterpreterError> {
    for statement in block.block {
        match eval_statement(statement, env)? {
            Some(return_value) => return Ok(Some(return_value)),
            None => {}
        }
    }

    Ok(None)
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
        let value = eval_exp(rhs, &mut env_clone)?;

        // if we are binding a function to a name, bind function to it's own name in it's closure
        if let Value::Function{args, block, mut closure} = value.clone() {
            closure.bind(name.clone(), &value)?;
            return env.bind(name, &Value::Function{ args, block, closure})
        }
        
        return env.bind(name, &value);
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
        AssignableKind::StructureField { field } => {
            if let Value::Structure(mut structure) = assign_to {
                let mut new_struct = structure.clone();

                // either assign to substruct
                let assign_to = match new_struct.remove(&field) {
                    Some(f) => f,
                    // or create the new struct
                    None => Value::Structure(HashMap::new()),
                };
                structure.insert(field,
                    assign_drilldown(assign_to, assignments, rhs, env)?
                );

                return Ok(Value::Structure(structure));
            }
            Err(unexpected_type(assign_to))

        },
    }
}

fn eval_nest(nest: NestKind, env: &mut Environment) -> Result<Option<Value>, InterpreterError> {
    match nest {
        NestKind::CondNest(condnest) => {
            match condnest {
                CondNestKind::IfElse { cond, then, or_else } => {
                    let cond_value: f64 = eval_exp(cond, env)?.try_into()?;
                    if cond_value as i64 != 0 {
                        let then_res = eval_block(then, env)?;
                        return Ok(then_res);
                    } 
                    let or_else_res = eval_block(or_else, env)?;
                    return Ok(or_else_res);
                },
                CondNestKind::If { cond, then } => {
                    let cond_value: f64 = eval_exp(cond, env)?.try_into()?;
                    if cond_value as i64 != 0 {
                        let then_res = eval_block(then, env)?;
                        return Ok(then_res);
                    }
                    return Ok(None);
                },
            }
        },
        NestKind::LoopNest(loopnest) => {
            match loopnest {
                LoopNestKind::While { cond, block } => {
                    let mut while_cond: f64 = eval_exp(cond.clone(), env)?.try_into()?;
                    while while_cond as i64 != 0 {
                        if let Some(return_result) = eval_block(block.clone(), env)? {
                            return Ok(Some(return_result));
                        }
                        while_cond = eval_exp(cond.clone(), env)?.try_into()?;
                    }
                    Ok(None)
                },
                LoopNestKind::For { init, cond, adv, block } =>{
                    eval_statement(*init, env)?;
                    let mut for_cond: f64 = eval_exp(cond.clone(), env)?.try_into()?;
                    while for_cond as i64 != 0 {
                        if let Some(return_result) = eval_block(block.clone(), env)? {
                            return Ok(Some(return_result));
                        }
                        eval_statement(*adv.clone(), env)?;
                        for_cond = eval_exp(cond.clone(), env)?.try_into()?;
                    }
                    Ok(None)
                },
            }
        },
    }
}

#[track_caller]
fn unexpected_type(value: Value) -> InterpreterError {
    let caller_line_number = std::panic::Location::caller().line();
    eprintln!("unexpected type: {}:{}", std::panic::Location::caller().file(), caller_line_number);
    InterpreterError::UnexpectedType(format!("{:?}", value))
}

fn unexpected_operator(op: OperatorKind) -> InterpreterError {
    InterpreterError::UnexepectedOperator(format!("{:?}", op))
}
