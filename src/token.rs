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
    Void,
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
        Token::Identity(name) => format!("Identity [{}], ", name.clone()),
        Token::Signed => "Signed, ".to_string(),
        Token::Unsigned => "Unsigned, ".to_string(),
        Token::Short => "Short, ".to_string(),
        Token::Long => "Long, ".to_string(),
        Token::Void => "Void, ".to_string(),
        Token::Char => "Char, ".to_string(),
        Token::Int => "Int, ".to_string(),
        Token::Return => "Return, ".to_string(),
        Token::If => "If, ".to_string(),
        Token::Else => "Else, ".to_string(),
        Token::For => "For, ".to_string(),
        Token::While => "While, ".to_string(),
        Token::Eof => "EOF".to_string(),
    }
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
    pub fn is_leftcurl(&self) -> bool {
        match self {
            Token::LeftCurl => true,
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
