pub mod detail;
mod machine;

use detail::{IsSymbol, Token, TokenType};
use machine::Machine;
use std::collections::LinkedList;

pub type TokenList = LinkedList<Token>;
pub type CharList = LinkedList<char>;

#[derive(Debug)]
pub struct Tokenizer {
    source: CharList,
}

impl Tokenizer {
    pub fn tokenize(input: &str) -> TokenList {
        let comments_excluded: String = input
            .split("//")
            .enumerate()
            .map(|(i, x)| {
                if i == 0 {
                    x.to_string()
                } else {
                    x.lines().skip(1).collect::<String>()
                }
            })
            .collect();

        Tokenizer::new(&comments_excluded)
            .tokenize_impl(Machine::default())
            .take_tokens()
    }

    fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
        }
    }

    fn tokenize_impl(&mut self, mut machine: Machine) -> Machine {
        let first = match self.source.pop_front() {
            None => return machine.process_word(),
            Some(x) if x.is_whitespace() => return self.tokenize_impl(machine.process_word()),
            Some(x) => x,
        };

        let push_token = match machine.get_type() {
            Some(TokenType::Number) => !first.is_ascii_digit(),
            Some(TokenType::Symbol) => !first.is_symbol(),
            Some(TokenType::Text) => first.is_symbol(),
            None => false,
        };

        if push_token {
            let mut machine = machine.process_word();
            machine.word.push_back(first);
            self.tokenize_impl(machine)
        } else {
            machine.word.push_back(first);
            self.tokenize_impl(machine)
        }
    }
}
