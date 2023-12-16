use crate::report_error;

use super::token::{Token, TokenType};
pub struct Scanner {
    pub source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
    error: bool,
}

pub fn new(source: String) -> Scanner {
    Scanner {
        source,
        tokens: Vec::new(), // Initialize the tokens vector
        start: 0,
        current: 0,
        line: 1,
        error: false,
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: super::token::TokenType::EOF,
            lexeme: String::from(""),
            literal: String::from(""),
            line: self.line,
        });
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        self.current += 1;
        let c = self
            .source
            .chars()
            .nth(self.current - 1)
            .expect("index out of range");

        let token_type;
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                if self.match_char('=') {
                    token_type = TokenType::BANG_EQUAL;
                    self.current += 1
                } else {
                    token_type = TokenType::BANG
                };
                self.add_token(token_type, None);
            }
            '=' => {
                if self.match_char('=') {
                    token_type = TokenType::EQUAL_EQUAL;
                    self.current += 1
                } else {
                    token_type = TokenType::EQUAL
                };
                self.add_token(token_type, None);
            }
            '<' => {
                if self.match_char('=') {
                    token_type = TokenType::LESS_EQUAL;
                    self.current += 1
                } else {
                    token_type = TokenType::LESS;
                };
                self.add_token(token_type, None);
            }
            '>' => {
                if self.match_char('=') {
                    token_type = TokenType::GREATER_EQUAL;
                    self.current += 1
                } else {
                    token_type = TokenType::GREATER;
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.current += 1
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            '"' => self.string_literal(),
            _ => {
                let mut error_message = String::from("Unexpected character. ");
                error_message.push(c);
                report_error(self.line, error_message);
                self.error = true;
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        let value = literal.unwrap_or(String::from(""));
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: value,
            line: self.line,
        })
    }

    fn match_char(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self
            .source
            .chars()
            .nth(self.current)
            .expect("index out of range match")
            != expected
        {
            return false;
        }
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current)
            .expect("Index out of range");
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.is_at_end() {
            report_error(self.line, String::from("Unterminated string literal"));
            return;
        }
        self.current += 1;
        let value: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - self.start - 2)
            .collect();
        self.add_token(TokenType::STRING, Some(value));
    }
}
