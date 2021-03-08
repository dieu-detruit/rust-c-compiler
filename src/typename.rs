use crate::token::Token;

#[derive(Copy, Clone)]
pub enum StorageClass {
    Static,
}

#[derive(Copy, Clone)]
pub enum Mutability {
    Const,
    Mutable,
}

#[derive(Copy, Clone)]
pub enum SignedFlag {
    Signed,
    Unsigned,
}

#[derive(Copy, Clone)]
pub enum SizeModifier {
    Short,
    Long,
    LongLong,
}

#[derive(Copy, Clone)]
pub enum PrimitiveType {
    Char,
    Int,
}

#[derive(Clone)]
pub enum Typename {
    Void,
    Integer(SignedFlag, usize),
    UserDefined(String),
}

pub fn parse_userdefined_type(_token_list: Vec<Token>) -> Typename {
    Typename::UserDefined("Hoge".to_string())
}

pub fn parse_typename(token_list: Vec<Token>) -> Typename {
    let mut signed_flag: Option<SignedFlag> = None;
    let mut size_modifier: Option<SizeModifier> = None;
    let mut primitive_type: Option<PrimitiveType> = None;
    for token in token_list.iter() {
        match token {
            Token::Signed => {
                if signed_flag.replace(SignedFlag::Signed).is_some() {
                    // replaceして古い値がSomeなら重複
                    panic!("two ore more signed/unsigned keyword");
                }
            }
            Token::Unsigned => {
                if signed_flag.replace(SignedFlag::Unsigned).is_some() {
                    panic!("two or more signed/unsigned keyword");
                }
            }
            Token::Short => {
                if size_modifier.replace(SizeModifier::Short).is_some() {
                    panic!("multiple 'short'/'long' keywords in a declartion");
                }
            }
            Token::Long => match size_modifier {
                None => {
                    size_modifier.replace(SizeModifier::Long);
                }
                Some(length) => match length {
                    SizeModifier::Short => panic!("both long and short declartion"),
                    SizeModifier::Long => {
                        size_modifier.replace(SizeModifier::LongLong);
                    }
                    SizeModifier::LongLong => panic!("'long long long' is too long"),
                },
            },
            Token::Char => {
                if primitive_type.replace(PrimitiveType::Char).is_some() {
                    panic!("two or more data types in a declaration");
                }
            }
            Token::Int => {
                if primitive_type.replace(PrimitiveType::Int).is_some() {
                    panic!("two or more data types in a declaration");
                }
            }
            Token::Void => {
                if signed_flag.is_some() {
                    panic!("void type cannot be modified with 'signed' or 'unsigned'");
                }
                if size_modifier.is_some() {
                    panic!("void type cannot be modified with 'short' or 'long'");
                }
                if token_list.len() > 1 {
                    panic!("void type cannot be modified with another keyword");
                }
                return Typename::Void;
            }
            _ => {}
        };
    }
    if signed_flag.is_some() && primitive_type.is_none() {
        primitive_type.replace(PrimitiveType::Int);
    }
    match primitive_type {
        None => parse_userdefined_type(token_list),
        Some(int_type) => Typename::Integer(
            match signed_flag {
                None => SignedFlag::Signed,
                Some(specified) => specified,
            },
            match int_type {
                PrimitiveType::Int => match size_modifier {
                    None => 4,
                    Some(size) => match size {
                        SizeModifier::Short => 2,
                        SizeModifier::Long => 4,
                        SizeModifier::LongLong => 8,
                    },
                },
                PrimitiveType::Char => {
                    if size_modifier.is_some() {
                        panic!("cannot modify char type with 'long' or 'short'");
                    }
                    1
                }
            },
        ),
    }
}

pub fn sprint_typename(typename: &Typename) -> String {
    match typename {
        Typename::Void => "void".to_string(),
        Typename::Integer(flag, size) => {
            if let SignedFlag::Signed = flag {
                "Signed Integer(size: "
            } else {
                "Unsigned Integer(size: "
            }
            .to_string()
                + size.to_string().as_str()
                + ")"
        }
        Typename::UserDefined(name) => name.to_string(),
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;
    use crate::typename::{parse_typename, sprint_typename};

    #[test]
    fn parse_test() {
        //let test_type = vec![Token::Unsigned, Token::Int, Token::Long];
        //let test_type = vec![Token::Unsigned, Token::Short];
        //let test_type = vec![Token::Char, Token::Short];
        let test_type = vec![Token::Void];
        let typename = parse_typename(test_type);
        panic!("typename: {}", sprint_typename(&typename));
    }
}
