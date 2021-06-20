
#[derive(Debug)]
pub struct Program {
    pub program: Vec<Statement>
}

#[derive(Debug)]
pub struct Statement {
    pub statement: StatementKind
}

#[derive(Debug)]
pub enum StatementKind {
    Return(Exp),
    Assign{lhs: Exp, rhs: Exp},
    Exp(Exp),
    Nest(NestKind)
}

#[derive(Debug)]
pub struct Exp {
    pub exp: ExpKind
}

#[derive(Debug)]
pub enum ExpKind {
    Paren(Box<Exp>),
    Infix{term: Term, op_terms: Vec<(OpKind, Term)>},
    Term(Term)
}

#[derive(Debug)]
pub struct Term {
    pub term: TermKind
}

#[derive(Debug)]
pub enum TermKind {
    Paren(Box<Exp>),
    UnopUse{unop: UnopKind, term: Box<Term>},
    Function{args: Vec<String>, body: Block},
    FunctionCall{name: String, exps: Vec<Exp>},
    ArrayIndex{name: String, exp: Box<Exp>},
    ArrayInit{size: Box<Exp>},
    Name(String),
    Num(f64),
    String(String),
}

#[derive(Debug)]
pub struct Block {
    block: Vec<Statement>
}

#[derive(Debug)]
pub struct Nest {
    nest: NestKind
}

#[derive(Debug)]
pub enum NestKind {
    CondNest(CondNestKind),
    LoopNest(LoopNestKind)
}

#[derive(Debug)]
pub enum CondNestKind {
    IfElse{cond: Exp, then: Block, or_else: Block},
    If{cond: Exp, then: Block},
}

#[derive(Debug)]
pub enum LoopNestKind {
    While{cond: Exp, block: Block},
    // todo: adv could be an expression too?
    For{init: Box<Statement>, cond: Exp, adv: Box<Statement>, block: Block}
}

#[derive(Debug)]
pub enum OpKind {
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

#[derive(Debug)]
pub enum UnopKind {
    Not,
    Neg,
}