use std::collections::HashMap;

use crate::node::{BinaryType, LVar, Node};
use crate::tokenizer::{sprint_token, sprint_token_iter, tokenize, Token, TokenIter};

/*
 * 生成文法
 *
 * program = statement*
 * statement    = expression ";"
 *              | "{" statement* "}"
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
 */

pub struct Parser {
    token_iter: TokenIter,
    local_vars: HashMap<String, LVar>,
    pub offset_last: usize,
}

impl Parser {
    pub fn program(&mut self) -> Node {
        let mut code: Vec<Node> = Vec::new();
        while self.token_iter.clone().next().is_some() {
            code.push(self.statement());
        }
        Node::Block(code)
    }

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

    pub fn expression(&mut self) -> Node {
        eprintln!("expression() called");
        self.assign()
    }

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

    pub fn equality(&mut self) -> Node {
        eprintln!("equality() called");
        let mut node = self.inequality();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Equal => {
                    // ==
                    if let Token::Equal = token_iter_cp.next().unwrap_or(Token::Eof) {
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((node, self.inequality())), BinaryType::Equal);
                    } else {
                        return node;
                    }
                }
                Token::Exclamation => {
                    // !=
                    if let Token::Equal = token_iter_cp.next().unwrap_or(Token::Eof) {
                        self.token_iter.ignore(2);
                        node =
                            Node::Binary(Box::new((node, self.inequality())), BinaryType::NotEqual);
                    } else {
                        return node;
                    }
                }
                _ => {
                    return node;
                }
            };
        }
    }

    pub fn inequality(&mut self) -> Node {
        eprintln!("inequality() called");
        let mut node = self.add();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Lt => match token_iter_cp.next().unwrap_or(Token::Eof) {
                    Token::Equal => {
                        // <=
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((node, self.add())), BinaryType::LtEq);
                    }
                    _ => {
                        // <
                        self.token_iter.ignore(1);
                        node = Node::Binary(Box::new((node, self.add())), BinaryType::Lt);
                    }
                },
                Token::Gt => match token_iter_cp.next().unwrap_or(Token::Eof) {
                    Token::Equal => {
                        // >=
                        self.token_iter.ignore(2);
                        node = Node::Binary(Box::new((self.add(), node)), BinaryType::LtEq);
                    }
                    _ => {
                        // >
                        self.token_iter.ignore(1);
                        node = Node::Binary(Box::new((self.add(), node)), BinaryType::Lt);
                    }
                },
                _ => {
                    return node;
                }
            };
        }
    }

    pub fn add(&mut self) -> Node {
        eprintln!("add() called");
        let mut node = self.mul();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Plus => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Add);
                }
                Token::Minus => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.mul())), BinaryType::Sub);
                }
                _ => {
                    return node;
                }
            };
        }
    }
    pub fn mul(&mut self) -> Node {
        eprintln!("mul() called");
        let mut node = self.unary();

        loop {
            let mut token_iter_cp = self.token_iter.clone();
            let token = token_iter_cp.next().unwrap_or(Token::Eof);
            eprintln!("current token: {}", sprint_token(&token));

            match token {
                Token::Asterisk => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Mul)
                }
                Token::Slash => {
                    self.token_iter.ignore(1);
                    node = Node::Binary(Box::new((node, self.unary())), BinaryType::Div)
                }
                _ => {
                    return node;
                }
            };
        }
    }
    pub fn unary(&mut self) -> Node {
        eprintln!("unary() called");

        let mut token_iter_cp = self.token_iter.clone();
        let token = token_iter_cp.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
            Token::Plus => {
                self.token_iter.ignore(1);
                self.primary()
            }
            Token::Minus => {
                self.token_iter.ignore(1);
                Node::Binary(Box::new((Node::Num(0), self.primary())), BinaryType::Sub)
            }
            _ => self.primary(),
        };
    }
    pub fn primary(&mut self) -> Node {
        eprintln!("primary() called");

        let token = self.token_iter.next().unwrap_or(Token::Eof);
        eprintln!("current token: {}", sprint_token(&token));

        return match token {
            Token::LeftParen => {
                // ( expression )
                let node_expression = self.expression();
                if let Token::RightParen = self.token_iter.next().unwrap() {
                    node_expression
                } else {
                    panic!("Invalid Input");
                }
            }
            Token::Identity(name) => {
                return if let Token::LeftParen = self.token_iter.clone().next().unwrap() {
                    // Function Call
                    self.token_iter.ignore(1);
                    let mut arg_list: Vec<Node> = Vec::new();

                    match self.token_iter.clone().next().unwrap() {
                        Token::RightParen => {
                            // Call func()
                            self.token_iter.ignore(1);
                            return Node::FunctionCall(name, arg_list);
                        }
                        _ => {
                            arg_list.push(self.expression());
                        }
                    }
                    loop {
                        match self.token_iter.next().unwrap() {
                            Token::RightParen => {
                                // Call func(args)
                                return Node::FunctionCall(name, arg_list);
                            }
                            Token::Comma => {
                                arg_list.push(self.expression());
                            }
                            _ => {
                                panic!("Invaid argument expression for function call {}", name);
                            }
                        }
                    }
                } else {
                    // Variable
                    match self.local_vars.get(&name) {
                        None => {
                            self.local_vars.insert(
                                name.clone(),
                                LVar {
                                    offset: self.offset_last,
                                },
                            );
                            self.offset_last = self.offset_last + 32;
                            Node::LVar(self.offset_last - 32)
                        }
                        Some(local_var) => Node::LVar(local_var.offset),
                    }
                };
            }
            Token::Num(n) => Node::Num(n),
            _ => {
                panic!("Invalid Input");
            }
        };
    }
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
