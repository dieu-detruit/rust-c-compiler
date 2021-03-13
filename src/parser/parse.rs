use crate::node::Node;
use crate::tokenizer::TokenIter;
use std::collections::HashMap;

use super::Parser;
pub fn parse(token_iter: TokenIter) -> Node {
    let mut parser = Parser {
        token_iter: token_iter,
        local_vars: HashMap::new(),
        offset_last: 0,
    };

    parser.program()
}
