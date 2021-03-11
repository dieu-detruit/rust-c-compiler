use crate::node::{BinaryType, Node};
use crate::token::Token;

use super::Parser;
impl Parser {
    pub fn inequality(&mut self) -> Node {
        let mut node = self.add();

        loop {
            match self.token_iter.peep().unwrap_or(Token::Eof) {
                Token::Lt => {
                    self.token_iter.ignore(1);
                    match self.token_iter.peep().unwrap_or(Token::Eof) {
                        Token::Equal => {
                            // <=
                            self.token_iter.ignore(1);
                            node = Node::Binary(Box::new((node, self.add())), BinaryType::LtEq);
                        }
                        _ => {
                            // <
                            node = Node::Binary(Box::new((node, self.add())), BinaryType::Lt);
                        }
                    }
                }
                Token::Gt => {
                    self.token_iter.ignore(1);
                    match self.token_iter.peep().unwrap_or(Token::Eof) {
                        Token::Equal => {
                            // >=
                            self.token_iter.ignore(1);
                            node = Node::Binary(Box::new((self.add(), node)), BinaryType::LtEq);
                        }
                        _ => {
                            // >
                            node = Node::Binary(Box::new((self.add(), node)), BinaryType::Lt);
                        }
                    }
                }
                _ => {
                    return node;
                }
            };
        }
    }
}
