pub mod tokenizer;

use tokenizer::{tokenize, Token, TokenIter};

/*
 * 生成文法
 *
 * expr = mul ( "+" mul | "-" mul )*
 * mul = num ( "*" num | "/" num )*
 * primary = num | "(" expr ")"
 *
 */

pub enum Node {
    Unary(Box<Node>, UnaryType),
    Binary(Box<(Node, Node)>, BinaryType),
    Num(i32),
    Boolean(bool),
}

pub enum UnaryType {
    Plus,
    Minus,
    Not,
}

pub enum BinaryType {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

pub struct Tokenizer<'a> {
    token_iter: &'a mut TokenIter<'a>,
}

impl Tokenizer<'_> {
    pub fn expr(&mut self) -> Node {
        let node_mul = self.mul();

        match self.token_iter.next().unwrap() {
            Token::Plus => Node::Binary(Box::new((node_mul, self.mul())), BinaryType::Add),
            Token::Minus => Node::Binary(Box::new((node_mul, self.mul())), BinaryType::Sub),
            _ => return node_mul,
        }
    }
    pub fn mul(&mut self) -> Node {
        let node_primary = self.primary();

        match self.token_iter.next().unwrap() {
            Token::Asterisk => {
                Node::Binary(Box::new((node_primary, self.primary())), BinaryType::Mul)
            }
            Token::Slash => Node::Binary(Box::new((node_primary, self.primary())), BinaryType::Div),
            _ => node_primary,
        }
    }
    pub fn primary(&mut self) -> Node {
        match self.token_iter.next().unwrap() {
            Token::LeftParen => {
                let node_expr = self.expr();
                if let Token::RightParen = self.token_iter.next().unwrap() {
                    node_expr
                } else {
                    panic!("Invalid Input");
                }
            }
            Token::Num(n) => Node::Num(n),
            _ => {
                panic!("Invalid Input");
            }
        }
    }
}

pub fn parse(prog: &str) -> Node {
    let mut token_iter = tokenize(prog);
    let mut tokenizer = Tokenizer {
        token_iter: &mut token_iter,
    };
    return tokenizer.expr();
}

pub fn gen(node: Node) {
    match node {
        Node::Unary(unary_arg, unary_type) => {
            gen(*unary_arg);
            match unary_type {
                _ => {}
            }
        }
        Node::Binary(binary_arg, binary_type) => {
            gen(binary_arg.0);
            gen(binary_arg.1);
            match binary_type {
                BinaryType::Add => {
                    println!("  add rax, rdi");
                }
                BinaryType::Sub => {
                    println!("  sub rax, rdi");
                }
                BinaryType::Mul => {
                    println!("  imul rax, rdi");
                }
                BinaryType::Div => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                _ => {}
            }
        }
        Node::Num(n) => println!("  push {}", n),
        _ => {}
    }
    println!("  push rax");
}
