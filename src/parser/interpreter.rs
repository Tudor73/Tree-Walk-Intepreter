use crate::scanner::token::{LiteralType, Token, TokenType};

use super::expression::{self, Expr, ExprVisitor, Grouping, Literal};

#[derive(Debug, Clone)]

pub struct RuntimeError {
    pub message: String,
    pub token: Token,
}

pub struct Interpreter {
    pub runtime_error: RuntimeError,
}

impl ExprVisitor<LiteralType> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<LiteralType, String> {
        return Ok(expr.value.clone());
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<LiteralType, String> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&mut self, expr: &expression::Unary) -> Result<LiteralType, String> {
        let right: LiteralType = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => {
                let value = right.get_number()?;
                return Ok(LiteralType::Float(-value));
            }
            TokenType::BANG => return Ok(Interpreter::is_truthful(right)?),
            _ => return Err(String::from("unreachable ")),
        }
    }
    fn visit_binary_expr(&mut self, expr: &expression::Binary) -> Result<LiteralType, String> {
        let left: LiteralType = self.evaluate(&expr.left)?;
        let right: LiteralType = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                return Ok(LiteralType::Float(left_val - right_val));
            }
            TokenType::SLASH => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                return Ok(LiteralType::Float(left_val / right_val));
            }
            TokenType::STAR => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                return Ok(LiteralType::Float(left_val * right_val));
            }
            TokenType::PLUS => {
                let result = left + right;
                return result;
            }
            TokenType::GREATER => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                let comparison = if left_val > right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            TokenType::GREATER_EQUAL => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                let comparison = if left_val >= right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            TokenType::LESS => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                let comparison = if left_val < right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            TokenType::LESS_EQUAL => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                let comparison = if left_val <= right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            // THIS MAY NOT WORK
            TokenType::BANG_EQUAL => return Ok(LiteralType::Bool(left != right)),
            TokenType::EQUAL_EQUAL => return Ok(LiteralType::Bool(left == right)),

            _ => return Err(String::from("unreachable ")),
        }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, expression: Expr) -> Result<String, String> {
        let value = self.evaluate(&expression);
        match value {
            Ok(e) => return Ok(Interpreter::stringify(e)),
            Err(e) => {}
        };
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralType, String> {
        expr.accept(self)
    }

    fn is_truthful(literal: LiteralType) -> Result<LiteralType, String> {
        match literal {
            LiteralType::Bool(_) => return Ok(literal),
            LiteralType::Null => return Ok(LiteralType::Bool(false)),
            _ => return Ok(LiteralType::Bool(false)),
        }
    }
    fn stringify(literal: LiteralType) -> String {
        match literal {
            LiteralType::Null => return String::from("nil"),
            LiteralType::Float(f) => return f.to_string(),
            LiteralType::Bool(b) => return b.to_string(),
            LiteralType::String(s) => return s,
        }
    }
}
