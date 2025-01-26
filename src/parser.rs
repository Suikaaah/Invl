pub mod detail;

use crate::{
    parser::detail::{
        Expr, MainProc, MutOp, Proc, ProcId, Program, Statement, Type, TypedVariable, UnrOp,
        Variable,
    },
    tokenizer::{detail::Token, TokenList},
};
use detail::Direction;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Parser {
    tokens: TokenList,
}

impl Parser {
    pub const fn new(tokens: TokenList) -> Self {
        Self { tokens }
    }

    pub fn parse_program(&mut self) -> Program {
        let p_main = self.parse_main_proc();

        let mut procs = LinkedList::new();
        while !self.tokens.is_empty() {
            procs.push_back(self.parse_proc());
        }

        Program(p_main, procs)
    }

    fn parse_type(&mut self) -> Type {
        match self.pop_front() {
            Token::Int => Type::Int,
            Token::List => Type::List,
            Token::Array => {
                self.pop_assert(Token::LBracket);
                let c = self.parse_literal();
                self.pop_assert(Token::RBracket);
                Type::Array(c as usize)
            }
            x => panic!("expected type, found {x:?}"),
        }
    }

    fn parse_literal(&mut self) -> i32 {
        match self.pop_front() {
            Token::Literal(x) => x,
            Token::Minus => match self.pop_front() {
                Token::Literal(x) => -x,
                x => panic!("expected literal, found {x:?}"),
            },
            x => panic!("expected literal, found {x:?}"),
        }
    }

    fn parse_variable(&mut self) -> Variable {
        match self.pop_front() {
            Token::Name(x) => Variable(x),
            x => panic!("expected variable, found {x:?}"),
        }
    }

    fn parse_proc_id(&mut self) -> ProcId {
        match self.pop_front() {
            Token::Name(x) => ProcId(x),
            x => panic!("expected proc id, found {x:?}"),
        }
    }

    fn parse_typed_variable(&mut self) -> TypedVariable {
        TypedVariable(self.parse_type(), self.parse_variable())
    }

    fn parse_main_proc(&mut self) -> MainProc {
        self.pop_assert(Token::Procedure);
        self.pop_assert(Token::Main);
        self.pop_assert(Token::LParen);
        self.pop_assert(Token::RParen);

        let mut list = LinkedList::new();

        while matches!(self.seek_front(), Token::Int | Token::List | Token::Array) {
            list.push_back(self.parse_typed_variable());
        }

        let s = self.parse_statement();
        MainProc(list, s)
    }

    fn parse_proc(&mut self) -> Proc {
        self.pop_assert(Token::Procedure);
        let q = self.parse_proc_id();
        self.pop_assert(Token::LParen);

        let mut args = LinkedList::new();

        if !matches!(self.seek_front(), Token::RParen) {
            loop {
                args.push_back(self.parse_typed_variable());
                match self.seek_front() {
                    Token::Comma => {
                        self.pop_front();
                    }
                    _ => break,
                }
            }
        }

        self.pop_assert(Token::RParen);
        let s = self.parse_statement();
        Proc(q, args, s)
    }

