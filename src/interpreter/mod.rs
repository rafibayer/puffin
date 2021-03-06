//! Author: Rafael Bayer (2021)
//! The interpreter module defines the main entrypoint
//! to the Puffin interpreter, `eval`, as well as functions
//! to evaluate all parts of the Puffin AST.

use std::{
    cell::RefCell, collections::HashMap, convert::TryInto, f64::EPSILON, fmt::Display, rc::Rc,
    usize,
};

use crate::ast::node::*;
use value::Environment;
pub use value::Value;

use self::value::ClosureKind;

mod operations;
pub mod repl;
mod shunting_yard;
pub mod value;

/// Starting capacity for the expression evaluation stack.
const EXP_STACK_START_CAPACITY: usize = 4;

/// Interpreter error, essentially a Puffin Runtime error.
#[derive(Debug, Clone)]
pub enum InterpreterError {
    /// Usage of an unbound name
    UnboundName(String),
    /// Unexpected number of arguments to function
    ArgMismatch { expected: usize, got: usize },
    /// Type mismatch
    UnexpectedType(String),
    /// Attempted to rebind builtin name
    BuiltinRebinding(String),
    /// Error getting user input
    IOError(String),
    /// Array bounds error
    BoundsError { index: usize, size: usize },
    /// Range validity error
    RangeError { from: i128, to: i128 },
    /// User created error
    Error,
}

/// evaluates a program AST. Entrypoint of the interpreter
pub fn eval(program: &Program) -> Result<Value, InterpreterError> {
    eval_env(program, &Rc::new(RefCell::new(Environment::new())))
}

/// evaluates a program under a given environment
fn eval_env(program: &Program, env: &Rc<RefCell<Environment>>) -> Result<Value, InterpreterError> {
    for statement in &program.program {
        // if a statement has a value, it was a return statement,
        // we stop executing the program and return the value
        if let Some(return_val) = eval_statement(statement, env)? {
            return Ok(return_val);
        }
    }

    // if the program terminates without encountering a return in the body,
    // the program implicitly returns null
    Ok(Value::Null)
}

/// exactly the same as normal `eval_statement`, except we propagate expression values
/// as well as returns
fn eval_repl_statement(
    statement: &Statement,
    env: &Rc<RefCell<Environment>>,
) -> Result<Option<Value>, InterpreterError> {
    match &statement.statement {
        StatementKind::Return(exp) => return Ok(Some(eval_exp(exp, env)?)),
        StatementKind::Assign { lhs, rhs } => eval_assign(lhs, rhs, env),
        // repl version also returns expression values
        StatementKind::Exp(exp) => return Ok(Some(eval_exp(&exp, env)?)),
        StatementKind::Nest(nest) => match eval_nest(nest, env)? {
            Some(return_value) => return Ok(Some(return_value)),
            None => return Ok(None),
        },
    }?;

    Ok(None)
}

fn eval_statement(
    statement: &Statement,
    env: &Rc<RefCell<Environment>>,
) -> Result<Option<Value>, InterpreterError> {
    match &statement.statement {
        StatementKind::Return(exp) => return Ok(Some(eval_exp(exp, env)?)),
        StatementKind::Assign { lhs, rhs } => eval_assign(lhs, rhs, env),
        StatementKind::Exp(exp) => eval_exp(&exp, env),
        StatementKind::Nest(nest) => match eval_nest(nest, env)? {
            // if a nest statement has a value, it had a return statement,
            // we propagate this to the caller so they can return (or program).
            Some(return_value) => return Ok(Some(return_value)),
            None => return Ok(None),
        },
    }?;

    Ok(None)
}

fn eval_postfix(
    postop: &PostOp,
    value: Value,
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    Ok(match postop {
        PostOp::Subscript(exp) => eval_subscript(exp, value, env)?,
        PostOp::Call(exps) => eval_call(value, exps, env)?,
        PostOp::Dot(name) => eval_dot(value, name)?,
    })
}

fn eval_subscript(
    index_exp: &Exp,
    value: Value,
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    let index_float: f64 = eval_exp(index_exp, env)?.try_into()?;
    let index = index_float as usize;
    Ok(match value {
        // Array subscript
        Value::Array(arr) => {
            if index >= arr.borrow().len() {
                return Err(InterpreterError::BoundsError {
                    index,
                    size: arr.borrow().len(),
                });
            }
            arr.borrow()[index].clone()
        }
        // String subscript
        Value::String(string) => {
            if index >= string.len() {
                return Err(InterpreterError::BoundsError {
                    index,
                    size: string.len(),
                });
            }
            Value::from((string.as_bytes()[index] as char).to_string())
        }
        _ => return Err(unexpected_type(value)),
    })
}

