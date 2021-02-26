pub mod codegen;
pub mod node;
pub mod parser;
pub mod tokenizer;

use codegen::gen;
use node::sprint_node;
use parser::parse;

fn main() {
    let prog_string = std::env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let code = parse(prog_string.as_str());
    for statement in code.iter() {
        eprintln!("debug: {}", &sprint_node(&statement));
        gen(statement);
    }

    println!("    pop rax");
    println!("    ret");
}
