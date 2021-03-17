use crate::node::{Function, Node};
use crate::token::Token;
use crate::typename::Typename;

use super::Parser;
impl Parser {
    fn parse_arglist(&mut self) -> Vec<Typename> {
        let mut arg_list: Vec<Typename> = Vec::new();
        match self.token_iter.peep().unwrap() {
            Token::RightParen => {
                self.token_iter.ignore(1);
                return arg_list;
            }
            _ => {
                let typename = self
                    .declaration_impl()
                    .expect("invalid function argument list");
                arg_list.push(typename);
            }
        }
        loop {
            match self.token_iter.next().unwrap() {
                Token::RightParen => {
                    return arg_list;
                }
                Token::Comma => {
                    let typename = self
                        .declaration_impl()
                        .expect("invalid function argument list");
                    arg_list.push(typename);
                }
                _ => {
                    panic!("Invaid argument expression for function call");
                }
            }
        }
    }

    pub fn function(&mut self) -> Node {
        self.local_vars.clear();
        self.offset_last = 0;

        // return_typename funcname
        let (return_typename, name) = self
            .parse_declaration()
            .expect("invalid function declaration");
        // (
        if !self.token_iter.next().unwrap_or(Token::Eof).is_leftparen() {
            panic!("missing '(' before argument list");
        }
        // arg1, arg2, arg3, ...)
        let arg_list = self.parse_arglist();

        // { // do something }
        if self.token_iter.peep().unwrap_or(Token::Eof).is_semicolon() {
            self.token_iter.ignore(1);
            match self.functions.get(&name) {
                Some(_) => panic!("redelaration of function named {}", name),
                None => {
                    self.functions.insert(
                        name,
                        Function {
                            ret_typename: return_typename,
                            arg_typename: arg_list,
                        },
                    );
                }
            };
            Node::Empty
        } else {
            if self.functions.get(&name).is_none() {
                self.functions.insert(
                    name.clone(),
                    Function {
                        ret_typename: return_typename.clone(),
                        arg_typename: arg_list.clone(),
                    },
                );
            }
            let block = self.block();
            Node::Function(
                name,
                return_typename,
                arg_list,
                Box::new(block),
                self.offset_last,
            )
        }
    }
}