fn eval_call(
    callable: Value,
    exps: &[Exp],
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    Ok(match &callable {
        // function/closure call
        Value::Closure {
            kind,
            args,
            block,
            environment,
        } => {
            // ensure the call has the appropriate number of args for the function
            if exps.len() != args.len() {
                return Err(InterpreterError::ArgMismatch {
                    expected: args.len(),
                    got: exps.len(),
                });
            }

            let subenv = Rc::new(RefCell::new(Environment::new_sub(&environment)));

            // bind the args to the actuals
            for i in 0..args.len() {
                let actual = eval_exp(&exps[i], env)?;
                subenv.borrow_mut().bind(&args[i], actual)?;
            }

            // if the function was named, bind its name to itself to allow recursion
            if let ClosureKind::Named(name) = kind {
                subenv.borrow_mut().bind(name, callable.clone())?;
            // if the function was a receiver of a structure, bind the structure to "self"
            } else if let ClosureKind::Receiver(structure) = kind {
                subenv
                    .borrow_mut()
                    .bind("self", Value::Structure(structure.clone()))?;
            }

            // evaluate the closures body.
            // if the block evaluates to none, the implicit result is null
            eval_block(&block, &subenv)?.unwrap_or(Value::Null)
        }
        // builtin call
        Value::Builtin(f) => {
            let mut actuals = Vec::with_capacity(exps.len());
            // evaluate the actuals
            for actual in exps {
                actuals.push(eval_exp(&actual, env)?);
            }

            // call the builtin function body with the actuals.
            // the function body is responsible for validating number of args
            // for builtins, which allows dynamic number of args for certain builtins
            (f.body)(actuals)?
        }
        _ => {
            return Err(unexpected_type(callable));
        }
    })
}

fn eval_dot(dotable: Value, name: &str) -> Result<Value, InterpreterError> {
    Ok(match dotable {
        Value::Structure(map) => match map.borrow().get(name) {
            Some(value) => value.clone(),
            None => return Err(InterpreterError::UnboundName(name.to_string())),
        },
        _ => return Err(unexpected_type(dotable)),
    })
}

fn eval_exp(exp: &Exp, env: &Rc<RefCell<Environment>>) -> Result<Value, InterpreterError> {
    // use the shunting yard algorithm to convert the expression to postfix notation
    let mut rpn_queue = shunting_yard::as_rpn_queue(exp);

    // we then evaluate the postfix expression using a stack
    let mut stack: Vec<Value> = Vec::with_capacity(EXP_STACK_START_CAPACITY);

    // evaluate rpn
    while !rpn_queue.is_empty() {
        let top = rpn_queue.pop_front().unwrap();
        let result = match top {
            // evaluate operators
            TermKind::Operator(op, _, _) => match op {
                // unary (prefix) operators
                OperatorKind::Unary(unop) => {
                    let value = stack.pop().unwrap();
                    operations::unary(unop, value)?
                }
                // infix operators
                OperatorKind::Infix(infix) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    operations::infix(infix, left, right)?
                }
                // postfix operators
                OperatorKind::Postfix(postop) => {
                    let next = stack.pop().unwrap();
                    eval_postfix(postop, next, env)?
                }
            },
            // values get evaluated and pushed onto the stack
            TermKind::Value(v) => eval_value(v, env)?,
        };

        stack.push(result);
    }

    // after evaluating the expression, the final value on the stack is
    // the expressions result
    assert_eq!(1, stack.len());
    Ok(stack.pop().unwrap())
}

