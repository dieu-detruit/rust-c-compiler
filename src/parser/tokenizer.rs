pub enum Token {
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Lt,
    Gt,
    Equal,
    NotEqual,
    Num(i32),
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

        if self.s.as_bytes()[0] == b'+' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Plus);
        }

        if self.s.as_bytes()[0] == b'-' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Minus);
        }

        if self.s.as_bytes()[0] == b'*' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Asterisk);
        }

        if self.s.as_bytes()[0] == b'/' {
            self.s = self.s.split_at(1).1;
            return Some(Token::Slash);
        }

        if self.s.as_bytes()[0] == b'(' {
            self.s = self.s.split_at(1).1;
            return Some(Token::LeftParen);
        }

        if self.s.as_bytes()[0] == b')' {
            self.s = self.s.split_at(1).1;
            return Some(Token::RightParen);
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Num(i32::from_str_radix(digit_s, 10).unwrap()));
        }

        panic!("Invalid token stream")
    }
}

pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
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
        Token::Eof => String::from("EOF"),
        _ => String::from("Error"),
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
    use crate::parser::tokenizer::{sprint_token_iter, tokenize, Token};
    #[test]
    fn tokenize_test() {
        let prog = "1 + 2 + 3 + 4";
        let output = sprint_token_iter(tokenize(prog));

        //panic!("{}", output);
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
}
