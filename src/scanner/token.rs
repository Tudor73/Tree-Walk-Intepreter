use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub line: i32,
}

#[derive(PartialEq)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}
pub enum LiteralType {
    String(String),
    Float(f32),
}
impl fmt::Debug for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{:?}", s),
            LiteralType::Float(d) => write!(f, "{}", d),
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", TokenType::AND);
        map.insert("class", TokenType::CLASS);
        map.insert("else", TokenType::ELSE);
        map.insert("false", TokenType::FALSE);
        map.insert("for", TokenType::FOR);
        map.insert("fun", TokenType::FUN);
        map.insert("if", TokenType::IF);
        map.insert("nil", TokenType::NIL);
        map.insert("or", TokenType::OR);
        map.insert("print", TokenType::PRINT);
        map.insert("return", TokenType::RETURN);
        map.insert("super", TokenType::SUPER);
        map.insert("this", TokenType::THIS);
        map.insert("true", TokenType::TRUE);
        map.insert("var", TokenType::VAR);
        map.insert("while", TokenType::WHILE);
        map
    };
}
