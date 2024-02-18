use std::collections::HashMap;

use crate::scanner::token::{LiteralType, Token};

use super::{expression::Expr, interpreter::RuntimeError};

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(Var),
}

impl Stmt {
    fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        match self {
            Stmt::Expression(e) => e.accept(visitor),
            Stmt::Print(e) => e.accept(visitor),
            Stmt::Var(v) => v.accept(visitor),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}
pub struct PrintStmt {
    pub expression: Expr,
}
pub struct Var {
    pub initializer: Option<Expr>,
    pub name: Token,
}
pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<T, RuntimeError>;
    fn visit_print_statment(&mut self, stmt: &PrintStmt) -> Result<T, RuntimeError>;
    fn visit_var_statement(&mut self, stmt: &Var) -> Result<T, RuntimeError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_expression_stmt(self);
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_print_statment(self);
    }
}

impl Var {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_var_statement(self);
    }
}
pub struct Environment {
    pub values: HashMap<String, LiteralType>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            values: HashMap::new(),
        };
    }
    pub fn define(&mut self, name: &String, value: LiteralType) {
        self.values.insert(name.clone(), value);
        // this allows redefinition of variables which might be nice to remove later
    }
    pub fn get(&self, name: Token) -> Result<LiteralType, RuntimeError> {
        match self.values.get(&name.lexeme) {
            None => {
                return Err(RuntimeError::error(
                    name.line,
                    String::from("Undefined variable '".to_string() + &name.lexeme + "'."),
                ))
            }
            Some(v) => return Ok(v.clone()),
        }
    }
    pub fn assign(&mut self, name: Token, value: LiteralType) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        return Err(RuntimeError::error(
            name.line,
            String::from("Undefined variable '".to_string() + &name.lexeme + "'."),
        ));
    }
}
