use crate::node::Node;
use crate::token::Token;
use crate::typename::Typename;

use super::Parser;
impl Parser {
    fn parse_arglist(&mut self) -> Vec<(Typename, usize)> {
        let mut arg_list: Vec<(Typename, usize)> = Vec::new();
        match self.token_iter.peep().unwrap() {
            Token::RightParen => {
                self.token_iter.ignore(1);
                return arg_list;
            }
            _ => {
                let (typename, _name) = self.parse_declaration();
                arg_list.push((typename, 0));
            }
        }
        loop {
            match self.token_iter.next().unwrap() {
                Token::RightParen => {
                    return arg_list;
                }
                Token::Comma => {
                    let (typename, _name) = self.parse_declaration();
                    arg_list.push((typename, 0));
                }
                _ => {
                    panic!("Invaid argument expression for function call");
                }
            }
        }
    }

    pub fn function(&mut self) -> Node {
        // return_typename funcname
        let (return_typename, name) = self.parse_declaration();
        // (
        if !self.token_iter.next().unwrap_or(Token::Eof).is_leftparen() {
            panic!("missing '(' before argument list");
        }
        // arg1, arg2, arg3, ...)
        let arg_list = self.parse_arglist();

        // { // do something }
        Node::Function(name, return_typename, arg_list, Box::new(self.block()))
    }
}
