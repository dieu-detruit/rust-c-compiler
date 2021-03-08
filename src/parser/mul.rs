use crate::node::{BinaryType, Node};
use crate::token::{sprint_token, Token};

use super::Parser;
impl Parser {
    pub fn mul(&mut self) -> Node {
        eprintln!("mul() called");
        let mut node = self.unary();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
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
