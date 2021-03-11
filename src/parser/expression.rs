use crate::node::Node;

use super::Parser;
impl Parser {
    pub fn expression(&mut self) -> Node {
        self.assign()
    }
}
