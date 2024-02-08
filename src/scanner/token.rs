use lazy_static::lazy_static;
use std::{cmp::Ordering, collections::HashMap, fmt, ops::Add};

#[derive(Debug, Clone)]
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
#[derive(Clone)]
pub enum LiteralType {
    String(String),
    Float(f32),
    Bool(bool),
    Null,
}

impl Add for LiteralType {
    type Output = Result<LiteralType, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LiteralType::Float(x), LiteralType::Float(y)) => return Ok(LiteralType::Float(x + y)),
            (LiteralType::String(x), LiteralType::String(y)) => {
                return Ok(LiteralType::String(format!("{}{}", x, y)))
            }
            _ => return Err(String::from("Operands must be two numbers or two strings")),
        }
    }
}

impl PartialEq for LiteralType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralType::String(x), LiteralType::String(y)) => x == y,
            (LiteralType::Float(x), LiteralType::Float(y)) => x == y,
            (LiteralType::Bool(x), LiteralType::Bool(y)) => x == y,
            (LiteralType::Null, LiteralType::Null) => return true,
            _ => false,
        }
    }
}

impl PartialOrd for LiteralType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LiteralType::Null, o) => {
                if o != &LiteralType::Null {
                    return None;
                } else {
                    return Some(Ordering::Equal);
                }
            }
            (LiteralType::Float(x), LiteralType::Float(y)) => return x.partial_cmp(y),
            (LiteralType::String(x), LiteralType::String(y)) => return x.partial_cmp(y),
            (LiteralType::Bool(x), LiteralType::Bool(y)) => return x.partial_cmp(y),
            _ => return None,
        }
    }
}

impl fmt::Debug for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{:?}", s),
            LiteralType::Float(d) => write!(f, "{}", d),
            LiteralType::Bool(b) => write!(f, "{}", b),
            LiteralType::Null => write!(f, "{:?}", "null"),
        }
    }
}

impl LiteralType {
    pub fn get_number(&self) -> Result<f32, String> {
        match self {
            LiteralType::Float(f) => return Ok(f.clone()),
            _ => return Err(String::from("Operand must be a number")),
        }
    }

    pub fn get_string(&self) -> Result<String, String> {
        match self {
            LiteralType::String(f) => return Ok(f.clone()),
            _ => return Err(String::from("Operand must be a number")),
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
