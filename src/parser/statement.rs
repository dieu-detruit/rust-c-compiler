use crate::node::Node;
use crate::token::Token;

use super::Parser;

impl Parser {
    pub fn statement(&mut self) -> Node {
        return match self.token_iter.peep().unwrap_or(Token::Eof) {
            Token::LeftCurl => self.block(),
            Token::Semicolon => Node::Empty,
            Token::Return => {
                self.token_iter.ignore(1);
                if let Token::Semicolon = self.token_iter.peep().unwrap_or(Token::Eof) {
                    self.token_iter.ignore(1);
                    Node::Return(None)
                } else {
                    let return_expression = self.expression();
                    if let Token::Semicolon = self.token_iter.next().unwrap_or(Token::Eof) {
                        Node::Return(Some(Box::new(return_expression)))
                    } else {
                        panic!("Missing Semicolon at the end of the statement")
                    }
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

                if let Token::Else = self.token_iter.peep().unwrap_or(Token::Eof) {
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
                let initialize_expression =
                    if let Token::Semicolon = self.token_iter.peep().unwrap_or(Token::Eof) {
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
                    if let Token::Semicolon = self.token_iter.peep().unwrap_or(Token::Eof) {
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
                    if let Token::RightParen = self.token_iter.peep().unwrap_or(Token::Eof) {
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
            Token::Signed
            | Token::Unsigned
            | Token::Short
            | Token::Long
            | Token::Void
            | Token::Char
            | Token::Int => {
                // typename keywords
                let declaration = self.declaration().expect("invalid declaration");
                if !self.token_iter.next().unwrap_or(Token::Eof).is_semicolon() {
                    panic!("missing ';' after declaration statement");
                }
                declaration
            }
            Token::Identity(_) => {
                // Identity -> declaration or expression
                match self.declaration() {
                    None => {
                        let node = self.expression();
                        if let Token::Semicolon = self.token_iter.next().unwrap_or(Token::Eof) {
                            node
                        } else {
                            panic!("missing ';' after an expression statement");
                        }
                    }
                    Some(declaration) => {
                        if !self.token_iter.next().unwrap_or(Token::Eof).is_semicolon() {
                            panic!("missing ';' after declaration statement");
                        }
                        declaration
                    }
                }
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
