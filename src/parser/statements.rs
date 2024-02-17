use super::{expression::Expr, interpreter::RuntimeError};

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        match self {
            Stmt::Expression(e) => e.accept(visitor),
            Stmt::Print(e) => e.accept(visitor),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}
pub struct PrintStmt {
    pub expression: Expr,
}
pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, RuntimeError>;
    fn visit_print_statment(&self, expr: &PrintStmt) -> Result<T, RuntimeError>;
}

impl ExpressionStmt {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_expression_stmt(self);
    }
}

impl PrintStmt {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_print_statment(self);
    }
}