fn eval_value(
    value: &ValueKind,
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    match value {
        ValueKind::Paren(exp) => eval_exp(exp, env),
        ValueKind::Structure(fields) => {
            let map = Rc::new(RefCell::new(HashMap::with_capacity(fields.len())));
            for field in fields {
                let mut field_value = eval_exp(&field.exp, env)?;

                // if the fields value is a closure, check if the first argument is "self"
                if let Value::Closure {
                    args,
                    block,
                    environment,
                    ..
                } = &field_value
                {
                    // if it is, the closure is a receiver of the structure
                    if args.first() == Some(&"self".to_string()) {
                        // take out the "self" argument, leaving the remaining args
                        let other_args: Vec<String> = args[1..].to_vec();
                        field_value = Value::Closure {
                            kind: ClosureKind::Receiver(map.clone()),
                            args: other_args,
                            block: block.clone(),
                            environment: environment.clone(),
                        }
                    }
                }

                map.borrow_mut().insert(field.name.clone(), field_value);
            }

            Ok(Value::Structure(map))
        }
        ValueKind::FunctionDef { args, block } => {
            // functions evaluate to a closure that captures the local environment.
            // by default, closures are anonymous (self_name = None).
            // kind is changed later by eval_assign if we are binding this closure to a name,
            // or by eval_value if we are binding to a structure field
            Ok(Value::Closure {
                kind: ClosureKind::Anonymous,
                args: args.clone(),
                block: block.clone(),
                environment: env.clone(),
            })
        }
        ValueKind::Num(n) => Ok(Value::Num(*n)),
        ValueKind::String(string) => Ok(Value::String(string.clone())),
        ValueKind::ArrayInit(init_exp) => match init_exp {
            ArrayInitKind::Sized(size_exp) => {
                let size_float: f64 = eval_exp(size_exp, env)?.try_into()?;
                let size = size_float as usize;
                Ok(Value::Array(Rc::new(RefCell::new(vec![Value::Null; size]))))
            }
            ArrayInitKind::Range(from_exp, to_exp) => {
                let from_float: f64 = eval_exp(from_exp, env)?.try_into()?;
                let from = from_float as i128;

                let to_float: f64 = eval_exp(to_exp, env)?.try_into()?;
                let to = to_float as i128;

                if from > to {
                    return Err(InterpreterError::RangeError { from, to });
                }

                let vec: Vec<Value> = (from..to).map(|e| Value::from(e as f64)).collect();
                Ok(Value::Array(Rc::new(RefCell::new(vec))))
            }
        },
        ValueKind::Name(name) => env.borrow().get(name),
        ValueKind::Null => Ok(Value::Null),
    }
}

fn eval_block(
    block: &Block,
    env: &Rc<RefCell<Environment>>,
) -> Result<Option<Value>, InterpreterError> {
    for statement in &block.block {
        // propagate return statements
        if let Some(return_value) = eval_statement(statement, env)? {
            return Ok(Some(return_value));
        }
    }

    Ok(None)
}

fn eval_assign(
    lhs: &Assignable,
    rhs: &Exp,
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    let name = lhs.name.clone();
    let subassignment = &lhs.assignable;

    // simple assignment to name (a = something),
    // no subassignment (like a[5], or a.b)
    if subassignment.is_empty() {
        let value = eval_exp(&rhs, env)?;

        // if we are binding a closure, convert to a named closure
        if let Value::Closure {
            args,
            block,
            environment,
            ..
        } = value
        {
            let func_bind = Value::Closure {
                kind: ClosureKind::Named(name.clone()),
                args,
                block,
                environment,
            };
            return env.borrow_mut().bind(&name, func_bind);
        }

        return env.borrow_mut().bind(&name, value);
    }

    // otherwise we need to recursively assign to arrays/structures
    let mut bound = env.borrow().get(&name)?;
    let rhs = eval_exp(&rhs, env)?;

    bound = assign_drilldown(bound, subassignment, rhs, env)?;

    env.borrow_mut().bind(&name, bound)
}

