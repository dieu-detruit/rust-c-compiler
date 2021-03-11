use crate::node::Node;
use crate::token::Token;

use super::Parser;
impl Parser {
    pub fn assign(&mut self) -> Node {
        let node = self.equality();

        return match self.token_iter.peep().unwrap_or(Token::Eof) {
            Token::Equal => {
                self.token_iter.ignore(1);
                Node::Assign(Box::new((node, self.assign())))
            }
            _ => node,
        };
    }
}
