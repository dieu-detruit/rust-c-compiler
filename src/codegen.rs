use crate::node::{BinaryType, Node};
use crate::register::Register;
use crate::typename::{sizeof, Typename};

pub struct CodeGenerator {
    pub lines: Vec<String>,
    pub label_count: usize,
    pub rsp_sub_size: usize,
    pub label_func: usize,
}

fn stack_align(size: usize) -> usize {
    if size % 16 == 0 {
        size
    } else {
        16 * ((size / 16) + 1) - 8
    }
}

pub fn gen_lval_ptr(offset: usize, typename: &Typename) -> String {
    match typename {
        Typename::Integer(_, size) => format!(
            "{} PTR [rbp-{:#0x}]",
            match size {
                1 => "BYTE",
                2 => "WORD",
                4 => "DWORD",
                8 => "QWORD",
                _ => "",
            },
            8 + offset
        ),
        _ => String::new(),
    }
}

impl CodeGenerator {
    // -> bool : require pop stack
    pub fn gen(&mut self, node: &Node) -> bool {
        match node {
            Node::Function(name, _return_type, arg_types, block, local_var_size) => {
                self.lines.push(format!("{}:", name));
                self.lines.push(format!("    endbr64"));
                self.lines.push(format!("    push rbp"));
                self.lines.push(format!("    mov rbp, rsp"));
                self.lines.push(format!("    push rdx"));
                // rbpをpushするぶんを合わせる
                self.rsp_sub_size = stack_align(*local_var_size);
                self.lines
                    .push(format!("    sub rsp, {:#0x}", self.rsp_sub_size));
                // load arguments
                let mut offset: usize = 0;
                for (order, arg_type) in arg_types.iter().enumerate() {
                    if order < 6 {
                        let register_name = self
                            .gen_function_arg_register(order)
                            .get_name(sizeof(&arg_type));
                        self.lines.push(format!(
                            "    mov {0}, {1}",
                            gen_lval_ptr(offset, &arg_type),
                            register_name
                        ));
                    }
                    offset += sizeof(arg_type)
                }
                self.gen(&block);
                // 終了処理
                self.lines.push(format!(".Lendfunc{}:", self.label_func));
                self.lines
                    .push(format!("    add rsp, {:#0x}", self.rsp_sub_size));
                self.lines.push(format!("    pop rdx"));
                self.lines.push(format!("    pop rbp"));
                self.lines.push(format!("    ret"));
                self.label_func += 1;
                return false;
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
                // var update
                if self.gen(&for_arg.2) {
                    // if single statement
                    self.lines.push(format!("    pop rax"));
                }
                self.lines.push(format!("    jmp .Lbegin{}", label));
                self.lines.push(format!(".Lend{}:", label));
                return false;
            }
            /* 代入文 (assign statement) */
            Node::Assign(assign_args) => {
                self.gen(&assign_args.1);
                self.lines.push(format!("    pop rdi"));
                //self.lines.push(format!("    pop rax"));
                let (offset, typename) = assign_args.0.expect_lvar();
                self.lines.push(format!(
                    "    mov {}, {}",
                    gen_lval_ptr(offset, &typename),
                    Register::RDI.get_name(sizeof(&typename))
                ));
                self.lines.push(format!("    push rdi"));
            }
            /* return 文 (return statement) */
            Node::Return(return_expr_optional) => {
                if let Some(return_expr) = return_expr_optional {
                    self.gen(&*return_expr);
                    self.lines.push(format!("    pop rax"));
                }
                self.lines
                    .push(format!("    jmp .Lendfunc{}", self.label_func));
                return false;
            }
            /* 式(expression) */
            Node::FunctionCall(name, arg_list) => {
                for (order, arg) in arg_list.iter().enumerate().rev() {
                    self.gen(arg);
                    if order < 6 {
                        self.lines.push(format!("    pop rax"));
                        let register_name = self.gen_function_arg_register(order).get_name(8);
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
            Node::LVar(offset, typename) => {
                self.lines.push(format!(
                    "    mov {}, {}",
                    Register::RAX.get_name(sizeof(typename)),
                    gen_lval_ptr(*offset, typename)
                ));
                self.lines.push(format!("    push rax"));
            }
            Node::Num(n) => {
                self.lines.push(format!("    push {:#0x}", n));
            }
            Node::Boolean(flag) => {
                if *flag {
                    self.lines.push(format!("    push 1"));
                } else {
                    self.lines.push(format!("    push 0"));
                }
            }
            Node::Empty => {
                return false;
            }
        }
        return true;
    }

    pub fn gen_function_arg_register(&mut self, order: usize) -> Register {
        // rdi, rsi, rdx, rcx, r8, r9
        match order {
            0 => Register::RDI,
            1 => Register::RSI,
            2 => Register::RDX,
            3 => Register::RCX,
            4 => Register::Rn(8),
            5 => Register::Rn(9),
            _ => panic!("out of range"),
        }
    }
}
