use crate::node::{BinaryType, Node};
use crate::token::Token;

use super::Parser;

impl Parser {
    pub fn unary(&mut self) -> Node {
        return match self.token_iter.peep().unwrap_or(Token::Eof) {
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
