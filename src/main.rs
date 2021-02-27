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

    let (code, local_var_size) = parse(prog_string.as_str());

    println!("    push rbp");
    println!("    mov rbp, rsp");
    println!("    sub rsp, {}", local_var_size);

    for statement in code.iter() {
        eprintln!("debug: {}", &sprint_node(&statement));
        gen(statement);
        println!("    pop rax");
    }

    println!("    mov rsp, rbp");
    println!("    pop rbp");
    println!("    ret");
}
