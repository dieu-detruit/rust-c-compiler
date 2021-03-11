use crate::node::Node;

use super::Parser;
impl Parser {
    pub fn program(&mut self) -> Node {
        let mut code: Vec<Node> = Vec::new();
        while self.token_iter.peep().is_some() {
            code.push(self.function());
        }
        Node::Block(code)
    }
}
