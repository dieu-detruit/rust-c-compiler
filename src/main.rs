pub mod parser;

use parser::{gen, parse, sprint_node};

fn main() {
    let prog_string = std::env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let parent_node = parse(prog_string.as_str());

    gen(parent_node);
    //let output = String::from("debug: ") + &sprint_node(parent_node);
    //println!("{}", output);

    println!("    pop rax");
    println!("    ret");
}
