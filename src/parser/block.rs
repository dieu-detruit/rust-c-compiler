use crate::node::Node;
use crate::token::Token;

use super::Parser;
impl Parser {
    pub fn block(&mut self) -> Node {
        if !self.token_iter.next().unwrap_or(Token::Eof).is_leftcurl() {
            panic!("missing left curl before argument list");
        }
        let mut statements: Vec<Node> = Vec::new();

        loop {
            if let Token::RightCurl = self.token_iter.peep().unwrap_or(Token::Eof) {
                self.token_iter.ignore(1);
                return Node::Block(statements);
            } else {
                statements.push(self.statement());
            }
        }
    }
}
