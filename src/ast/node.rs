
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
    Assign{lhs: Exp, rhs: Exp},
    Exp(Exp),
    Nest(NestKind)
}

#[derive(Debug, Clone)]
pub struct Exp {
    pub exp: ExpKind
}

#[derive(Debug, Clone)]
pub enum ExpKind {
    Paren(Box<Exp>),
    Infix{term: Term, op_terms: Vec<(OpKind, Term)>},
    Term(Term)
}

#[derive(Debug, Clone)]
pub struct Term {
    pub term: TermKind
}

#[derive(Debug, Clone)]
pub enum TermKind {
    Paren(Box<Exp>),
    UnopUse{unop: UnopKind, term: Box<Term>},
    Function{args: Vec<String>, body: Block},
    FunctionCall{name: Box<Term>, exps: Vec<Exp>},
    ArrayIndex{subscriptable: Box<Term>, exps: Vec<Exp>},
    ArrayInit{size: Box<Exp>},
    Name(String),
    Num(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub block: Vec<Statement>
}

#[derive(Debug, Clone)]
pub struct Nest {
    nest: NestKind
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
pub enum LoopNestKind {
    While{cond: Exp, block: Block},
    // todo: adv could be an expression too?
    For{init: Box<Statement>, cond: Exp, adv: Box<Statement>, block: Block}
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnopKind {
    Not,
    Neg,
}