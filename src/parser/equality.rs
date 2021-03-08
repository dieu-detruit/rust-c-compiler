use crate::node::{BinaryType, Node};
use crate::token::{sprint_token, Token};

use super::Parser;
impl Parser {
    pub fn equality(&mut self) -> Node {
        eprintln!("equality() called");
        let mut node = self.inequality();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Equal => {
                    // ==
                    if let Token::Equal = token_iter_cp.next().unwrap_or(Token::Eof) {
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((node, self.inequality())), BinaryType::Equal);
                    } else {
                        return node;
                    }
                }
                Token::Exclamation => {
                    // !=
                    if let Token::Equal = token_iter_cp.next().unwrap_or(Token::Eof) {
                        self.token_iter.ignore(2);
                        node =
                            Node::Binary(Box::new((node, self.inequality())), BinaryType::NotEqual);
                    } else {
                        return node;
                    }
                }
                _ => {
                    return node;
                }
            };
        }
    }
}
