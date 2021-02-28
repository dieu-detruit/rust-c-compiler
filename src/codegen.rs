use crate::node::{BinaryType, Node};

pub struct CodeGenerator {
    pub label_count: usize,
}

impl CodeGenerator {
    pub fn gen_lval(&mut self, node: &Node) {
        match node {
            Node::LVar(offset) => {
                println!("    mov rax, rbp");
                println!("    sub rax, {}", offset);
                println!("    push rax");
            }
            _ => {
                panic!("Not LVar");
            }
        }
    }

    pub fn gen(&mut self, node: &Node) {
        match node {
            Node::LVar(_offset) => {
                self.gen_lval(node);
                println!("    pop rax");
                println!("    mov rax, [rax]");
                println!("    push rax");
            }
            Node::Assign(assign_args) => {
                self.gen_lval(&assign_args.0);
                self.gen(&assign_args.1);
                println!("    pop rdi");
                println!("    pop rax");
                println!("    mov [rax], rdi");
                println!("    push rdi");
            }
            Node::Return(return_expr) => {
                self.gen(&*return_expr);
                println!("    pop rax");
                println!("    mov rsp, rbp");
                println!("    pop rbp");
                println!("    ret");
            }
            Node::If(if_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.gen(&if_arg.0);
                println!("    pop rax");
                println!("    cmp rax, 0");
                println!("    je .Lend{}", label);
                self.gen(&if_arg.1);
                println!(".Lend{}:", label);
            }
            Node::IfElse(if_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.gen(&if_arg.0);
                println!("    pop rax");
                println!("    cmp rax, 0");
                println!("    je .Lelse{}", label);
                self.gen(&if_arg.1);
                println!("    jmp .Lend{}", label);
                println!(".Lelse{}:", label);
                self.gen(&if_arg.2);
                println!(".Lend{}:", label);
            }
            Node::While(while_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                println!(".Lbegin{}:", label);
                self.gen(&while_arg.0);
                println!("    pop rax");
                println!("    cmp rax, 0");
                println!("    je .Lend{}", label);
                self.gen(&while_arg.1);
                println!("    jmp .Lbegin{}", label);
                println!(".Lend{}:", label);
            }
            Node::For(for_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.gen(&for_arg.0); // initialization
                println!(".Lbegin{}:", label);
                self.gen(&for_arg.1); // loop condition
                println!("    pop rax");
                println!("    cmp rax, 0");
                println!("    je .Lend{}", label);
                self.gen(&for_arg.3); // loop statement
                self.gen(&for_arg.2); // var update
                println!("    jmp .Lbegin{}", label);
                println!(".Lend{}:", label);
            }
            Node::Unary(unary_arg, _unary_type) => {
                self.gen(&unary_arg);
                match _unary_type {
                    _ => {}
                }
            }
            Node::Binary(binary_arg, binary_type) => {
                self.gen(&binary_arg.0);
                self.gen(&binary_arg.1);
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
            Node::Boolean(flag) => {
                if *flag {
                    println!("    push 1")
                } else {
                    println!("    push 0")
                }
            }
            Node::Empty => {}
        }
    }
}
