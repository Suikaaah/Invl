mod checker;
mod cvt;
mod parser;
mod tokenizer;

use checker::Checker;
use cvt::Cvt;
use parser::Parser;
use std::fs;
use tokenizer::Tokenizer;

fn main() {
    let source = fs::read_to_string("main.invl").expect("come on");
    let tokens = Tokenizer::tokenize(&source);
    let program = Parser::new(tokens).parse_program();
    Checker::check(&program);
    fs::write("main.cpp", program.cvt()).expect("come on");
}
