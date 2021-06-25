/*!
This module contains definitons for all AST nodes.
The root node is the struct Program, 
which contains a vector of all the programs statements
*/


#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub program: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub statement: StatementKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Return(Exp),
    Assign { lhs: Assingnable, rhs: Exp },
    Exp(Exp),
    Nest(NestKind),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assingnable {
    pub name: String,
    pub assignable: Vec<AssignableKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignableKind {
    ArrayIndex { index: Exp },
    StructureField { field: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exp {
    pub exp: Vec<TermKind>,
}

type Precedence = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum TermKind {
    Operator(OperatorKind, Associativity, Precedence),
    Value(ValueKind),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    Paren(Box<Exp>),
    Structure(Vec<Field>),
    FunctionDef { args: Vec<String>, block: Block },
    Num(f64),
    String(String),
    ArrayInit(Box<Exp>),
    Name(String),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub exp: Exp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub block: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NestKind {
    CondNest(CondNestKind),
    LoopNest(LoopNestKind),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CondNestKind {
    IfElse {
        cond: Exp,
        then: Block,
        or_else: Block,
    },
    If {
        cond: Exp,
        then: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorKind {
    Unary(Unop),
    Infix(InfixOp),
    Postfix(PostOp),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoopNestKind {
    While {
        cond: Exp,
        block: Block,
    },
    // todo: adv could be an expression too?
    For {
        init: Box<Statement>,
        cond: Exp,
        adv: Box<Statement>,
        block: Block,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostOp {
    Subscript(Box<Exp>),
    Call(Vec<Exp>),
    Dot(String),
}

#[derive(Debug, Clone, PartialEq)]
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
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unop {
    Not,
    Neg,
}
