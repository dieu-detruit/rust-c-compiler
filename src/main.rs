pub mod codegen;
pub mod node;
pub mod parser;
pub mod register;
pub mod token;
pub mod tokenizer;
pub mod typename;

use codegen::CodeGenerator;
use node::sprint_node;
use parser::parse::parse;
use tokenizer::{sprint_token_iter, tokenize};

fn main() {
    let prog_string = std::env::args().nth(1).unwrap();

    // Tokenize
    let token_iter = tokenize(prog_string);
    let output = sprint_token_iter(token_iter.clone());
    eprintln!("tokenize result: {}", output);

    // Parse
    let code = parse(token_iter);

    // Code Generation
    let mut generator = CodeGenerator {
        lines: Vec::new(),
        label_count: 0,
        rsp_sub_size: 0,
        label_func: 0,
    };
    eprintln!("parse result: {}", &sprint_node(&code));

    println!(".intel_syntax noprefix");
    println!(".globl main");
    generator.gen(&code);
    for line in generator.lines {
        println!("{}", line);
    }
}
