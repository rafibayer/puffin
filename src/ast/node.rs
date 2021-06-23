use std::convert::TryInto;

use crate::interpreter::{self, InterpreterError, value::environment};

use super::ASTError;


#[derive(Debug, Clone)]
pub struct Program {
    pub program: Vec<Statement>
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub statement: StatementKind
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Return(Exp),
    Assign{lhs: Assingnable, rhs: Exp},
    Exp(Exp),
    Nest(NestKind)
}

#[derive(Debug, Clone)]
pub struct Assingnable {
    pub name: String,
    pub assignable: Vec<AssignableKind>
}

#[derive(Debug, Clone)]
pub enum AssignableKind {
    ArrayIndex{index: Exp},
    StructureField{field: String},
}


#[derive(Debug, Clone)]
pub struct Exp {
    pub exp: Vec<TermKind>
}

#[derive(Debug, Clone)]
pub enum TermKind {
    Operator(OperatorKind, Associativity, usize),
    Value(ValueKind)
}

#[derive(Debug, Clone)]
pub enum Associativity {
    Left,
    Right
}

#[derive(Debug, Clone)]
pub enum ValueKind {
    Paren(Box<Exp>),
    Structure(Vec<Field>),
    Function{args: Vec<String>, block: Block},
    Num(f64),
    String(String),
    ArrayInit(Box<Exp>),
    Name(String),
    Null
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub exp: Exp
}


#[derive(Debug, Clone)]
pub struct Block {
    pub block: Vec<Statement>
}

#[derive(Debug, Clone)]
pub enum NestKind {
    CondNest(CondNestKind),
    LoopNest(LoopNestKind)
}

#[derive(Debug, Clone)]
pub enum CondNestKind {
    IfElse{cond: Exp, then: Block, or_else: Block},
    If{cond: Exp, then: Block},
}

#[derive(Debug, Clone)]
pub enum OperatorKind {
    Unary(Unop),
    Infix(InfixOp),
    Postfix(PostOp),
}

#[derive(Debug, Clone)]
pub enum LoopNestKind {
    While{cond: Exp, block: Block},
    // todo: adv could be an expression too?
    For{init: Box<Statement>, cond: Exp, adv: Box<Statement>, block: Block}
}

#[derive(Debug, Clone)]
pub enum PostOp {
    Subscript(Box<Exp>),
    Call(Vec<Exp>),
    Dot(String)
}

#[derive(Debug, Clone)]
pub enum InfixOp {
    Mul,
    Mod,
    Div,
    Plus,
    Minus,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    And,
    Or
}

#[derive(Debug, Clone)]
pub enum Unop {
    Not,
    Neg,
}