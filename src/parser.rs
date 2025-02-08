pub mod detail;
pub mod mat;

use crate::{
    parser::detail::{
        Expr, MainProc, MutOp, Proc, ProcId, Program, Statement, Type, TypedVariable, UnrOp,
        Variable,
    },
    tokenizer::{detail::Token, TokenList},
};
use detail::Direction;
use mat::InvlMat;
use std::{collections::LinkedList, rc::Rc};

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
            Token::Name(x) => Variable::new(x),
            x => panic!("expected variable, found {x:?}"),
        }
    }

    fn parse_proc_id(&mut self) -> ProcId {
        match self.pop_front() {
            Token::Name(x) => ProcId::new(x),
            x => panic!("expected proc id, found {x:?}"),
        }
    }

    fn parse_typed_variable(&mut self) -> TypedVariable {
        TypedVariable(self.parse_type(), self.parse_variable())
    }

    fn parse_main_proc(&mut self) -> MainProc {
        self.pop_assert(Token::Invl);
        self.pop_assert(Token::Main);
        self.pop_assert(Token::LParen);
        self.pop_assert(Token::RParen);

        let mut list = LinkedList::new();

        while matches!(self.seek_front(), Token::Int | Token::List | Token::Array) {
            let t_x = self.parse_typed_variable();
            if let Token::Equal = self.seek_front() {
                self.pop_front();
                list.push_back((t_x, Some(self.parse_expr(0))));
            } else {
                list.push_back((t_x, None));
            }
        }

        let s = if matches!(self.seek_front(), Token::With) {
            Statement::Skip
        } else {
            self.parse_statement()
        };
        self.pop_assert(Token::With);
        let i = self.parse_invl();
        MainProc(list, s, i)
    }

    fn parse_mat(&mut self) -> InvlMat {
        let mut numbers = Vec::new();

        if !matches!(self.seek_front(), Token::RBracket) {
            loop {
                match self.pop_front() {
                    Token::Literal(x) => numbers.push(x),
                    Token::Minus => match self.pop_front() {
                        Token::Literal(x) => numbers.push(-x),
                        x => panic!("expected literal, found {x:?}"),
                    },
                    x => panic!("expected literal or minus, found {x:?}"),
                }

                match self.seek_front() {
                    Token::Semicolon => {
                        self.pop_front();
                    }
                    Token::RBracket => break,
                    _ => {}
                }
            }
        }

        self.pop_assert(Token::RBracket);
        InvlMat::new(numbers).expect("invalid invl_mat")
    }

    fn parse_proc(&mut self) -> Proc {
        let either = match self.pop_front() {
            token @ (Token::Invl | Token::Inj) => token,
            x => panic!("expected invl or inj, found {x:?}"),
        };

        let q = self.parse_proc_id();

        match self.pop_front() {
            Token::LParen => {
                let mut args = LinkedList::new();

                if !matches!(self.seek_front(), Token::RParen) {
                    loop {
                        args.push_back(self.parse_typed_variable());
                        match self.seek_front() {
                            Token::Comma => {
                                self.pop_front();
                            }
                            Token::RParen => break,
                            x => panic!("unexpected token: {x:?}"),
                        }
                    }
                }

                self.pop_assert(Token::RParen);
                let s = self.parse_statement();

                match either {
                    Token::Inj => Proc::Inj(q, args, s),
                    Token::Invl => {
                        self.pop_assert(Token::With);
                        let i = self.parse_invl();
                        Proc::Invl(q, args, s, i)
                    }
                    _ => unreachable!(),
                }
            }
            Token::LBracket => Proc::Mat(q, self.parse_mat()),
            x => panic!("expected proc, found {x:?}"),
        }
    }

    fn parse_invl(&mut self) -> Statement {
        fn check(statement: Statement) -> Statement {
            match statement {
                s @ (Statement::Mut(_, MutOp::Xor | MutOp::Swap, _)
                | Statement::IndexedMut(_, _, MutOp::Xor | MutOp::Swap, _)
                | Statement::Call(_, _)
                | Statement::Uncall(_, _)
                | Statement::Skip
                | Statement::Print(_)) => s,
                Statement::Sequence(l, r) => {
                    Statement::Sequence(Box::new(check(*l)), Box::new(check(*r)))
                }
                x => panic!("expected invl, found {x:?}"),
            }
        }

        check(self.parse_statement())
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
                    Statement::IndexedMut(Variable::new(x), e_l, op, e_r)
                }
                _ => {
                    let op = self.parse_mut_op();
                    let e_r = self.parse_expr(0);
                    Statement::Mut(Variable::new(x), op, e_r)
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
                let e = self.parse_expr(0);
                self.pop_assert(Token::Comma);
                let x = self.parse_variable();
                self.pop_assert(Token::RParen);

                match token {
                    Token::Push => Statement::Push(e, x),
                    Token::Pop => Statement::Pop(e, x),
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

                        if let Token::Comma = self.seek_front() {
                            self.pop_front();
                        } else {
                            break;
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
            Token::Print => {
                self.pop_assert(Token::LParen);
                let x = self.parse_variable();
                self.pop_assert(Token::RParen);
                Statement::Print(x)
            }
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
                | Token::Skip
                | Token::Print,
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
                Expr::Wrapped(Rc::new(e))
            }
            Token::LBracket => {
                let mut items = LinkedList::new();

                if let Token::RBracket = self.seek_front() {
                    self.pop_front();
                    return Expr::Array(Rc::new(items));
                }

                loop {
                    items.push_back(self.parse_expr(0));
                    match self.seek_front() {
                        Token::Comma => {
                            self.pop_front();
                        }
                        Token::RBracket => {
                            self.pop_front();
                            break;
                        }
                        x => panic!("unexpected token found in array: {x:?}"),
                    }
                }

                Expr::Array(Rc::new(items))
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
                    Expr::Indexed(Variable::new(x), Rc::new(e))
                } else {
                    Expr::Variable(Variable::new(x))
                }
            }
            token @ (Token::Exclamation | Token::Minus) => {
                let e = self.parse_expr(255);
                let op = match token {
                    Token::Exclamation => UnrOp::Not,
                    Token::Minus => UnrOp::Negative,
                    _ => unreachable!(),
                };

                Expr::UnrOp(op, Rc::new(e))
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

            first = Expr::BinOp(Rc::new(first), op_detail.op, Rc::new(second));
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
