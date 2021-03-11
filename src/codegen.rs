use crate::node::{BinaryType, Node};

pub struct CodeGenerator {
    pub lines: Vec<String>,
    pub label_count: usize,
    pub max_stack_size: usize,
    pub current_stack_size: usize,
}

impl CodeGenerator {
    pub fn gen_lval(&mut self, node: &Node) {
        match node {
            Node::LVar(offset) => {
                self.lines.push(format!("    mov rax, rbp"));
                self.lines.push(format!("    sub rax, {}", offset));
                self.lines.push(format!("    push rax"));
            }
            _ => {
                panic!("Not LVar");
            }
        }
    }

    fn pop(&mut self) {
        self.current_stack_size -= self.current_stack_size;
    }

    fn push() {}

    // -> bool : require pop stack
    pub fn gen(&mut self, node: &Node) -> bool {
        match node {
            Node::Function(name, _return_type, _args, block) => {
                self.lines.push(format!("{}:", name));
                self.lines.push(format!("    push rbp"));
                self.lines.push(format!("    mov rbp, rsp"));
                self.lines.push(format!("    sub rsp, {}", 32));

                self.gen(block);
            }
            Node::Block(statements) => {
                for statement in statements.iter() {
                    if self.gen(statement) {
                        // required to pop
                        self.lines.push(format!("    pop rax"));
                    }
                }
                return false;
            }
            /* 制御構文(control statements) */
            Node::If(if_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.gen(&if_arg.0);
                self.lines.push(format!("    pop rax"));
                self.lines.push(format!("    cmp rax, 0"));
                self.lines.push(format!("    je .Lend{}", label));
                // true case statement(s)
                if self.gen(&if_arg.1) {
                    // required to pop
                    self.lines.push(format!("    pop rax"));
                }
                self.lines.push(format!(".Lend{}:", label));
                return false;
            }
            Node::IfElse(if_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.gen(&if_arg.0);
                self.lines.push(format!("    pop rax"));
                self.lines.push(format!("    cmp rax, 0"));
                self.lines.push(format!("    je .Lelse{}", label));
                // true case statement(s)
                if self.gen(&if_arg.1) {
                    // required to pop
                    self.lines.push(format!("    pop rax"));
                    self.lines.push(format!("    pop rax"));
                }
                self.lines.push(format!("    jmp .Lend{}", label));
                self.lines.push(format!(".Lelse{}:", label));
                // otherwise statement(s)
                if self.gen(&if_arg.2) {
                    // required to pop
                    self.lines.push(format!("    pop rax"));
                }
                self.lines.push(format!(".Lend{}:", label));
                return false;
            }
            Node::While(while_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.lines.push(format!(".Lbegin{}:", label));
                self.gen(&while_arg.0);
                self.lines.push(format!("    pop rax"));
                self.lines.push(format!("    cmp rax, 0"));
                self.lines.push(format!("    je .Lend{}", label));
                // loop statement(s)
                if self.gen(&while_arg.1) {
                    // if single statement
                    self.lines.push(format!("    pop rax"));
                }
                self.lines.push(format!("    jmp .Lbegin{}", label));
                self.lines.push(format!(".Lend{}:", label));
                return false;
            }
            Node::For(for_arg) => {
                let label = self.label_count;
                self.label_count = self.label_count + 1;
                self.gen(&for_arg.0); // initialization
                self.lines.push(format!(".Lbegin{}:", label));
                self.gen(&for_arg.1); // loop condition
                self.lines.push(format!("    pop rax"));
                self.lines.push(format!("    cmp rax, 0"));
                self.lines.push(format!("    je .Lend{}", label));
                // loop statement(s)
                if self.gen(&for_arg.3) {
                    // if single statement
                    self.lines.push(format!("    pop rax"));
                }
                self.gen(&for_arg.2); // var update
                self.lines.push(format!("    jmp .Lbegin{}", label));
                self.lines.push(format!(".Lend{}:", label));
                return false;
            }
            /* 代入文 (assign statement) */
            Node::Assign(assign_args) => {
                self.gen_lval(&assign_args.0);
                self.gen(&assign_args.1);
                self.lines.push(format!("    pop rdi"));
                self.lines.push(format!("    pop rax"));
                self.lines.push(format!("    mov [rax], rdi"));
                self.lines.push(format!("    push rdi"));
            }
            /* return 文 (return statement) */
            Node::Return(return_expr_optional) => match return_expr_optional {
                None => {
                    self.lines.push(format!("    ret"));
                }
                Some(return_expr) => {
                    self.gen(&*return_expr);
                    self.lines.push(format!("    pop rax"));
                    self.lines.push(format!("    mov rsp, rbp"));
                    self.lines.push(format!("    pop rbp"));
                    self.lines.push(format!("    ret"));
                }
            },
            /* 式(expression) */
            Node::FunctionCall(name, arg_list) => {
                for (order, arg) in arg_list.iter().enumerate().rev() {
                    self.gen(arg);
                    if order < 6 {
                        self.lines.push(format!("    pop rax"));
                        let register_name = self.gen_function_arg_register(order);
                        self.lines.push(format!("    mov {}, rax", register_name));
                    }
                }
                self.lines.push(format!("    call {}", name));
                self.lines.push(format!("    push rax"));
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
                self.lines.push(format!("    pop rdi"));
                self.lines.push(format!("    pop rax"));
                match binary_type {
                    BinaryType::Add => {
                        self.lines.push(format!("    add rax, rdi"));
                    }
                    BinaryType::Sub => {
                        self.lines.push(format!("    sub rax, rdi"));
                    }
                    BinaryType::Mul => {
                        self.lines.push(format!("    imul rax, rdi"));
                    }
                    BinaryType::Div => {
                        self.lines.push(format!("    cqo"));
                        self.lines.push(format!("    idiv rdi"));
                    }
                    BinaryType::Equal => {
                        self.lines.push(format!("    cmp rax, rdi"));
                        self.lines.push(format!("    sete al"));
                        self.lines.push(format!("    movzb rax, al"));
                    }
                    BinaryType::NotEqual => {
                        self.lines.push(format!("    cmp rax, rdi"));
                        self.lines.push(format!("    setne al"));
                        self.lines.push(format!("    movzb rax, al"));
                    }
                    BinaryType::Lt => {
                        self.lines.push(format!("    cmp rax, rdi"));
                        self.lines.push(format!("    setl al"));
                        self.lines.push(format!("    movzb rax, al"));
                    }
                    BinaryType::LtEq => {
                        self.lines.push(format!("    cmp rax, rdi"));
                        self.lines.push(format!("    setle al"));
                        self.lines.push(format!("    movzb rax, al"));
                    }
                }
                self.lines.push(format!("    push rax"));
            }
            Node::LVar(_offset) => {
                self.gen_lval(node);
                self.lines.push(format!("    pop rax"));
                self.lines.push(format!("    mov rax, [rax]"));
                self.lines.push(format!("    push rax"));
            }
            Node::Num(n) => self.lines.push(format!("    push {}", n)),
            Node::Boolean(flag) => {
                if *flag {
                    self.lines.push(format!("    push 1"));
                } else {
                    self.lines.push(format!("    push 0"));
                }
            }
            Node::Empty => {}
        }
        return true;
    }

    pub fn gen_function_arg_register(&mut self, order: usize) -> String {
        // rdi, rsi, rdx, rcx, r8, r9
        match order {
            0 => "rdi",
            1 => "rsi",
            2 => "rdx",
            3 => "rcx",
            4 => "r8",
            5 => "r9",
            _ => "",
        }
        .to_string()
    }
}
