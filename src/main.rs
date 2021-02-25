pub mod parser;

use parser::{gen, parse, sprint_node};

fn main() {
    let prog_string = std::env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let parent_node = parse(prog_string.as_str());
    eprintln!("debug: {}", &sprint_node(&parent_node));

    gen(parent_node);

    println!("    pop rax");
    println!("    ret");
}
