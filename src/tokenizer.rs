use crate::token::Token;

#[derive(Clone)]
pub struct TokenIter {
    s: String,
}

pub fn tokenize(s: String) -> TokenIter {
    TokenIter { s: s.clone() }
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
                let token = match &*ident_s {
                    "return" => Token::Return,
                    "signed" => Token::Signed,
                    "unsigned" => Token::Unsigned,
                    "short" => Token::Short,
                    "long" => Token::Long,
                    "char" => Token::Char,
                    "int" => Token::Int,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "while" => Token::While,
                    _ => Token::Identity(ident_s),
                };
                (Some(token), remain_s)
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
    match token {
        Token::Num(n) => format!("Num: {}, ", n),
        Token::Plus => "Mark +, ".to_string(),
        Token::Minus => "Mark -, ".to_string(),
        Token::Asterisk => "Mark *, ".to_string(),
        Token::Slash => "Mark /, ".to_string(),
        Token::LeftParen => "Mark (, ".to_string(),
        Token::RightParen => "Mark ), ".to_string(),
        Token::Lt => "Mark <, ".to_string(),
        Token::Gt => "Mark >, ".to_string(),
        Token::Equal => "Mark =, ".to_string(),
        Token::Exclamation => "Mark !, ".to_string(),
        Token::Semicolon => "Mark ;, ".to_string(),
        Token::LeftCurl => "Mark {, ".to_string(),
        Token::RightCurl => "Mark }, ".to_string(),
        Token::Comma => "Mark \",\", ".to_string(),
        Token::Identity(name) => format!("Var [{}], ", name.clone()),
        Token::Return => "Return, ".to_string(),
        Token::If => "If, ".to_string(),
        Token::Else => "Else, ".to_string(),
        Token::For => "For, ".to_string(),
        Token::While => "While, ".to_string(),
        Token::Eof => "EOF".to_string(),
        _ => String::new(),
    }
}

pub fn sprint_token_iter(token_iter: TokenIter) -> String {
    let mut output = "debug: ".to_string();
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
