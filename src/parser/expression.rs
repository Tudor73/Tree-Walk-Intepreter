use crate::scanner::token::{LiteralType, Token};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Grouping(Grouping),
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralType,
}
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<T, String>;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<T, String>;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<T, String>;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<T, String>;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, String> {
        match self {
            Expr::Binary(b) => b.accept(visitor),
            Expr::Unary(b) => b.accept(visitor),
            Expr::Literal(b) => b.accept(visitor),
            Expr::Grouping(b) => b.accept(visitor),
        }
    }
}

impl Binary {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, String> {
        return visitor.visit_binary_expr(self);
    }
}

impl Unary {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, String> {
        return visitor.visit_unary_expr(self);
    }
}
impl Literal {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, String> {
        return visitor.visit_literal_expr(self);
    }
}

impl Grouping {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, String> {
        return visitor.visit_grouping_expr(self);
    }
}

pub struct AstPrinter;
impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<String, String> {
        let mut expressions: Vec<&Expr> = vec![];
        expressions.push(&*expr.left);
        expressions.push(&*expr.right);
        return self.format_expressions(&expr.operator.lexeme, &expressions);
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<String, String> {
        return self.format_expression(&expr.operator.lexeme, &expr.right);
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<String, String> {
        match &expr.value {
            LiteralType::String(s) => return Ok(s.clone()),
            LiteralType::Float(f) => return Ok(f.to_string()),
            LiteralType::Bool(b) => return Ok(b.to_string()),
            LiteralType::Null(n) => return Ok(n.clone()),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<String, String> {
        return self.format_expression(&String::from("grouping"), &expr.expression);
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> Result<String, String> {
        return expr.accept(self);
    }
    fn format_expressions(&mut self, name: &String, exprs: &Vec<&Expr>) -> Result<String, String> {
        let mut result = String::new();
        result += "(";
        result += name.as_str();
        for expr in exprs.iter() {
            result += " ";
            result += expr.accept(self)?.as_str();
        }
        result += ")";
        return Ok(result);
    }
    fn format_expression(&mut self, name: &String, expr: &Expr) -> Result<String, String> {
        let mut result = String::new();
        result += "(";
        result += name.as_str();
        result += " ";
        result += expr.accept(self)?.as_str();
        result += ")";
        return Ok(result);
    }
}
