use crate::node::{BinaryType, Node};
use crate::token::{sprint_token, Token};

use super::Parser;

impl Parser {
    pub fn unary(&mut self) -> Node {
        eprintln!("unary() called");

        let mut token_iter_cp = self.token_iter.clone();
        let token = token_iter_cp.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
            Token::Plus => {
                self.token_iter.ignore(1);
                self.primary()
            }
            Token::Minus => {
                self.token_iter.ignore(1);
                Node::Binary(Box::new((Node::Num(0), self.primary())), BinaryType::Sub)
            }
            _ => self.primary(),
        };
    }
}
