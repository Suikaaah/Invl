pub mod detail;
pub mod r#for;
pub mod mat;

use crate::{
    parser::detail::{
        Expr, MainProc, MutOp, Proc, ProcId, Program, Statement, Type, TypedVariable, UnrOp,
        Variable,
    },
    tokenizer::{TokenList, detail::Token},
};
use detail::{Direction, InnerType, VariableOrLiteral};
use r#for::For;
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

    fn parse_inner_type(&mut self) -> InnerType {
        match self.pop_front() {
            Token::Int => InnerType::Int,
            Token::List => InnerType::List,
            Token::Array => {
                self.pop_assert(Token::LAngleBracket);
                let c = self.parse_literal();
                self.pop_assert(Token::RAngleBracket);
                InnerType::Array(c as usize)
            }
            x => panic!("expected inner type, found {x:?}"),
        }
    }

    fn parse_type(&mut self) -> Type {
        if let Token::Const = self.seek_front() {
            self.pop_front();
            Type {
                r#const: true,
                inner: self.parse_inner_type(),
            }
        } else {
            Type {
                r#const: false,
                inner: self.parse_inner_type(),
            }
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

        while matches!(
            self.seek_front(),
            Token::Const | Token::Int | Token::List | Token::Array
        ) {
            let t_x = self.parse_typed_variable();
            if let Token::Equal = self.seek_front() {
                self.pop_front();
                list.push_back((t_x, Some(self.parse_expr(0))));
            } else {
                list.push_back((t_x, None));
            }
        }

        let s = if let Token::With = self.seek_front() {
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
            x => panic!("expected invl or inj, found {x:?} {self:?}"),
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

                match either {
                    Token::Inj => Proc::Inj(q, args, self.parse_statement()),
                    Token::Invl => {
                        let s = if let Token::With = self.seek_front() {
                            Statement::Skip
                        } else {
                            self.parse_statement()
                        };
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
                | Statement::IndexedSwap(_, _, _)
                | Statement::Call(_, _)
                | Statement::Uncall(_, _)
                | Statement::Skip
                | Statement::Print(_)
                | Statement::IfThenElse(_, _, _)
                | Statement::For(_)) => s,
                Statement::Sequence(l, r) => {
                    Statement::Sequence(Box::new(check(*l)), Box::new(check(*r)))
                }
                x => panic!("expected invl, found {x:?}"),
            }
        }

        check(self.parse_statement())
    }

    fn parse_var_pack(&mut self) -> LinkedList<Variable> {
        let mut l = LinkedList::new();
        if let Token::LBracket = self.seek_front() {
            self.pop_front();
            loop {
                l.push_back(self.parse_variable());
                match self.pop_front() {
                    Token::Comma => {}
                    Token::RBracket => break,
                    x => panic!("unexpected token found in var pack: {x:?}"),
                }
            }
        } else {
            l.push_back(self.parse_variable());
        }

        l
    }

    fn parse_maybe_indexed(&mut self) -> (Variable, Option<Variable>) {
        let x = self.parse_variable();
        let i = if let Token::LBracket = self.seek_front() {
            self.pop_front();
            let i = self.parse_variable();
            self.pop_assert(Token::RBracket);
            Some(i)
        } else {
            None
        };
        (x, i)
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

                match self.pop_front() {
                    Token::End => {
                        Statement::IfThenElse(e_l, Box::new(s_l), Box::new(Statement::Skip))
                    }
                    Token::Fi => {
                        let e_r = self.parse_expr(0);
                        Statement::IfThenElseFi(e_l, Box::new(s_l), Box::new(Statement::Skip), e_r)
                    }
                    Token::Else => {
                        let s_r = self.parse_statement();

                        match self.pop_front() {
                            Token::Fi => {
                                let e_r = self.parse_expr(0);
                                Statement::IfThenElseFi(e_l, Box::new(s_l), Box::new(s_r), e_r)
                            }
                            Token::End => Statement::IfThenElse(e_l, Box::new(s_l), Box::new(s_r)),
                            x => panic!("expected fi or end, found {x:?}"),
                        }
                    }
                    x => panic!("expected end, fi or else, bound {x:?}"),
                }
            }
            Token::From => {
                let e_l = self.parse_expr(0);
                let s_l;
                match self.pop_front() {
                    Token::Do => {
                        s_l = self.parse_statement();
                        self.pop_assert(Token::Loop);
                    }
                    Token::Loop => {
                        s_l = Statement::Skip;
                    }
                    x => panic!("expected do or loop, found {x:?}"),
                }
                let s_r = self.parse_statement();
                self.pop_assert(Token::Until);
                let e_r = self.parse_expr(0);
                Statement::FromDoLoopUntil(e_l, Box::new(s_l), Box::new(s_r), e_r)
            }
            token @ (Token::PushFront | Token::PushBack | Token::PopFront | Token::PopBack) => {
                self.pop_assert(Token::LParen);

                let l = if let Token::Literal(n) = self.seek_front() {
                    let n = *n;
                    self.pop_front();
                    VariableOrLiteral::Literal(n)
                } else {
                    VariableOrLiteral::Variable(self.parse_variable())
                };

                self.pop_assert(Token::Comma);
                let r = self.parse_variable();
                self.pop_assert(Token::RParen);

                match token {
                    Token::PushFront => Statement::PushFront(l, r),
                    Token::PushBack => Statement::PushBack(l, r),
                    Token::PopFront => Statement::PopFront(l, r),
                    Token::PopBack => Statement::PopBack(l, r),
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
                assert_eq!(tx_l, tx_r);
                Statement::LocalDelocal(tx_l, e_l, Box::new(s), tx_r, e_r)
            }
            token @ (Token::Call | Token::Uncall) => {
                let q = self.parse_proc_id();
                self.pop_assert(Token::LParen);

                let mut args = LinkedList::new();

                if !matches!(self.seek_front(), Token::RParen) {
                    loop {
                        args.push_back(self.parse_variable());

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
            Token::For => {
                let mut vars = LinkedList::new();
                let mut containers = LinkedList::new();
                if let Token::LParen = self.seek_front() {
                    self.pop_front();
                    loop {
                        vars.push_back(self.parse_var_pack());
                        match self.pop_front() {
                            Token::Comma => {}
                            Token::RParen => break,
                            x => panic!("unexpected token found in param list: {x:?}"),
                        }
                    }
                } else {
                    vars.push_back(self.parse_var_pack());
                }

                self.pop_assert(Token::In);

                if let Token::LParen = self.seek_front() {
                    self.pop_front();
                    loop {
                        containers.push_back(self.parse_maybe_indexed());
                        match self.pop_front() {
                            Token::Comma => {}
                            Token::RParen => break,
                            x => panic!("unexpected token found in param list: {x:?}"),
                        }
                    }
                } else {
                    containers.push_back(self.parse_maybe_indexed());
                }

                assert_eq!(vars.len(), containers.len(), "unmatched params");

                let s = self.parse_statement();
                self.pop_assert(Token::End);

                Statement::For(For {
                    vars,
                    containers,
                    statement: Box::new(s),
                })
            }
            Token::Swap => {
                self.pop_assert(Token::LParen);
                let x = self.parse_variable();
                self.pop_assert(Token::Comma);
                let l = self.parse_expr(0);
                self.pop_assert(Token::Comma);
                let r = self.parse_expr(0);
                self.pop_assert(Token::RParen);

                Statement::IndexedSwap(x, l, r)
            }
            x => panic!("expected statement, found {x:?}"),
        };

        match self.tokens.front() {
            Some(
                Token::Name(_)
                | Token::If
                | Token::From
                | Token::PushFront
                | Token::PushBack
                | Token::PopFront
                | Token::PopBack
                | Token::Local
                | Token::Call
                | Token::Uncall
                | Token::Skip
                | Token::Print
                | Token::For
                | Token::Swap,
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
                    Expr::Array(Rc::new(items))
                } else {
                    loop {
                        items.push_back(self.parse_expr(0));
                        match self.pop_front() {
                            Token::Comma => {}
                            Token::RBracket => break,
                            x => panic!("unexpected token found in array: {x:?}"),
                        }
                    }

                    Expr::Array(Rc::new(items))
                }
            }
            token @ (Token::Empty | Token::Size) => {
                self.pop_assert(Token::LParen);
                let x = self.parse_variable();
                self.pop_assert(Token::RParen);

                match token {
                    Token::Empty => Expr::Empty(x),
                    Token::Size => Expr::Size(x),
                    _ => unreachable!(),
                }
            }
            Token::Name(x) => {
                if let Some(Token::LBracket) = self.tokens.front() {
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
            Token::Spaceship => MutOp::Swap,
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
