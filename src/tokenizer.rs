use crate::token::{sprint_token, Token};

#[derive(Clone)]
pub struct TokenIter {
    s: String,
}

pub fn tokenize(s: String) -> TokenIter {
    TokenIter { s: s.clone() }
}

fn tokenize_str(input: &String) -> (Option<Token>, String) {
    let mut s = input.clone();

    // 空白文字を飛ばす
    s = s.trim_start().to_string();
    // 終わりならNone
    if s.is_empty() {
        return (None, s);
    }

    let first_byte = s.as_bytes()[0];
    return match first_byte {
        b'+' => tokenize_byte(s, Token::Plus),
        b'-' => tokenize_byte(s, Token::Minus),
        b'*' => tokenize_byte(s, Token::Asterisk),
        b'/' => tokenize_byte(s, Token::Slash),
        b'(' => tokenize_byte(s, Token::LeftParen),
        b')' => tokenize_byte(s, Token::RightParen),
        b'<' => tokenize_byte(s, Token::Lt),
        b'>' => tokenize_byte(s, Token::Gt),
        b'=' => tokenize_byte(s, Token::Equal),
        b'!' => tokenize_byte(s, Token::Exclamation),
        b';' => tokenize_byte(s, Token::Semicolon),
        b'{' => tokenize_byte(s, Token::LeftCurl),
        b'}' => tokenize_byte(s, Token::RightCurl),
        b',' => tokenize_byte(s, Token::Comma),
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
                "void" => Token::Void,
                "_Bool" => Token::Void,
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

pub fn tokenize_byte(input: String, token: Token) -> (Option<Token>, String) {
    (Some(token), input.split_at(1).1.to_string())
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, remain_s) = tokenize_str(&self.s);
        self.s = remain_s;
        return token;
    }
}

impl TokenIter {
    // イテレータを進めずに次を覗き見る
    // Peekableは拘束しないといけないので面倒
    pub fn peep(&self) -> Option<Token> {
        let s_cp = self.s.clone();
        tokenize_str(&s_cp).0
    }

    // skipで型が変わるのが面倒なので実装
    // TODO: advance_by()がstableになったら置き換える
    pub fn ignore(&mut self, n: usize) {
        for _i in 0..n {
            let (tokenize_result, remain_s) = tokenize_str(&self.s);
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

pub fn sprint_token_iter(token_iter: TokenIter) -> String {
    let mut output = String::new();
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
        let prog = "1 + 2 + 3 + 4".to_string();
        let output = sprint_token_iter(tokenize(prog));

        panic!("{}", output);
    }

    #[test]
    fn is_leftparen_test() {
        let token = Token::LeftParen;
        if !token.is_leftparen() {
            panic!("It's not a left paren");
        }
    }
}
