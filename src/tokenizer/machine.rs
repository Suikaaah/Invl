use crate::tokenizer::{
    detail::{IsSymbol, Token, TokenType},
    CharList, TokenList,
};
use lazy_static::lazy_static;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Machine {
    pub word: CharList,
    pub tokens: TokenList,
}

impl Machine {
    pub fn process_word(mut self) -> Self {
        lazy_static! {
            static ref table: BTreeMap<CharList, Token> = {
                let mut retval = BTreeMap::new();
                let mut push = |x: &str, t: Token| retval.insert(x.chars().collect(), t);
                push("[", Token::LBracket);
                push("]", Token::RBracket);
                push("(", Token::LParen);
                push(")", Token::RParen);
                push("int", Token::Int);
                push("list", Token::List);
                push("array", Token::Array);
                push("invl", Token::Invl);
                push("inj", Token::Inj);
                push("main", Token::Main);
                push("+", Token::Plus);
                push("-", Token::Minus);
                push("^", Token::Caret);
                push("+=", Token::PlusEqual);
                push("-=", Token::MinusEqual);
                push("^=", Token::CaretEqual);
                push("*", Token::Asterisk);
                push("/", Token::Slash);
                push("%", Token::Percent);
                push("&", Token::Ampersand);
                push("|", Token::VerticalBar);
                push("&&", Token::DoubleAmpersand);
                push("||", Token::DoubleVerticalBar);
                push("<", Token::LAngleBracket);
                push(">", Token::RAngleBracket);
                push("=", Token::Equal);
                push("!=", Token::ExclamationEqual);
                push("<=", Token::LAngleBracketEqual);
                push(">=", Token::RAngleBracketEqual);
                push("<=>", Token::Swap);
                push("if", Token::If);
                push("then", Token::Then);
                push("else", Token::Else);
                push("fi", Token::Fi);
                push("from", Token::From);
                push("do", Token::Do);
                push("loop", Token::Loop);
                push("until", Token::Until);
                push("push", Token::Push);
                push("pop", Token::Pop);
                push("local", Token::Local);
                push("delocal", Token::Delocal);
                push("call", Token::Call);
                push("uncall", Token::Uncall);
                push("skip", Token::Skip);
                push("empty", Token::Empty);
                push("top", Token::Top);
                push("!", Token::Exclamation);
                push(",", Token::Comma);
                push("nil", Token::Nil);
                push("size", Token::Size);
                push("print", Token::Print);
                push("with", Token::With);
                push(";", Token::Semicolon);
                retval
            };
        }

        if self.word.is_empty() {
            return self;
        }

        if let Some(token) = table.get(&self.word) {
            return self.push_and_clear(token.clone());
        }

        let token = match self.get_type().expect("empty word") {
            TokenType::Symbol => {
                let mut right = CharList::new();
                loop {
                    if self.word.len() < 2 {
                        panic!("invalid symbol");
                    }

                    right.push_front(self.word.pop_back().expect("unreachable"));

                    if let Some(token) = table.get(&self.word) {
                        self.tokens.push_back(token.clone());
                        self.word = right;
                        return self.process_word();
                    }
                }
            }
            TokenType::Number => {
                let string = self.stringify();
                let parsed = match string.parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => panic!("{} is not a valid int", string),
                };

                Token::Literal(parsed)
            }
            TokenType::Text => Token::Name(self.stringify()),
        };

        self.push_and_clear(token)
    }

    pub fn get_type(&self) -> Option<TokenType> {
        self.word.front().map(|x| {
            if x.is_ascii_digit() {
                TokenType::Number
            } else if x.is_symbol() {
                TokenType::Symbol
            } else {
                TokenType::Text
            }
        })
    }

    pub fn take_tokens(self) -> TokenList {
        self.tokens
    }

    fn push_and_clear(mut self, token: Token) -> Self {
        self.tokens.push_back(token);
        self.word.clear();
        self
    }

    fn stringify(&self) -> String {
        self.word.iter().collect()
    }
}
