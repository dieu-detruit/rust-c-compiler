use std::collections::HashMap;

use crate::node::{Function, Node};
use crate::tokenizer::TokenIter;
use crate::typename::{sizeof, Typename};

use super::Parser;
impl Parser {
    pub fn sizeof(&self, node: &Node) {
        use std::cmp;
        use Node::*;
        match node {
            Unary(_arg, unary_type) => match unary_type {
                UnaryType::Not => sizoef(&Typename::Int),
            },
            Binary(args, binary_type) => match binary_type {
                Add | Sub | Mul | Div => cmp::max(sizeof(&args.0), sizeof(&args.1)),
                Equal | NotEqual | Lt | LtEq => sizeof(&Typename::Boolean),
            },
            Num(n) => sizeof(&Typename::Int),
            Boolean(f) => sizeof(&Typename::Boolean),
            LVar(_offset, typename) => sizeof(&typename),
            Assign(_) | Return(_) | If(_) | IfElse(_) | For(_) | While(_) | Block(_) => {
                panic!("statement cannot be the target of sizeof()")
            }
            Function(name, return_type, arg_types, block, local_var_size) => {
                panic!("function node cannot be the target of sizeof()")
            }
            FunctionCall(name, _args) => {
                let func = self
                    .functions
                    .get(&name)
                    .expect("no such function is declared");
                sizeof(func.ret_typename)
            }
            Empty => {}
        }
    }
}
