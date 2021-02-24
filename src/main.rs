pub mod parser;

use parser::{gen, parse};

fn main() {
    let prog_string = std::env::args().nth(1).unwrap();

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    let parent_node = parse(prog_string.as_str());
    gen(parent_node);

    println!("  ret");
}
