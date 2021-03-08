use std::collections::HashMap;

use crate::node::{LVar, Node};
use crate::tokenizer::{sprint_token_iter, tokenize, TokenIter};

/*
 * 生成文法
 *
 * program = function*
 * function = ident "(" declaration? ("," declaration)* ")" block
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
pub mod equality;
pub mod expression;
pub mod inequality;
pub mod mul;
pub mod primary;
pub mod program;
pub mod statement;
pub mod unary;

pub struct Parser {
    token_iter: TokenIter,
    local_vars: HashMap<String, LVar>,
    pub offset_last: usize,
}

impl Parser {
    //pub fn function(&mut self) -> Node {
    //let (return_typename, name) = self.declaration();

    //let block = self.block();
    //Function(name, return_typename)
    //}

    //pub fn declaration(&mut self) -> (Typename, String) {
    //// Identityの列を得る
    //let mut ident_list: Vec<String> = Vec::new();
    //ident_list.push(self.token_iter.next().unwrap());
    //ident_list.push(self.token_iter.next().unwrap());
    //if (ident_list[0].is_identity() || ident_list[1].is_identity) {
    //panic!("Invalid input for declaration");
    //}
    //while let Token::Identity(_) = self.token_iter.clone().next().unwrap_or(Token::Eof) {
    //ident_list.push(self.token_iter.next().unwrap());
    //}
    //// パース
    //let name = ident_list.pop();
    //let typename = parse_typename(ident_list);
    //}

    //pub fn block(&mut self) -> Node {
    //self.token_iter.ignore(1);
    //let mut statements: Vec<Node> = Vec::new();
    //loop {
    //if let Token::RightCurl = self.token_iter.clone().next().unwrap_or(Token::Eof) {
    //self.token_iter.ignore(1);
    //return Node::Block(statements);
    //} else {
    //statements.push(self.statement());
    //}
    //}
    //}
}

pub fn parse(prog: String) -> (Node, usize) {
    let mut token_iter = tokenize(prog.clone());
    let output = sprint_token_iter(token_iter);
    eprintln!("{}", output);
    token_iter = tokenize(prog);

    let mut parser = Parser {
        token_iter: token_iter,
        local_vars: HashMap::new(),
        offset_last: 0,
    };

    return (parser.program(), parser.offset_last - 32);
}
