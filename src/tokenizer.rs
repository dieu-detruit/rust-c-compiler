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
    LeftCurl,
    RightCurl,
    Comma,
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

#[derive(Clone)]
pub struct TokenIter {
    s: String,
}

pub fn tokenize(s: String) -> TokenIter {
    TokenIter { s: s.clone() }
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
    pub fn is_rightcurl(&self) -> bool {
        match self {
            Token::RightCurl => true,
            _ => false,
        }
    }
    pub fn is_comma(&self) -> bool {
        match self {
            Token::Comma => true,
            _ => false,
        }
    }
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, remain_s) = self.tokenize_str(&self.s);
        self.s = remain_s;
        return token;
    }
}

impl TokenIter {
    fn tokenize_str(&self, input: &String) -> (Option<Token>, String) {
        let mut s = input.clone();

        // 空白文字を飛ばす
        s = s.trim_start().to_string();
        // 終わりならNone
        if s.is_empty() {
            return (None, s);
        }

        let first_byte = s.as_bytes()[0];
        return match first_byte {
            b'+' => self.tokenize_byte(s, Token::Plus),
            b'-' => self.tokenize_byte(s, Token::Minus),
            b'*' => self.tokenize_byte(s, Token::Asterisk),
            b'/' => self.tokenize_byte(s, Token::Slash),
            b'(' => self.tokenize_byte(s, Token::LeftParen),
            b')' => self.tokenize_byte(s, Token::RightParen),
            b'<' => self.tokenize_byte(s, Token::Lt),
            b'>' => self.tokenize_byte(s, Token::Gt),
            b'=' => self.tokenize_byte(s, Token::Equal),
            b'!' => self.tokenize_byte(s, Token::Exclamation),
            b';' => self.tokenize_byte(s, Token::Semicolon),
            b'{' => self.tokenize_byte(s, Token::LeftCurl),
            b'}' => self.tokenize_byte(s, Token::RightCurl),
            b',' => self.tokenize_byte(s, Token::Comma),
            b'0'..=b'9' => {
                let (digit_s, remain_s) = split_digit(s);
                s = remain_s;
                (
                    Some(Token::Num(i32::from_str_radix(&digit_s, 10).unwrap())),
                    s,
                )
            }
            b'a'..=b'z' | b'_' => {
                let (ident_s, remain_s) = split_identity(s);
                (
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
                        Some(Token::Identity(ident_s))
                    },
                    remain_s,
                )
            }
            _ => (None, s),
        };
    }

    pub fn tokenize_byte(&self, input: String, token: Token) -> (Option<Token>, String) {
        (Some(token), input.split_at(1).1.to_string())
    }

    // イテレータを進めずに次を覗き見る
    // Peekableは拘束しないといけないので面倒
    pub fn peep() {}

    // skipで型が変わるのが面倒なので実装
    // TODO: advance_by()がstableになったら置き換える
    pub fn ignore(&mut self, n: usize) {
        for _i in 0..n {
            let (tokenize_result, remain_s) = self.tokenize_str(&self.s);
            if let None = tokenize_result {
                return;
            }
            self.s = remain_s;
        }
    }
}

pub fn split_digit(s: String) -> (String, String) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    let (former, latter) = s.split_at(first_non_num_idx);
    (former.to_string(), latter.to_string())
}

pub fn split_identity(s: String) -> (String, String) {
    let first_non_ident_idx = s
        .find(|c: char| !char::is_alphabetic(c) && !char::is_numeric(c) && c != '_')
        .unwrap_or(s.len());
    let (former, latter) = s.split_at(first_non_ident_idx);
    (former.to_string(), latter.to_string())
}

pub fn sprint_token(token: &Token) -> String {
    return match token {
        Token::Num(n) => format!("Num: {}, ", n),
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
        Token::LeftCurl => String::from("Mark {, "),
        Token::RightCurl => String::from("Mark }, "),
        Token::Comma => String::from("Mark \",\", "),
        Token::Identity(name) => format!("Var [{}], ", name.clone()),
        Token::Return => String::from("Return, "),
        Token::If => String::from("If, "),
        Token::Else => String::from("Else, "),
        Token::For => String::from("For, "),
        Token::While => String::from("While, "),
        Token::Eof => String::from("EOF"),
    };
}

pub fn sprint_token_iter(token_iter: TokenIter) -> String {
    let mut output = String::from("debug: ");
    for token in token_iter {
        output.push_str(sprint_token(&token).as_str());
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
