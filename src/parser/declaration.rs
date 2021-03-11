use crate::node::{LVar, Node};
use crate::token::Token;
use crate::typename::{is_typename_token, parse_typename, Typename};

use super::Parser;
impl Parser {
    pub fn parse_declaration(&mut self) -> (Typename, String) {
        // Tokenの列を作る
        let mut ident_list: Vec<Token> = Vec::new();
        ident_list.push(self.token_iter.next().unwrap());
        ident_list.push(self.token_iter.next().unwrap());
        if !is_typename_token(&ident_list[0]) || !is_typename_token(&ident_list[1]) {
            panic!("Invalid input for declaration");
        }
        while is_typename_token(&self.token_iter.peep().unwrap_or(Token::Eof)) {
            ident_list.push(self.token_iter.next().unwrap());
        }
        // パース
        let name = ident_list.pop().unwrap().expect_identity();
        (parse_typename(ident_list), name)
    }

    pub fn declaration(&mut self) -> Node {
        let (typename, name) = self.parse_declaration();
        let size = match typename {
            Typename::Void => panic!("void type cannot be used for variable declaration"),
            Typename::Integer(_, size) => size,
            _ => panic!("user defined type is not supported on this compiler"),
        };
        match self
            .local_vars
            .get(&(self.current_block_id.to_string() + &name))
        {
            Some(_) => panic!("redelaration of variable '{}' in this scope", name),
            None => {
                self.local_vars.insert(
                    self.current_block_id.to_string() + &name,
                    LVar {
                        offset: self.offset_last,
                        size: size,
                    },
                );
                self.offset_last = self.offset_last + 8 * size;
            }
        };
        Node::Empty
    }
}
