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
                let f = |x: &str| x.chars().collect();
                let mut retval = BTreeMap::new();
                retval.insert(f("["), Token::LBracket);
                retval.insert(f("]"), Token::RBracket);
                retval.insert(f("("), Token::LParen);
                retval.insert(f(")"), Token::RParen);
                retval.insert(f("int"), Token::Int);
                retval.insert(f("list"), Token::List);
                retval.insert(f("array"), Token::Array);
                retval.insert(f("procedure"), Token::Procedure);
                retval.insert(f("main"), Token::Main);
                retval.insert(f("+"), Token::Plus);
                retval.insert(f("-"), Token::Minus);
                retval.insert(f("^"), Token::Caret);
                retval.insert(f("+="), Token::PlusEqual);
                retval.insert(f("-="), Token::MinusEqual);
                retval.insert(f("^="), Token::CaretEqual);
                retval.insert(f("*"), Token::Asterisk);
                retval.insert(f("/"), Token::Slash);
                retval.insert(f("%"), Token::Percent);
                retval.insert(f("&"), Token::Ampersand);
                retval.insert(f("|"), Token::VerticalBar);
                retval.insert(f("&&"), Token::DoubleAmpersand);
                retval.insert(f("||"), Token::DoubleVerticalBar);
                retval.insert(f("<"), Token::LAngleBracket);
                retval.insert(f(">"), Token::RAngleBracket);
                retval.insert(f("="), Token::Equal);
                retval.insert(f("!="), Token::ExclamationEqual);
                retval.insert(f("<="), Token::LAngleBracketEqual);
                retval.insert(f(">="), Token::RAngleBracketEqual);
                retval.insert(f("<=>"), Token::Swap);
                retval.insert(f("if"), Token::If);
                retval.insert(f("then"), Token::Then);
                retval.insert(f("else"), Token::Else);
                retval.insert(f("fi"), Token::Fi);
                retval.insert(f("from"), Token::From);
                retval.insert(f("do"), Token::Do);
                retval.insert(f("loop"), Token::Loop);
                retval.insert(f("until"), Token::Until);
                retval.insert(f("push"), Token::Push);
                retval.insert(f("pop"), Token::Pop);
                retval.insert(f("local"), Token::Local);
                retval.insert(f("delocal"), Token::Delocal);
                retval.insert(f("call"), Token::Call);
                retval.insert(f("uncall"), Token::Uncall);
                retval.insert(f("skip"), Token::Skip);
                retval.insert(f("empty"), Token::Empty);
                retval.insert(f("top"), Token::Top);
                retval.insert(f("!"), Token::Exclamation);
                retval.insert(f(","), Token::Comma);
                retval.insert(f("nil"), Token::Nil);
                retval.insert(f("size"), Token::Size);
                retval.insert(f("print"), Token::Print);
                retval
            };
        }

        if self.word.is_empty() {
            return self;
        }

        if let Some(token) = table.get(&self.word) {
            return self.push_and_clear(token.clone());
        }

        let token = match self.get_type().expect("word should not be empty") {
            TokenType::Symbol => {
                let mut r = CharList::new();
                loop {
                    if self.word.len() < 2 {
                        panic!("invalid symbol");
                    }

                    r.push_front(self.word.pop_back().expect("unreachable"));

                    if let Some(token) = table.get(&self.word) {
                        self.tokens.push_back(token.clone());
                        self.word = r;
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
