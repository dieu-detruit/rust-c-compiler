use crate::node::Node;
use crate::token::{sprint_token, Token};

use super::Parser;

impl Parser {
    pub fn statement(&mut self) -> Node {
        eprintln!("statement() called");

        let mut token_iter_cp = self.token_iter.clone();
        let token = token_iter_cp.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
            Token::LeftCurl => {
                self.token_iter.ignore(1);
                let mut statements: Vec<Node> = Vec::new();
                loop {
                    if let Token::RightCurl = self.token_iter.clone().next().unwrap_or(Token::Eof) {
                        self.token_iter.ignore(1);
                        return Node::Block(statements);
                    } else {
                        statements.push(self.statement());
                    }
                }
            }
            Token::Return => {
                self.token_iter.ignore(1);
                let return_expression = self.expression();
                if let Token::Semicolon = self.token_iter.next().unwrap_or(Token::Eof) {
                    Node::Return(Box::new(return_expression))
                } else {
                    panic!("Missing Semicolon")
                }
            }
            Token::If => {
                self.token_iter.ignore(1);
                // expect "("
                if !self.token_iter.next().unwrap_or(Token::Eof).is_leftparen() {
                    panic!("Missing '(' in \"if\" statement");
                }
                // if condition
                let cond = self.expression();
                // expect ")"
                if !self.token_iter.next().unwrap_or(Token::Eof).is_rightparen() {
                    panic!("Missing ')' in \"if\" statement");
                }
                // statement executed if true
                let statement_true = self.statement();

                if let Token::Else = self.token_iter.clone().next().unwrap_or(Token::Eof) {
                    self.token_iter.ignore(1);
                    Node::IfElse(Box::new((cond, statement_true, self.statement())))
                } else {
                    Node::If(Box::new((cond, statement_true)))
                }
            }
            Token::For => {
                self.token_iter.ignore(1);
                // expect "("
                if !self.token_iter.next().unwrap_or(Token::Eof).is_leftparen() {
                    panic!("Missing '(' in \"for\" statement");
                }
                // initialize expression
                let initialize_expression = if let Token::Semicolon =
                    self.token_iter.clone().next().unwrap_or(Token::Eof)
                {
                    Node::Empty
                } else {
                    self.expression()
                };
                // expect ";"
                if !self.token_iter.next().unwrap_or(Token::Eof).is_semicolon() {
                    panic!("Missing first ';' in \"for\" statement");
                }
                // loop condition
                let loop_condition =
                    if let Token::Semicolon = token_iter_cp.next().unwrap_or(Token::Eof) {
                        Node::Boolean(true)
                    } else {
                        self.expression()
                    };
                // expect ";"
                if !self.token_iter.next().unwrap_or(Token::Eof).is_semicolon() {
                    panic!("Missing second ';' in \"for\" statement");
                }
                // update expression
                let update_expression =
                    if let Token::RightParen = token_iter_cp.next().unwrap_or(Token::Eof) {
                        self.token_iter.ignore(1);
                        Node::Empty
                    } else {
                        self.expression()
                    };
                // expect ")"
                if !self.token_iter.next().unwrap_or(Token::Eof).is_rightparen() {
                    panic!("Missing ')' in \"for\" statement");
                }
                Node::For(Box::new((
                    initialize_expression,
                    loop_condition,
                    update_expression,
                    self.statement(),
                )))
            }
            Token::While => {
                self.token_iter.ignore(1);
                // expect "("
                if !self.token_iter.next().unwrap_or(Token::Eof).is_leftparen() {
                    panic!("Missing '(' in \"while\" statement");
                }
                // loop condition
                let cond = self.expression();
                // expect ")"
                if !self.token_iter.next().unwrap_or(Token::Eof).is_rightparen() {
                    panic!("Missing ')' in \"while\" statement");
                }
                Node::While(Box::new((cond, self.statement())))
            }
            _ => {
                let node = self.expression();
                if let Token::Semicolon = self.token_iter.next().unwrap_or(Token::Eof) {
                    node
                } else {
                    panic!("Missing Semicolon");
                }
            }
        };
    }
}
