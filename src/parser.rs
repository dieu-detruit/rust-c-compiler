pub mod tokenizer;

use tokenizer::{sprint_token, sprint_token_iter, tokenize, Token, TokenIter};

/*
 * 生成文法
 *
 * expr = mul ( "+" mul | "-" mul )*
 * mul = unary ( "*" unary | "/" unary )*
 * unary = ( "+" | "-" )? primary
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

pub struct Parser<'a> {
    token_iter: TokenIter<'a>,
}

impl Parser<'_> {
    pub fn expr(&mut self) -> Node {
        eprintln!("expr() called");
        let mut node = self.mul();
        eprintln!("mul() returned");

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Plus => {
                    eprintln!("Plus");
                    self.token_iter.next();
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Add);
                }
                Token::Minus => {
                    eprintln!("Minus");
                    self.token_iter.next();
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Sub);
                }
                _ => {
                    return node;
                }
            };
        }
    }
    pub fn mul(&mut self) -> Node {
        eprintln!("mul() called");
        let mut node = self.unary();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Asterisk => {
                    eprintln!("Asterisk");
                    self.token_iter.next();
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Mul)
                }
                Token::Slash => {
                    eprintln!("Slash");
                    self.token_iter.next();
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Div)
                }
                _ => {
                    return node;
                }
            };
        }
    }
    pub fn unary(&mut self) -> Node {
        eprintln!("unary() called");

        let mut token_iter_cp = self.token_iter.clone();
        let token = token_iter_cp.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
            Token::Plus => {
                eprintln!("UnaryPlus");
                self.token_iter.next();
                self.primary()
            }
            Token::Minus => {
                eprintln!("UnaryMinus");
                self.token_iter.next();
                Node::Binary(Box::new((Node::Num(0), self.primary())), BinaryType::Sub)
            }
            _ => self.primary(),
        };
    }
    pub fn primary(&mut self) -> Node {
        let token = self.token_iter.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        eprintln!("primary() called");
        return match token {
            Token::LeftParen => {
                eprintln!("Left Paren");
                let node_expr = self.expr();
                if let Token::RightParen = self.token_iter.next().unwrap() {
                    node_expr
                } else {
                    panic!("Invalid Input");
                }
            }
            Token::Num(n) => {
                eprintln!("Num");
                Node::Num(n)
            }
            _ => {
                panic!("Invalid Input");
            }
        };
    }
}

pub fn parse(prog: &str) -> Node {
    let mut token_iter = tokenize(prog);
    let output = sprint_token_iter(token_iter);
    eprintln!("{}", output);
    token_iter = tokenize(prog);

    let mut parser = Parser {
        token_iter: token_iter,
    };
    return parser.expr();
}

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
                _ => {}
            }
            println!("    push rax");
        }
        Node::Num(n) => println!("    push {}", n),
        _ => {}
    }
}

pub fn sprint_node(node: &Node) -> String {
    match node {
        Node::Num(n) => n.to_string(),
        Node::Boolean(b) => {
            if *b {
                String::from("True")
            } else {
                String::from("False")
            }
        }
        Node::Unary(_unary_arg, _unary_type) => String::from(""),
        Node::Binary(binary_arg, binary_type) => {
            return match binary_type {
                BinaryType::Add => String::from("+"),
                BinaryType::Sub => String::from("-"),
                BinaryType::Mul => String::from("*"),
                BinaryType::Div => String::from("/"),
                _ => String::from(""),
            } + "("
                + &sprint_node(&binary_arg.0)
                + ","
                + &sprint_node(&binary_arg.1)
                + ")"
        }
        _ => String::from(""),
    }
}
