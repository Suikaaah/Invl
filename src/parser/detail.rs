use std::collections::LinkedList;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    Array(usize),
    List,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Xor,
    Mul,
    Div,
    Remainder,
    BitwiseAnd,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy)]
pub enum UnrOp {
    Negative,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum MutOp {
    Add,
    Sub,
    Xor,
    Swap,
}

#[derive(Debug, Clone)]
pub struct Variable(pub String);

#[derive(Debug, Clone)]
pub struct ProcId(pub String);

#[derive(Debug)]
pub struct Program(pub MainProc, pub LinkedList<Proc>);

#[derive(Debug, Clone)]
pub struct TypedVariable(pub Type, pub Variable);

#[derive(Debug)]
pub struct MainProc(pub LinkedList<TypedVariable>, pub Statement);

#[derive(Debug)]
pub struct Proc(pub ProcId, pub LinkedList<TypedVariable>, pub Statement);

#[derive(Debug)]
pub enum Statement {
    Mut(Variable, MutOp, Expr),
    IndexedMut(Variable, Expr, MutOp, Expr),
    IfThenElseFi(Expr, Box<Statement>, Box<Statement>, Expr),
    FromDoLoopUntil(Expr, Box<Statement>, Box<Statement>, Expr),
    Push(Expr, Expr),
    Pop(Expr, Expr),
    LocalDelocal(TypedVariable, Expr, Box<Statement>, TypedVariable, Expr),
    Call(ProcId, LinkedList<Expr>),
    Uncall(ProcId, LinkedList<Expr>),
    Skip,
    Sequence(Box<Statement>, Box<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Const(i32),
    Variable(Variable),
    Indexed(Variable, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnrOp(UnrOp, Box<Expr>),
    Empty(Variable),
    Top(Variable),
    Nil,
    Size(Variable),
    Wrapped(Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    _Left,
    Right,
}

#[derive(Debug)]
pub struct BinOpDetail {
    pub op: BinOp,
    pub prec: u8,
    pub direction: Direction,
}

impl BinOpDetail {
    pub const fn new(op: BinOp, prec: u8, direction: Direction) -> Self {
        Self {
            op,
            prec,
            direction,
        }
    }
}
