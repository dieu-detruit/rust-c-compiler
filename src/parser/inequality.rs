use crate::node::{BinaryType, Node};
use crate::token::{sprint_token, Token};

use super::Parser;
impl Parser {
    pub fn inequality(&mut self) -> Node {
        eprintln!("inequality() called");
        let mut node = self.add();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Lt => match token_iter_cp.next().unwrap_or(Token::Eof) {
                    Token::Equal => {
                        // <=
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((node, self.add())), BinaryType::LtEq);
                    }
                    _ => {
                        // <
                        self.token_iter.ignore(1);
                        node = Node::Binary(Box::new((node, self.add())), BinaryType::Lt);
                    }
                },
                Token::Gt => match token_iter_cp.next().unwrap_or(Token::Eof) {
                    Token::Equal => {
                        // >=
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((self.add(), node)), BinaryType::LtEq);
                    }
                    _ => {
                        // >
                        self.token_iter.ignore(1);
                        node = Node::Binary(Box::new((self.add(), node)), BinaryType::Lt);
                    }
                },
                _ => {
                    return node;
                }
            };
        }
    }
}
