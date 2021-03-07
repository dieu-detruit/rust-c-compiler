use crate::token::Token;

pub enum StorageClass {
    Static,
}

pub enum Mutability {
    Const,
    Mutable,
}

pub enum SignedFlag {
    Signed,
    Unsigned,
}

pub enum SizeModifier {
    Short,
    Normal,
    Long,
    LongLong,
}

pub enum PrimitiveType {
    Char(SignedFlag),
    Int(SignedFlag, SizeModifier),
}

pub enum Type {
    Primitive(PrimitiveType),
    UserDefined(String),
}

pub fn parse_primitive_type(token_list: Vec<Token>) {
    let mut signedflag = SignedFlag::Signed;
    let mut sizeModifier = SizeModifier::Normal;
    for token in token_list.iter() {}
}
