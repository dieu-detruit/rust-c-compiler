use crate::node::Node;
use crate::token::{sprint_token, Token};

use super::Parser;

impl Parser {
    pub fn primary(&mut self) -> Node {
        eprintln!("primary()");
        eprintln!(
            "next token: {}",
            sprint_token(&self.token_iter.peep().unwrap_or(Token::Eof))
        );
        match self.token_iter.next().unwrap_or(Token::Eof) {
            Token::LeftParen => {
                // ( expression )
                let node_expression = self.expression();
                if let Token::RightParen = self.token_iter.next().unwrap() {
                    node_expression
                } else {
                    panic!("Invalid Input");
                }
            }
            Token::Identity(name) => {
                return if self.token_iter.peep().unwrap().is_leftparen() {
                    // Function Call
                    eprintln!("this is function call");
                    self.token_iter.ignore(1);
                    let mut arg_list: Vec<Node> = Vec::new();

                    if self.functions.get(&name).is_none() {
                        panic!("function named {} is not declared here", name);
                    }

                    match self.token_iter.clone().next().unwrap() {
                        Token::RightParen => {
                            // Call func()
                            self.token_iter.ignore(1);
                            return Node::FunctionCall(name, arg_list);
                        }
                        _ => {
                            arg_list.push(self.expression());
                        }
                    }
                    loop {
                        match self.token_iter.next().unwrap() {
                            Token::RightParen => {
                                // Call func(args)
                                return Node::FunctionCall(name, arg_list);
                            }
                            Token::Comma => {
                                arg_list.push(self.expression());
                            }
                            _ => {
                                panic!("Invaid argument expression for function call {}", name);
                            }
                        }
                    }
                } else {
                    // Variable
                    let var = self
                        .local_vars
                        .get(&name)
                        .expect("'{}' is not declared in this scope");
                    Node::LVar(var.offset, var.typename.clone())
                };
            }
            Token::Num(n) => Node::Num(n),
            _ => {
                panic!("Invalid Input");
            }
        }
    }
}
