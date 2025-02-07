mod checker;
mod cvt;
mod parser;
mod tokenizer;

use checker::Checker;
use cvt::Cvt;
use parser::Parser;
use std::{
    fs,
    process::{Command, Output},
};
use tokenizer::Tokenizer;

fn main() {
    let source = fs::read_to_string("main.invl").expect("come on");
    let tokens = Tokenizer::tokenize(&source);
    let program = Parser::new(tokens).parse_program();
    Checker::check(&program);
    fs::write("main.cpp", program.cvt()).expect("come on");

    let Output { stdout, stderr, .. } = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg(include_str!("../command_windows"))
            .output()
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg(include_str!("../command_linux"))
            .output()
    } else {
        unimplemented!("mac?")
    }
    .expect("come on");

    let convert = |v| String::from_utf8(v).expect("invalid utf8");
    let stdout = convert(stdout);
    let stderr = convert(stderr);

    print!("STDERR:\n{stderr}");
    print!("STDOUT:\n{stdout}");
}
