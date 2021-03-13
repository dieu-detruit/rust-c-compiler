use crate::node::{LVar, Node};
use crate::token::{sprint_token, Token};
use crate::typename::{is_typename_token, parse_typename, sizeof, Typename};

use super::Parser;
impl Parser {
    pub fn parse_declaration(&mut self) -> Option<(Typename, String)> {
        // Tokenの列を作る
        let mut ident_list: Vec<Token> = Vec::new();
        let mut token_iter_clone = self.token_iter.clone();
        ident_list.push(token_iter_clone.next().unwrap());
        ident_list.push(token_iter_clone.next().unwrap());
        eprintln!("first token: {}", sprint_token(&ident_list[0]));
        eprintln!("second token: {}", sprint_token(&ident_list[1]));
        if !is_typename_token(&ident_list[0]) || !is_typename_token(&ident_list[1]) {
            return None;
        }
        while is_typename_token(&token_iter_clone.peep().unwrap_or(Token::Eof)) {
            ident_list.push(token_iter_clone.next().unwrap());
        }
        // パース
        match ident_list.pop().unwrap_or(Token::Eof) {
            Token::Identity(name) => {
                self.token_iter.ignore(ident_list.len() + 1);
                Some((parse_typename(ident_list), name))
            }
            _ => None,
        }
    }

    pub fn declaration_impl(&mut self) -> Option<Typename> {
        match self.parse_declaration() {
            None => None,
            Some(parsed_result) => {
                let (typename, name) = parsed_result;
                if let Typename::Void = typename {
                    panic!("variable cannot be declared as void type")
                }
                match self.local_vars.get(&name) {
                    Some(_) => panic!("redelaration of variable '{}' in this scope", name),
                    None => {
                        self.local_vars.insert(
                            name,
                            LVar {
                                offset: self.offset_last,
                                typename: typename.clone(),
                            },
                        );
                        self.offset_last += sizeof(&typename);
                    }
                };
                Some(typename)
            }
        }
    }

    pub fn declaration(&mut self) -> Option<Node> {
        match self.declaration_impl() {
            None => None,
            Some(typename) => match typename {
                Typename::Void => panic!("variable cannot be declared as void type"),
                _ => Some(Node::Empty),
            },
        }
    }
}
