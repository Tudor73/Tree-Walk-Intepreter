use crate::scanner::token::{LiteralType, Token};
use std::vec;

use super::expression::{Binary, Expr, Grouping, Literal, Unary};
use crate::scanner::token::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser { tokens, current: 0 };
    }

    fn expression(&mut self) -> Expr {
        return self.comparison();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        let types: Vec<TokenType> = std::vec![TokenType::EQUAL_EQUAL, TokenType::BANG_EQUAL];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.comparison();
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let types: Vec<TokenType> = vec![
            TokenType::LESS,
            TokenType::LESS_EQUAL,
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
        ];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.term();
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return expr;
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let types: Vec<TokenType> = vec![TokenType::MINUS, TokenType::PLUS];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.factor();
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let types: Vec<TokenType> = vec![TokenType::STAR, TokenType::SLASH];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.unary();
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        let types: Vec<TokenType> = vec![TokenType::BANG, TokenType::MINUS];
        if self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.unary();
            return Expr::Unary(Unary {
                operator: operator.clone(),
                right: Box::new(right),
            });
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(TokenType::FALSE) {
            return Expr::Literal(Literal {
                value: LiteralType::Bool(false),
            });
        }
        if self.match_token(TokenType::TRUE) {
            return Expr::Literal(Literal {
                value: LiteralType::Bool(true),
            });
        }

        if self.match_token(TokenType::NIL) {
            return Expr::Literal(Literal {
                value: LiteralType::Null(String::from("null")),
            });
        }
        if self.match_token(TokenType::NUMBER) {
            return Expr::Literal(Literal {
                value: Parser::previous(self.tokens.clone(), self.current).literal,
            });
        }
        if self.match_token(TokenType::STRING) {
            return Expr::Literal(Literal {
                value: Parser::previous(self.tokens.clone(), self.current).literal,
            });
        }
        if self.match_token(TokenType::LEFT_PAREN) {
            let expr = self.expression();
            self.consume(
                TokenType::RIGHT_PAREN,
                String::from("Expect ')' after expression"),
            )
            .unwrap();
            return Expr::Grouping(Grouping {
                expression: Box::new(expr),
            });
        } else {
            panic!("Invalid expression")
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, String> {
        if (self.check(token_type)) {
            return Ok(self.advance());
        }
        return Err(message);
    }

    fn match_tokens(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type.clone()) {
            self.advance();
            return true;
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return Parser::previous(self.tokens.clone(), self.current);
    }
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }
    fn previous(tokens: Vec<Token>, current: usize) -> Token {
        return tokens[current - 1].clone();
    }
}
