use crate::node::Node;
use crate::token::{sprint_token, Token};

use super::Parser;
impl Parser {
    pub fn assign(&mut self) -> Node {
        eprintln!("assign() called");
        let node = self.equality();
        let mut token_iter_cp = self.token_iter.clone();
        let token = token_iter_cp.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
            Token::Equal => {
                self.token_iter.ignore(1);
                Node::Assign(Box::new((node, self.assign())))
            }
            _ => node,
        };
    }
}