    fn parse_statement(&mut self) -> Statement {
        let first = match self.pop_front() {
            Token::Name(x) => match self.seek_front() {
                Token::LBracket => {
                    self.pop_front();
                    let e_l = self.parse_expr(0);
                    self.pop_assert(Token::RBracket);
                    let op = self.parse_mut_op();
                    let e_r = self.parse_expr(0);
                    Statement::IndexedMut(Variable(x), e_l, op, e_r)
                }
                _ => {
                    let op = self.parse_mut_op();
                    let e_r = self.parse_expr(0);
                    Statement::Mut(Variable(x), op, e_r)
                }
            },
            Token::If => {
                let e_l = self.parse_expr(0);
                self.pop_assert(Token::Then);
                let s_l = self.parse_statement();
                self.pop_assert(Token::Else);
                let s_r = self.parse_statement();
                self.pop_assert(Token::Fi);
                let e_r = self.parse_expr(0);
                Statement::IfThenElseFi(e_l, Box::new(s_l), Box::new(s_r), e_r)
            }
            Token::From => {
                let e_l = self.parse_expr(0);
                self.pop_assert(Token::Do);
                let s_l = self.parse_statement();
                self.pop_assert(Token::Loop);
                let s_r = self.parse_statement();
                self.pop_assert(Token::Until);
                let e_r = self.parse_expr(0);
                Statement::FromDoLoopUntil(e_l, Box::new(s_l), Box::new(s_r), e_r)
            }
            token @ (Token::Push | Token::Pop) => {
                self.pop_assert(Token::LParen);
                let e_l = self.parse_expr(0);
                self.pop_assert(Token::Comma);
                let e_r = self.parse_expr(0);
                self.pop_assert(Token::RParen);

                match token {
                    Token::Push => Statement::Push(e_l, e_r),
                    Token::Pop => Statement::Pop(e_l, e_r),
                    _ => unreachable!(),
                }
            }
            Token::Local => {
                let tx_l = self.parse_typed_variable();
                self.pop_assert(Token::Equal);
                let e_l = self.parse_expr(0);
                let s = self.parse_statement();
                self.pop_assert(Token::Delocal);
                let tx_r = self.parse_typed_variable();
                self.pop_assert(Token::Equal);
                let e_r = self.parse_expr(0);
                Statement::LocalDelocal(tx_l, e_l, Box::new(s), tx_r, e_r)
            }
            token @ (Token::Call | Token::Uncall) => {
                let q = self.parse_proc_id();
                self.pop_assert(Token::LParen);

                let mut args = LinkedList::new();

                if !matches!(self.seek_front(), Token::RParen) {
                    loop {
                        args.push_back(self.parse_expr(0));
                        match self.seek_front() {
                            Token::Comma => {
                                self.pop_front();
                            }
                            _ => break,
                        }
                    }
                }

                self.pop_assert(Token::RParen);

                match token {
                    Token::Call => Statement::Call(q, args),
                    Token::Uncall => Statement::Uncall(q, args),
                    _ => unreachable!(),
                }
            }
            Token::Skip => Statement::Skip,
            x => panic!("expected statement, found {x:?}"),
        };

        match self.tokens.front() {
            Some(
                Token::Name(_)
                | Token::If
                | Token::From
                | Token::Push
                | Token::Pop
                | Token::Local
                | Token::Call
                | Token::Uncall
                | Token::Skip,
            ) => Statement::Sequence(Box::new(first), Box::new(self.parse_statement())),
            _ => first,
        }
    }

    fn parse_expr(&mut self, min_prec: u8) -> Expr {
        let mut first = match self.pop_front() {
            Token::Literal(x) => Expr::Const(x),
            Token::Nil => Expr::Nil,
            Token::LParen => {
                let e = self.parse_expr(0);
                self.pop_assert(Token::RParen);
                Expr::Wrapped(Box::new(e))
            }
            token @ (Token::Empty | Token::Top | Token::Size) => {
                self.pop_assert(Token::LParen);
                let x = self.parse_variable();
                self.pop_assert(Token::RParen);

                match token {
                    Token::Empty => Expr::Empty(x),
                    Token::Top => Expr::Top(x),
                    Token::Size => Expr::Size(x),
                    _ => unreachable!(),
                }
            }
            Token::Name(x) => {
                if matches!(self.tokens.front(), Some(Token::LBracket)) {
                    self.pop_front();
                    let e = self.parse_expr(0);
                    self.pop_assert(Token::RBracket);
                    Expr::Indexed(Variable(x), Box::new(e))
                } else {
                    Expr::Variable(Variable(x))
                }
            }
            token @ (Token::Exclamation | Token::Minus) => {
                let e = self.parse_expr(255);
                let op = match token {
                    Token::Exclamation => UnrOp::Not,
                    Token::Minus => UnrOp::Negative,
                    _ => unreachable!(),
                };

                Expr::UnrOp(op, Box::new(e))
            }
            x => panic!("invalid token: {x:?} {self:?}"),
        };

        while let Some(front) = self.tokens.front() {
            let op_detail = match front.detail() {
                Some(x) if x.prec >= min_prec => x,
                _ => break,
            };

            self.pop_front();

            let next_min_prec = match op_detail.direction {
                Direction::_Left => op_detail.prec + 1,
                Direction::Right => op_detail.prec,
            };

            let second = self.parse_expr(next_min_prec);

            first = Expr::BinOp(Box::new(first), op_detail.op, Box::new(second));
        }

        first
    }

    fn parse_mut_op(&mut self) -> MutOp {
        match self.pop_front() {
            Token::PlusEqual => MutOp::Add,
            Token::MinusEqual => MutOp::Sub,
            Token::CaretEqual => MutOp::Xor,
            Token::Swap => MutOp::Swap,
            x => panic!("expected mut op, found {x:?}"),
        }
    }

    fn pop_assert(&mut self, token: Token) -> Token {
        match self.pop_front() {
            x if x == token => token,
            x => panic!("{x:?} != {token:?}"),
        }
    }

    fn pop_front(&mut self) -> Token {
        self.tokens.pop_front().expect("no token left")
    }

    fn seek_front(&self) -> &Token {
        self.tokens.front().expect("no token left")
    }
}
