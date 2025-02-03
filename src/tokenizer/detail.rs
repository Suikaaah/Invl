use crate::parser::detail::{BinOp, BinOpDetail, Direction};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Name(String),
    Literal(i32),
    LBracket,
    RBracket,
    LParen,
    RParen,
    Int,
    List,
    Array,
    Invl,
    Inj,
    Main,
    Plus,
    Minus,
    Caret,
    PlusEqual,
    MinusEqual,
    CaretEqual,
    Asterisk,
    Slash,
    Percent,
    Ampersand,
    VerticalBar,
    DoubleAmpersand,
    DoubleVerticalBar,
    LAngleBracket,
    RAngleBracket,
    Equal,
    ExclamationEqual,
    LAngleBracketEqual,
    RAngleBracketEqual,
    If,
    Then,
    Else,
    Fi,
    From,
    Do,
    Loop,
    Until,
    Push,
    Pop,
    Local,
    Delocal,
    Call,
    Uncall,
    Skip,
    Empty,
    Top,
    Exclamation,
    Comma,
    Swap,
    Nil,
    Size,
    Print,
    With,
}

impl Token {
    pub fn detail(&self) -> Option<BinOpDetail> {
        let f = |op, prec, dir| BinOpDetail::new(op, prec, dir);

        Some(match self {
            Token::DoubleVerticalBar => f(BinOp::LogicalOr, 0, Direction::Right),
            Token::DoubleAmpersand => f(BinOp::LogicalAnd, 1, Direction::Right),
            Token::VerticalBar => f(BinOp::BitwiseOr, 2, Direction::Right),
            Token::Caret => f(BinOp::Xor, 3, Direction::Right),
            Token::Ampersand => f(BinOp::BitwiseAnd, 4, Direction::Right),
            Token::Equal => f(BinOp::Equal, 5, Direction::Right),
            Token::ExclamationEqual => f(BinOp::NotEqual, 5, Direction::Right),
            Token::LAngleBracket => f(BinOp::LessThan, 6, Direction::Right),
            Token::RAngleBracket => f(BinOp::GreaterThan, 6, Direction::Right),
            Token::LAngleBracketEqual => f(BinOp::LessEqual, 6, Direction::Right),
            Token::RAngleBracketEqual => f(BinOp::GreaterEqual, 6, Direction::Right),
            Token::Plus => f(BinOp::Add, 7, Direction::Right),
            Token::Minus => f(BinOp::Sub, 7, Direction::Right),
            Token::Asterisk => f(BinOp::Mul, 8, Direction::Right),
            Token::Slash => f(BinOp::Div, 8, Direction::Right),
            Token::Percent => f(BinOp::Remainder, 8, Direction::Right),
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum TokenType {
    Number,
    Symbol,
    Text,
}

pub trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        matches!(
            self,
            '[' | ']'
                | '('
                | ')'
                | ','
                | '+'
                | '-'
                | '^'
                | '*'
                | '/'
                | '%'
                | '&'
                | '|'
                | '<'
                | '>'
                | '='
                | '!'
        )
    }
}
