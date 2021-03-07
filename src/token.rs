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
    // identity
    Identity(String),
    // reserved keyword
    Signed,
    Unsigned,
    Short,
    Long,
    Char,
    Int,
    Return,
    If,
    Else,
    For,
    While,
    // EOF
    Eof,
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
    pub fn is_identity(&self) -> bool {
        match self {
            Token::Identity(_) => true,
            _ => false,
        }
    }
    pub fn expect_identity(&self) -> String {
        match self {
            Token::Identity(name) => name.clone(),
            _ => panic!("Identity expected but another type of token found"),
        }
    }
}
