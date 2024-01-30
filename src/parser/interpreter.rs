use crate::scanner::token::{LiteralType, TokenType};

use super::expression::{self, Expr, ExprVisitor, Grouping, Literal};

#[derive(Debug, Clone)]
enum Object {
    Literal(LiteralType),
    Expression(Expr),
}
impl Object {
    fn get_number(&self) -> Result<f32, String> {
        match self {
            Object::Literal(l) => match l {
                LiteralType::Float(f) => return Ok(f.clone()),
                _ => return Err(String::from("Operand must be a number")),
            },
            _ => return Err(String::from("Operand must be a number 2")),
        }
    }
    fn get_string(&self) -> Result<String, String> {
        match self {
            Object::Literal(l) => match l {
                LiteralType::String(f) => return Ok(f.clone()),
                _ => return Err(String::from("Operand must be a string")),
            },
            _ => return Err(String::from("Operand must be a string 2")),
        }
    }
    fn get_literal(&self) -> Result<LiteralType, String> {
        match self {
            Object::Literal(l) => return Ok(l.clone()),
            _ => return Err(String::from("Expected operand found expression")),
        }
    }
}

pub struct Interpreter;
impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Object, String> {
        return Ok(Object::Literal(expr.value.clone()));
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Object, String> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&mut self, expr: &expression::Unary) -> Result<Object, String> {
        let right: Object = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => {
                let value = right.get_number()?;
                return Ok(Object::Literal(LiteralType::Float(-value)));
            }
            TokenType::BANG => return Ok(Object::Literal(Interpreter::is_truthful(right)?)),
            _ => return Err(String::from("unreachable ")),
        }
    }
    fn visit_binary_expr(&mut self, expr: &expression::Binary) -> Result<Object, String> {
        let left: Object = self.evaluate(&expr.left)?;
        let right: Object = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                return Ok(Object::Literal(LiteralType::Float(left_val - right_val)));
            }
            TokenType::SLASH => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                return Ok(Object::Literal(LiteralType::Float(left_val / right_val)));
            }
            TokenType::STAR => {
                let left_val = left.get_number()?;
                let right_val = right.get_number()?;
                return Ok(Object::Literal(LiteralType::Float(left_val * right_val)));
            }
            TokenType::PLUS => {
                let left_literal = left.get_literal()?;
                let right_literal = right.get_literal()?;

                if matches!(left_literal, LiteralType::Float(_))
                    && matches!(right_literal, LiteralType::Float(_))
                {
                    let left_val = left.get_number()?;
                    let right_val = right.get_number()?;
                    return Ok(Object::Literal(LiteralType::Float(left_val + right_val)));
                } else if matches!(left_literal, LiteralType::String(_))
                    && matches!(right_literal, LiteralType::String(_))
                {
                    let left_val = left.get_string()?;
                    let right_val = right.get_string()?;
                    return Ok(Object::Literal(LiteralType::String(
                        left_val + right_val.as_str(),
                    )));
                } else {
                    return Err(String::from("Invalid operand"));
                }
            }

            _ => return Err(String::from("unreachable ")),
        }
    }
}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Object, String> {
        expr.accept(self)
    }

    fn is_truthful(ob: Object) -> Result<LiteralType, String> {
        match ob {
            Object::Literal(literal) => match literal {
                LiteralType::Bool(b) => return Ok(literal),
                LiteralType::Null(_) => return Ok(LiteralType::Bool(false)),
                _ => return Ok(LiteralType::Bool(false)),
            },
            _ => return Err(String::from("only literal type")),
        }
    }
}
