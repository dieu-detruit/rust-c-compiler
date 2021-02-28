pub mod codegen;
pub mod node;
pub mod parser;
pub mod tokenizer;

use codegen::CodeGenerator;
use node::sprint_node;
use parser::parse;

fn main() {
    let prog_string = std::env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let (code, local_var_size) = parse(prog_string);

    println!("    push rbp");
    println!("    mov rbp, rsp");
    println!("    sub rsp, {}", local_var_size);

    let mut generator = CodeGenerator { label_count: 0 };
    eprintln!("debug: {}", &sprint_node(&code));
    generator.gen(&code);
}
