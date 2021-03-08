use crate::node::{BinaryType, Node};
use crate::token::{sprint_token, Token};

use super::Parser;
impl Parser {
    pub fn add(&mut self) -> Node {
        eprintln!("add() called");
        let mut node = self.mul();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
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
