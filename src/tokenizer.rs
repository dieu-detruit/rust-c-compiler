pub enum Token {
    // symbols
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Lt,
    Gt,
    Equal,
    Exclamation,
    Semicolon,
    // literal
    Num(i32),
    // variable
    Identity(String),
    // reserved keyword
    Return,
    If,
    Else,
    For,
    While,
    // EOF
    Eof,
}

#[derive(Copy, Clone)]
pub struct TokenIter<'a> {
    s: &'a str,
}

pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter { s }
}

impl Token {
    pub fn expect_num(&self) -> i32 {
        match self {
            Token::Num(n) => *n,
            _ => panic!("Invalid Code"),
        }
    }

    pub fn is_leftparen(&self) -> bool {
        match self {
            Token::LeftParen => true,
            _ => false,
        }
    }
    pub fn is_rightparen(&self) -> bool {
        match self {
            Token::RightParen => true,
            _ => false,
        }
    }
    pub fn is_semicolon(&self) -> bool {
        match self {
            Token::Semicolon => true,
            _ => false,
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // 空白文字を飛ばす
        self.s = self.s.trim_start();
        // 終わりならNone
        if self.s.is_empty() {
            return None;
        }

        let first_byte = self.s.as_bytes()[0];
        return match first_byte {
            b'+' => self.tokenize_byte(Token::Plus),
            b'-' => self.tokenize_byte(Token::Minus),
            b'*' => self.tokenize_byte(Token::Asterisk),
            b'/' => self.tokenize_byte(Token::Slash),
            b'(' => self.tokenize_byte(Token::LeftParen),
            b')' => self.tokenize_byte(Token::RightParen),
            b'<' => self.tokenize_byte(Token::Lt),
            b'>' => self.tokenize_byte(Token::Gt),
            b'=' => self.tokenize_byte(Token::Equal),
            b'!' => self.tokenize_byte(Token::Exclamation),
            b';' => self.tokenize_byte(Token::Semicolon),
            b'0'..=b'9' => {
                let (digit_s, remain_s) = split_digit(self.s);
                self.s = remain_s;
                Some(Token::Num(i32::from_str_radix(digit_s, 10).unwrap()))
            }
            b'a'..=b'z' | b'_' => {
                let (ident_s, remain_s) = split_identity(self.s);
                self.s = remain_s;
                if ident_s == "return" {
                    Some(Token::Return)
                } else if ident_s == "if" {
                    Some(Token::If)
                } else if ident_s == "else" {
                    Some(Token::Else)
                } else if ident_s == "for" {
                    Some(Token::For)
                } else if ident_s == "while" {
                    Some(Token::While)
                } else {
                    Some(Token::Identity(ident_s.to_string()))
                }
            }
            _ => None,
        };
    }
}

impl<'a> TokenIter<'a> {
    pub fn tokenize_byte(&mut self, token: Token) -> Option<Token> {
        self.s = self.s.split_at(1).1;
        Some(token)
    }

    // skipで型が変わるのが面倒なので実装
    pub fn ignore(&mut self, n: usize) {
        for _i in 0..n {
            self.s = self.s.trim_start();
            if self.s.is_empty() {
                return;
            }

            match self.s.as_bytes()[0] {
                b'+' | b'-' | b'*' | b'/' | b'(' | b')' | b'<' | b'>' | b'=' | b'!' | b';' => {
                    self.s = self.s.split_at(1).1;
                }
                b'0'..=b'9' => {
                    self.s = split_digit(self.s).1;
                }
                b'a'..=b'z' | b'_' => {
                    self.s = split_identity(self.s).1;
                }
                _ => {}
            };
        }
    }
}

pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

pub fn split_identity(s: &str) -> (&str, &str) {
    let first_non_ident_idx = s
        .find(|c: char| !char::is_alphabetic(c) && !char::is_numeric(c) && c != '_')
        .unwrap_or(s.len());
    s.split_at(first_non_ident_idx)
}

pub fn sprint_token(token: &Token) -> String {
    return match token {
        Token::Num(n) => String::from(format!("Num: {}, ", n)),
        Token::Plus => String::from("Mark +, "),
        Token::Minus => String::from("Mark -, "),
        Token::Asterisk => String::from("Mark *, "),
        Token::Slash => String::from("Mark /, "),
        Token::LeftParen => String::from("Mark (, "),
        Token::RightParen => String::from("Mark ), "),
        Token::Lt => String::from("Mark <, "),
        Token::Gt => String::from("Mark >, "),
        Token::Equal => String::from("Mark =, "),
        Token::Exclamation => String::from("Mark !, "),
        Token::Semicolon => String::from("Mark ;, "),
        Token::Identity(name) => format!("Var [{}], ", name.clone()),
        Token::Return => String::from("Return, "),
        Token::If => String::from("If, "),
        Token::Else => String::from("Else, "),
        Token::For => String::from("For, "),
        Token::While => String::from("While, "),
        Token::Eof => String::from("EOF"),
    };
}

pub fn sprint_token_iter<'a>(token_iter: TokenIter<'a>) -> String {
    let mut output = String::from("debug: ");
    for token in token_iter {
        output.push_str(&sprint_token(&token));
    }
    return output;
}

#[cfg(test)]
mod test {
    use crate::tokenizer::{sprint_token_iter, tokenize, Token};
    #[test]
    fn tokenize_test() {
        let prog = "1 + 2 + 3 + 4";
        let output = sprint_token_iter(tokenize(prog));

        panic!("{}", output);
    }
    #[test]
    fn clone_tokeniter_test() {
        let prog = "1 + 2 + 3 + 4";
        let mut token_iter = tokenize(prog);
        let token_iter_cp = token_iter.clone();

        let original = sprint_token_iter(token_iter);
        token_iter.next();
        let cloned = sprint_token_iter(token_iter_cp);

        panic!("\noriginal {0},\n cloned: {1}", original, cloned);
    }

    #[test]
    fn is_leftparen_test() {
        let token = Token::LeftParen;
        if !token.is_leftparen() {
            panic!("It's not a left paren");
        }
    }
}