/// recursive assignment for nested bindings.
/// assign_to is the current thing being assigned to this iteration
/// assigments is the list of next things to assign to.
/// rhs is the final value being bound
/// For example, in the expression `thing.field[5] = 7`;
/// In the first iteration, `assign_to = thing`, `assignments = [ field, [5] ]`, and `rhs = 7`.
fn assign_drilldown(
    assign_to: Value,
    assignments: &[AssignableKind],
    rhs: Value,
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    // base case, we have reached the last assignable, and we return the final value back
    if assignments.is_empty() {
        return Ok(rhs);
    }

    // next thing we are assigning to, either an array index or a structure field
    let next = &assignments[0];
    match next {
        AssignableKind::ArrayIndex { index } => {
            if let Value::Array(arr) = assign_to {
                // compute the index to assign to
                let index_val: f64 = eval_exp(&index, env)?.try_into()?;
                let index_val = index_val as usize;

                if index_val >= arr.borrow().len() {
                    return Err(InterpreterError::BoundsError {
                        index: index_val,
                        size: arr.borrow().len(),
                    });
                }

                // temporarily replace the value at the index with null so we can modify it
                // this is preferable to removing it, because replace is O(1) as it does not cause a shift
                // in all the other elements which would be o(n)
                let inner_value = std::mem::replace(&mut arr.borrow_mut()[index_val], Value::Null);

                // re-insert after assinging to the inner value
                arr.borrow_mut()[index_val] =
                    assign_drilldown(inner_value, &assignments[1..], rhs, env)?;

                return Ok(Value::Array(arr));
            }
            Err(unexpected_type(assign_to))
        }
        AssignableKind::StructureField { field } => {
            if let Value::Structure(structure) = assign_to {
                // todo: this remove may trigger an expensive resize, any way to
                // do this via swapping like in the array case? potentially could also use
                // get_mut
                let inner_value = match structure.borrow_mut().remove(field) {
                    Some(f) => f,
                    // or create the new struct
                    None => Value::from(HashMap::new()),
                };

                structure.borrow_mut().insert(
                    field.clone(),
                    assign_drilldown(inner_value, &assignments[1..], rhs, env)?,
                );

                return Ok(Value::Structure(structure));
            }
            Err(unexpected_type(assign_to))
        }
    }
}

fn eval_nest(
    nest: &NestKind,
    env: &Rc<RefCell<Environment>>,
) -> Result<Option<Value>, InterpreterError> {
    match nest {
        NestKind::CondNest(condnest) => match condnest {
            CondNestKind::IfElse {
                cond,
                then,
                or_else,
            } => {
                let cond_value: f64 = eval_exp(&cond, env)?.try_into()?;
                if cond_value as i64 != 0 {
                    let then_res = eval_block(then, env)?;
                    return Ok(then_res);
                }
                let or_else_res = eval_block(or_else, env)?;
                Ok(or_else_res)
            }
            CondNestKind::If { cond, then } => {
                let cond_value: f64 = eval_exp(&cond, env)?.try_into()?;
                if cond_value as i64 != 0 {
                    let then_res = eval_block(then, env)?;
                    return Ok(then_res);
                }
                Ok(None)
            }
        },
        NestKind::LoopNest(loopnest) => match loopnest {
            LoopNestKind::While { cond, block } => {
                let mut while_cond: f64 = eval_exp(&cond, env)?.try_into()?;
                while while_cond as i64 != 0 {
                    if let Some(return_result) = eval_block(block, env)? {
                        return Ok(Some(return_result));
                    }
                    while_cond = eval_exp(&cond, env)?.try_into()?;
                }
                Ok(None)
            }
            LoopNestKind::For {
                init,
                cond,
                adv,
                block,
            } => {
                eval_statement(init, env)?;
                let mut for_cond: f64 = eval_exp(&cond, env)?.try_into()?;
                while for_cond as i64 != 0 {
                    if let Some(return_result) = eval_block(block, env)? {
                        return Ok(Some(return_result));
                    }
                    eval_statement(adv, env)?;
                    for_cond = eval_exp(&cond, env)?.try_into()?;
                }
                Ok(None)
            }
            LoopNestKind::ForIn { name, array, block } => {
                let array = eval_exp(&array, env)?;
                let vector = match array {
                    Value::Array(v) => v,
                    other => return Err(unexpected_type(other)),
                };

                let mut index: usize = 0;
                while index < vector.borrow().len() {
                    env.borrow_mut()
                        .bind(name, vector.borrow()[index].clone())?;
                    if let Some(return_result) = eval_block(block, env)? {
                        return Ok(Some(return_result));
                    }
                    index += 1;
                }
                Ok(None)
            }
        },
    }
}

//#[track_caller]
fn unexpected_type(value: Value) -> InterpreterError {
    //let caller = std::panic::Location::caller();
    //eprintln!("unexpected type: {:#?}, {}:{}", &value, caller.file(), caller.line());
    InterpreterError::UnexpectedType(format!("{}", value))
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<std::io::Error> for InterpreterError {
    fn from(io_err: std::io::Error) -> Self {
        InterpreterError::IOError(io_err.to_string())
    }
}
