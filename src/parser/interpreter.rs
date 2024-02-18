use std::{env, os::macos::raw::stat, rc::Rc};

use crate::{
    report_error,
    scanner::token::{LiteralType, TokenType},
};

use super::{
    expression::{self, Expr, ExprVisitor, Grouping, Literal},
    statements::{self, Environment, ExpressionStmt, Stmt, StmtVisitor},
};

#[derive(Debug, Clone)]

pub struct RuntimeError {
    pub message: String,
    pub line: i32,
}

impl RuntimeError {
    pub fn error(line: i32, message: String) -> RuntimeError {
        let err = RuntimeError {
            message: message.clone(),
            line,
        };
        report_error(line, &message);
        return err;
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl ExprVisitor<LiteralType> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<LiteralType, RuntimeError> {
        return Ok(expr.value.clone());
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<LiteralType, RuntimeError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&mut self, expr: &expression::Unary) -> Result<LiteralType, RuntimeError> {
        let right: LiteralType = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => {
                let value = right.get_number(&expr.operator.line)?;
                return Ok(LiteralType::Float(-value));
            }
            TokenType::BANG => return Ok(LiteralType::Bool(!Interpreter::is_truthful(right))),
            _ => return Err(RuntimeError::error(0, String::from("unreachable "))),
        }
    }
    fn visit_binary_expr(
        &mut self,
        expr: &expression::Binary,
    ) -> Result<LiteralType, RuntimeError> {
        let left: LiteralType = self.evaluate(&expr.left)?;
        let right: LiteralType = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                return Ok(LiteralType::Float(left_val - right_val));
            }
            TokenType::SLASH => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                return Ok(LiteralType::Float(left_val / right_val));
            }
            TokenType::STAR => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                return Ok(LiteralType::Float(left_val * right_val));
            }
            TokenType::PLUS => {
                let result = left + right;
                match result {
                    Ok(t) => return Ok(t),
                    Err(s) => return Err(RuntimeError::error(expr.operator.line, s)),
                }
            }
            TokenType::GREATER => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                let comparison = if left_val > right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            TokenType::GREATER_EQUAL => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                let comparison = if left_val >= right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            TokenType::LESS => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                let comparison = if left_val < right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            TokenType::LESS_EQUAL => {
                let left_val = left.get_number(&expr.operator.line)?;
                let right_val = right.get_number(&expr.operator.line)?;
                let comparison = if left_val <= right_val { true } else { false };
                return Ok(LiteralType::Bool(comparison));
            }
            // THIS MAY NOT WORK
            TokenType::BANG_EQUAL => return Ok(LiteralType::Bool(left != right)),
            TokenType::EQUAL_EQUAL => return Ok(LiteralType::Bool(left == right)),

            _ => return Err(RuntimeError::error(0, String::from("unreachable "))),
        }
    }
    fn visit_variable_expr(
        &mut self,
        expr: &expression::Variable,
    ) -> Result<LiteralType, RuntimeError> {
        self.environment.get(expr.name.clone())
    }
    fn visit_assign_expr(
        &mut self,
        expr: &expression::Assign,
    ) -> Result<LiteralType, RuntimeError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), RuntimeError> {
        self.evaluate(&stmt.expression)?;
        return Ok(());
    }
    fn visit_print_statment(&mut self, stmt: &statements::PrintStmt) -> Result<(), RuntimeError> {
        let value = Interpreter::stringify(self.evaluate(&stmt.expression)?);
        println!("{}", value);
        return Ok(());
    }

    fn visit_var_statement(&mut self, stmt: &statements::Var) -> Result<(), RuntimeError> {
        let mut value = LiteralType::Null;
        if let Some(e) = stmt.initializer.clone() {
            value = self.evaluate(&e)?;
        }
        Ok(self.environment.define(&stmt.name.lexeme, value))
    }
    fn visit_block_stmt(&mut self, stmt: &statements::Block) -> Result<(), RuntimeError> {
        self.execute_block(
            stmt.statements.clone(),
            Environment::new_with_enclosing(self.environment.clone()),
        )
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {
            environment: Environment::new(),
        };
    }
    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements.iter() {
            match stmt {
                Stmt::Expression(e) => e.accept(self)?,
                Stmt::Print(e) => e.accept(self)?,
                Stmt::Var(v) => v.accept(self)?,
                Stmt::Block(v) => v.accept(self)?,
            };
        }
        return Ok(());
    }

    fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            statement.accept(self)?;
        }
        self.environment = previous.clone();
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralType, RuntimeError> {
        expr.accept(self)
    }

    fn is_truthful(literal: LiteralType) -> bool {
        match literal {
            LiteralType::Bool(b) => return b,
            LiteralType::Null => return false,
            _ => return true,
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
