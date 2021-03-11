use crate::node::{BinaryType, Node};
use crate::token::Token;

use super::Parser;
impl Parser {
    pub fn add(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            match self.token_iter.peep().unwrap_or(Token::Eof) {
                Token::Plus => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Add);
                }
                Token::Minus => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Sub);
                }
                _ => {
                    return node;
                }
            };
        }
    }
}
