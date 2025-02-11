use crate::parser::mat::InvlMat;
use std::{collections::LinkedList, rc::Rc};

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
pub struct Variable(pub Rc<String>);

impl Variable {
    pub fn new(name: String) -> Self {
        Self(Rc::new(name))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcId(pub Rc<String>);

impl ProcId {
    pub fn new(name: String) -> Self {
        Self(Rc::new(name))
    }
}

#[derive(Debug)]
pub struct Program(pub MainProc, pub LinkedList<Proc>);

#[derive(Debug, Clone)]
pub struct TypedVariable(pub Type, pub Variable);

#[derive(Debug)]
pub struct MainProc(
    pub LinkedList<(TypedVariable, Option<Expr>)>,
    pub Statement,
    pub Statement,
);

#[derive(Debug)]
pub enum Proc {
    Inj(ProcId, LinkedList<TypedVariable>, Statement),
    Invl(ProcId, LinkedList<TypedVariable>, Statement, Statement),
    Mat(ProcId, InvlMat),
}

#[derive(Debug)]
pub enum Statement {
    Mut(Variable, MutOp, Expr),
    IndexedMut(Variable, Expr, MutOp, Expr),
    IfThenElseFi(Expr, Box<Statement>, Box<Statement>, Expr),
    FromDoLoopUntil(Expr, Box<Statement>, Box<Statement>, Expr),
    Push(Expr, Variable),
    Pop(Expr, Variable),
    LocalDelocal(TypedVariable, Expr, Box<Statement>, TypedVariable, Expr),
    Call(ProcId, LinkedList<Expr>),
    Uncall(ProcId, LinkedList<Expr>),
    Skip,
    Print(Variable),
    For(Variable, Expr, Box<Statement>),
    IfThenElse(Expr, Box<Statement>, Box<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Const(i32),
    Variable(Variable),
    Array(Rc<LinkedList<Expr>>),
    Indexed(Variable, Rc<Expr>),
    BinOp(Rc<Expr>, BinOp, Rc<Expr>),
    UnrOp(UnrOp, Rc<Expr>),
    Empty(Variable),
    Top(Variable),
    Nil,
    Size(Variable),
    Wrapped(Rc<Expr>),
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
