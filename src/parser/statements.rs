use std::collections::HashMap;

use crate::scanner::token::{LiteralType, Token};

use super::{expression::Expr, interpreter::RuntimeError};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(Var),
    Block(Block),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        match self {
            Stmt::Expression(e) => e.accept(visitor),
            Stmt::Print(e) => e.accept(visitor),
            Stmt::Var(v) => v.accept(visitor),
            Stmt::Block(v) => v.accept(visitor),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}
#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}
#[derive(Debug, Clone)]
pub struct Var {
    pub initializer: Option<Expr>,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<T, RuntimeError>;
    fn visit_print_statment(&mut self, stmt: &PrintStmt) -> Result<T, RuntimeError>;
    fn visit_var_statement(&mut self, stmt: &Var) -> Result<T, RuntimeError>;
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<T, RuntimeError>;
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
impl Block {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_block_stmt(self);
    }
}
#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, LiteralType>,
}

impl<'a> Environment {
    pub fn new() -> Environment {
        return Environment {
            enclosing: None,
            values: HashMap::new(),
        };
    }
    pub fn new_with_enclosing(enclosing: Environment) -> Environment {
        return Environment {
            enclosing: Some(Box::new(enclosing)),
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
                if let Some(e) = &self.enclosing {
                    return e.get(name);
                }
                return Err(RuntimeError::error(
                    name.line,
                    String::from("Undefined variable '".to_string() + &name.lexeme + "'."),
                ));
            }
            Some(v) => return Ok(v.clone()),
        }
    }
    pub fn assign(&mut self, name: Token, value: LiteralType) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        } else if let Some(e) = &mut self.enclosing {
            e.assign(name.clone(), value)?;
        }
        return Err(RuntimeError::error(
            name.line,
            String::from("Undefined variable '".to_string() + &name.lexeme + "'."),
        ));
    }
}
