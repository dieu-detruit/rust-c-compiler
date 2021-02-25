use crate::node::{BinaryType, Node, UnaryType};
use crate::tokenizer::{sprint_token, sprint_token_iter, tokenize, Token, TokenIter};

/*
 * 生成文法
 *
 * expr = equality
 * equality = inequality ( "==" inequality | "!=" inequality )*
 * inequality = add ( "<" add | "<=" add | ">" add | ">=" add )*
 * add = mul ( "+" mul | "-" mul )*
 * mul = unary ( "*" unary | "/" unary )*
 * unary = ( "+" | "-" )? primary
 * primary = num | "(" expr ")"
 *
 */

pub struct Parser<'a> {
    token_iter: TokenIter<'a>,
}

impl Parser<'_> {
    pub fn expr(&mut self) -> Node {
        self.equality()
    }

    pub fn equality(&mut self) -> Node {
        eprintln!("equality() called");
        let mut node = self.inequality();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Equal => {
                    if let Token::Equal = token_iter_cp.next().unwrap_or(Token::Eof) {
                        eprintln!("Equal");
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((node, self.inequality())), BinaryType::Equal);
                    } else {
                        panic!("Invalid Input");
                    }
                }
                Token::Exclamation => {
                    if let Token::Equal = token_iter_cp.next().unwrap_or(Token::Eof) {
                        eprintln!("NotEqual");
                        self.token_iter.ignore(2);
                        node =
                            Node::Binary(Box::new((node, self.inequality())), BinaryType::NotEqual);
                    } else {
                        panic!("Invalid Input");
                    }
                }
                _ => {
                    return node;
                }
            };
        }
    }

    pub fn inequality(&mut self) -> Node {
        eprintln!("inequality() called");
        let mut node = self.add();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Lt => match token_iter_cp.next().unwrap_or(Token::Eof) {
                    Token::Equal => {
                        eprintln!("LtEq");
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((node, self.add())), BinaryType::LtEq);
                    }
                    _ => {
                        eprintln!("Lt");
                        self.token_iter.ignore(1);
                        node = Node::Binary(Box::new((node, self.add())), BinaryType::Lt);
                    }
                },
                Token::Gt => match token_iter_cp.next().unwrap_or(Token::Eof) {
                    Token::Equal => {
                        eprintln!("GtEq");
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((self.add(), node)), BinaryType::LtEq);
                    }
                    _ => {
                        eprintln!("Gt");
                        self.token_iter.ignore(1);
                        node = Node::Binary(Box::new((self.add(), node)), BinaryType::Lt);
                    }
                },
                _ => {
                    return node;
                }
            };
        }
    }

    pub fn add(&mut self) -> Node {
        eprintln!("add() called");
        let mut node = self.mul();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Plus => {
                    eprintln!("Plus");
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Add);
                }
                Token::Minus => {
                    eprintln!("Minus");
                    self.token_iter.ignore(1);
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
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Mul)
                }
                Token::Slash => {
                    eprintln!("Slash");
                    self.token_iter.ignore(1);
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
                self.token_iter.ignore(1);
                self.primary()
            }
            Token::Minus => {
                eprintln!("UnaryMinus");
                self.token_iter.ignore(1);
                Node::Binary(Box::new((Node::Num(0), self.primary())), BinaryType::Sub)
            }
            _ => self.primary(),
        };
    }
    pub fn primary(&mut self) -> Node {
        eprintln!("primary() called");

        let token = self.token_iter.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

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
