use crate::node::LVar;
use crate::tokenizer::TokenIter;
use std::collections::HashMap;

/*
 * 生成文法
 *
 * program = function*
 * function = declaration "(" declaration? ("," declaration)* ")" block
 * declaration = ident+ ident
 * block = "{" statement* "}"
 * statement    = expression ";"
 *              | declaration ";"
 *              | ident ident ";"
 *              | block
 *              | "if" "(" expression ")" statement ( "else" statement )?
 *              | "while" "(" expression ")" statement
 *              | "for" "(" expression? ";" expression? ";" expression? ")" statement
 *              | "return" expression ";"
 * expression = assign
 * assign = equality ( "=" assign )?
 * equality = inequality ( "==" inequality | "!=" inequality )*
 * inequality = add ( "<" add | "<=" add | ">" add | ">=" add )*
 * add = mul ( "+" mul | "-" mul )*
 * mul = unary ( "*" unary | "/" unary )*
 * unary = ( "+" | "-" )? primary
 * primary  = num
 *          | ident ( "(" expr? ("," expr )* ")" )?
 *          | "(" expression ")"
 *
 *
 */

pub mod add;
pub mod assign;
pub mod block;
pub mod declaration;
pub mod equality;
pub mod expression;
pub mod function;
pub mod inequality;
pub mod mul;
pub mod parse;
pub mod primary;
pub mod program;
pub mod statement;
pub mod unary;

pub struct Parser {
    token_iter: TokenIter,
    pub local_vars: HashMap<String, LVar>,
    pub current_block_id: usize,
    pub current_block_var_size: usize,
    pub offset_last: usize,
}
