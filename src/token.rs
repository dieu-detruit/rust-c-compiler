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
    Boolean,
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
    use Token::*;
    match token {
        Num(n) => format!("Num: {}, ", n),
        Plus => "Mark +, ".to_string(),
        Minus => "Mark -, ".to_string(),
        Asterisk => "Mark *, ".to_string(),
        Slash => "Mark /, ".to_string(),
        LeftParen => "Mark (, ".to_string(),
        RightParen => "Mark ), ".to_string(),
        Lt => "Mark <, ".to_string(),
        Gt => "Mark >, ".to_string(),
        Equal => "Mark =, ".to_string(),
        Exclamation => "Mark !, ".to_string(),
        Semicolon => "Mark ;, ".to_string(),
        LeftCurl => "Mark {, ".to_string(),
        RightCurl => "Mark }, ".to_string(),
        Comma => "Mark \",\", ".to_string(),
        Identity(name) => format!("Identity [{}], ", name.clone()),
        Signed => "Signed, ".to_string(),
        Unsigned => "Unsigned, ".to_string(),
        Short => "Short, ".to_string(),
        Long => "Long, ".to_string(),
        Void => "Void, ".to_string(),
        Boolean => "Boolean, ".to_string(),
        Char => "Char, ".to_string(),
        Int => "Int, ".to_string(),
        Return => "Return, ".to_string(),
        If => "If, ".to_string(),
        Else => "Else, ".to_string(),
        For => "For, ".to_string(),
        While => "While, ".to_string(),
        Eof => "EOF".to_string(),
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
