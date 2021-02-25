use crate::node::{BinaryType, Node, UnaryType};

pub fn gen(node: Node) {
    match node {
        Node::Unary(unary_arg, _unary_type) => {
            gen(*unary_arg);
            match _unary_type {
                _ => {}
            }
        }
        Node::Binary(binary_arg, binary_type) => {
            gen(binary_arg.0);
            gen(binary_arg.1);
            println!("    pop rdi");
            println!("    pop rax");
            match binary_type {
                BinaryType::Add => {
                    println!("    add rax, rdi");
                }
                BinaryType::Sub => {
                    println!("    sub rax, rdi");
                }
                BinaryType::Mul => {
                    println!("    imul rax, rdi");
                }
                BinaryType::Div => {
                    println!("    cqo");
                    println!("    idiv rdi");
                }
                BinaryType::Equal => {
                    println!("    cmp rax, rdi");
                    println!("    sete al");
                    println!("    movzb rax, al");
                }
                BinaryType::NotEqual => {
                    println!("    cmp rax, rdi");
                    println!("    setne al");
                    println!("    movzb rax, al");
                }
                BinaryType::Lt => {
                    println!("    cmp rax, rdi");
                    println!("    setl al");
                    println!("    movzb rax, al");
                }
                BinaryType::LtEq => {
                    println!("    cmp rax, rdi");
                    println!("    setle al");
                    println!("    movzb rax, al");
                }
            }
            println!("    push rax");
        }
        Node::Num(n) => println!("    push {}", n),
        _ => {}
    }
}
