use std::vec;

use crate::{
    parser::expression::Variable,
    report_error,
    scanner::token::{LiteralType, Token},
};

use super::{
    expression::{Assign, Binary, Expr, Grouping, Literal, Unary},
    interpreter::RuntimeError,
    statements::{Block, ExpressionStmt, If, PrintStmt, Stmt, Var},
};
use crate::scanner::token::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser { tokens, current: 0 };
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, RuntimeError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    pub fn declaration(&mut self) -> Result<Stmt, RuntimeError> {
        // SYNCHRONIZE SHOULD GO HERE PROBABLY (inside match statements for these 2 functions)
        if self.match_token(TokenType::VAR) {
            return self.var_declaration();
        }
        self.statement()
    }

    pub fn var_declaration(&mut self) -> Result<Stmt, RuntimeError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.".to_string())?;
        let mut initializer = None;
        if self.match_token(TokenType::EQUAL) {
            initializer = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.".to_string(),
        )?;
        return Ok(Stmt::Var(Var { name, initializer }));
    }

    fn assignment(&mut self) -> Result<Expr, RuntimeError> {
        let expr = self.equality()?;

        if self.match_token(TokenType::EQUAL) {
            let equals = Parser::previous(self.tokens.clone(), self.current);
            let value = self.assignment()?;
            if let Expr::Variable(v) = expr {
                return Ok(Expr::Assign(Assign {
                    name: v.name,
                    value: Box::new(value),
                }));
            }
            return Err(RuntimeError::error(
                equals.line,
                "Invalid assignment target. ".to_string(),
            ));
        }
        return Ok(expr);
    }

    fn statement(&mut self) -> Result<Stmt, RuntimeError> {
        if self.match_token(TokenType::IF) {
            return self.if_statement();
        }
        if self.match_token(TokenType::PRINT) {
            return self.print_statement();
        }
        if self.match_token(TokenType::LEFT_BRACE) {
            return Ok(Stmt::Block(Block {
                statements: self.block()?,
            }));
        }
        return self.expression_statement();
    }
    fn if_statement(&mut self) -> Result<Stmt, RuntimeError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after if. ".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if. ".to_string())?;
        let then_branch = self.statement()?;
        if self.match_token(TokenType::ELSE) {
            return Ok(Stmt::If(If {
                condition: condition,
                then_branch: Box::new(then_branch),
                else_branch: Some(Box::new(self.statement()?)),
            }));
        } else {
            return Ok(Stmt::If(If {
                condition: condition,
                then_branch: Box::new(then_branch),
                else_branch: None,
            }));
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, RuntimeError> {
        let mut statments: Vec<Stmt> = vec![];

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statments.push(self.declaration()?);
        }
        self.consume(
            TokenType::RIGHT_BRACE,
            "Expect '} at the end of block'".to_string(),
        );
        Ok(statments)
    }

    fn print_statement(&mut self) -> Result<Stmt, RuntimeError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value. ".to_string())?;
        return Ok(Stmt::Print(PrintStmt { expression: value }));
    }

    fn expression_statement(&mut self) -> Result<Stmt, RuntimeError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value. ".to_string())?;
        return Ok(Stmt::Expression(ExpressionStmt { expression: value }));
    }

    fn expression(&mut self) -> Result<Expr, RuntimeError> {
        self.assignment()
    }

    fn equality(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.comparison()?;
        let types: Vec<TokenType> = std::vec![TokenType::EQUAL_EQUAL, TokenType::BANG_EQUAL];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.term()?;
        let types: Vec<TokenType> = vec![
            TokenType::LESS,
            TokenType::LESS_EQUAL,
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
        ];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.term()?;
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return Ok(expr);
    }
    fn term(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.factor()?;
        let types: Vec<TokenType> = vec![TokenType::MINUS, TokenType::PLUS];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, RuntimeError> {
        let mut expr = self.unary()?;
        let types: Vec<TokenType> = vec![TokenType::STAR, TokenType::SLASH];
        while self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Binary {
                operator: operator.clone(),
                left: Box::new(expr),
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, RuntimeError> {
        let types: Vec<TokenType> = vec![TokenType::BANG, TokenType::MINUS];
        if self.match_tokens(&types) {
            let operator = Parser::previous(self.tokens.clone(), self.current);
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator: operator.clone(),
                right: Box::new(right),
            }));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, RuntimeError> {
        println!("{:?}", self.tokens[self.current]);
        if self.match_token(TokenType::FALSE) {
            return Ok(Expr::Literal(Literal {
                value: LiteralType::Bool(false),
            }));
        }
        if self.match_token(TokenType::TRUE) {
            return Ok(Expr::Literal(Literal {
                value: LiteralType::Bool(true),
            }));
        }

        if self.match_token(TokenType::NIL) {
            return Ok(Expr::Literal(Literal {
                value: LiteralType::Null,
            }));
        }
        if self.match_token(TokenType::NUMBER) {
            return Ok(Expr::Literal(Literal {
                value: Parser::previous(self.tokens.clone(), self.current).literal,
            }));
        }
        if self.match_token(TokenType::STRING) {
            return Ok(Expr::Literal(Literal {
                value: Parser::previous(self.tokens.clone(), self.current).literal,
            }));
        }
        if self.match_token(TokenType::IDENTIFIER) {
            return Ok(Expr::Variable(Variable {
                name: Parser::previous(self.tokens.clone(), self.current),
            }));
        }
        if self.match_token(TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RIGHT_PAREN,
                String::from("Expect ')' after expression"),
            )?;
            return Ok(Expr::Grouping(Grouping {
                expression: Box::new(expr),
            }));
        } else {
            // NOTE: to specify the token need to do smth similar to the error function here
            // Parser::error(self.peek().clone(), &"Expect expression".to_string());
            return Err(RuntimeError::error(
                self.peek().clone().line,
                "Expect expression ".to_string(),
            ));
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, RuntimeError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        // Parser::error(self.peek().clone(), &message);
        return Err(RuntimeError::error(self.peek().clone().line, message));
    }

    fn error(token: Token, message: &String) {
        let mut error_string = String::new();
        if token.token_type == TokenType::EOF {
            error_string.push_str("at end ");
            error_string += message.as_str();
            report_error(token.line, &error_string);
        } else {
            error_string.push_str("at '");
            error_string.push_str(&token.lexeme);
            error_string.push_str("'");
            error_string += message.as_str();
            report_error(token.line, &error_string);
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if Parser::previous(self.tokens.clone(), self.current).token_type
                == TokenType::SEMICOLON
            {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }
            self.advance();
        }
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
