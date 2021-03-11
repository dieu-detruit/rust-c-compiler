use crate::node::{BinaryType, Node};
use crate::token::Token;

use super::Parser;
impl Parser {
    pub fn mul(&mut self) -> Node {
        let mut node = self.unary();

        loop {
            match self.token_iter.peep().unwrap_or(Token::Eof) {
                Token::Asterisk => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Mul)
                }
                Token::Slash => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Div)
                }
                _ => {
                    return node;
                }
            };
        }
    }
}
