use crate::node::{LVar, Node};
use crate::token::{sprint_token, Token};

use super::Parser;

impl Parser {
    pub fn primary(&mut self) -> Node {
        eprintln!("primary() called");

        let token = self.token_iter.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
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
                return if self.token_iter.clone().next().unwrap().is_leftparen() {
                    // Function Call
                    self.token_iter.ignore(1);
                    let mut arg_list: Vec<Node> = Vec::new();

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
                    match self.local_vars.get(&name) {
                        None => {
                            self.local_vars.insert(
                                name.clone(),
                                LVar {
                                    offset: self.offset_last,
                                },
                            );
                            self.offset_last = self.offset_last + 32;
                            Node::LVar(self.offset_last - 32)
                        }
                        Some(local_var) => Node::LVar(local_var.offset),
                    }
                };
            }
            Token::Num(n) => Node::Num(n),
            _ => {
                panic!("Invalid Input");
            }
        };
    }
}
