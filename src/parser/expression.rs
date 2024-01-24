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
pub trait ExprVisitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String;
    fn visit_unary_expr(&mut self, expr: &Unary) -> String;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String;
    fn visit_literal_expr(&mut self, expr: &Literal) -> String;
}

impl Expr {
    pub fn accept<V>(&self, visitor: &mut V) -> String
    where
        V: ExprVisitor,
    {
        match self {
            Expr::Binary(b) => visitor.visit_binary_expr(b),
            Expr::Unary(b) => visitor.visit_unary_expr(b),
            Expr::Literal(b) => visitor.visit_literal_expr(b),
            Expr::Grouping(b) => visitor.visit_grouping_expr(b),
        }
    }
}
pub struct AstPrinter;
impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        let mut expressions: Vec<&Expr> = vec![];
        expressions.push(&*expr.left);
        expressions.push(&*expr.right);
        return self.format_expressions(&expr.operator.lexeme, &expressions);
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        return self.format_expression(&expr.operator.lexeme, &expr.right);
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        match &expr.value {
            LiteralType::String(s) => return s.clone(),
            LiteralType::Float(f) => return f.to_string(),
            LiteralType::Bool(b) => return b.to_string(),
            LiteralType::Null(n) => return n.clone(),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        return self.format_expression(&String::from("grouping"), &expr.expression);
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        return expr.accept(self);
    }
    fn format_expressions(&mut self, name: &String, exprs: &Vec<&Expr>) -> String {
        let mut result = String::new();
        result += "(";
        result += name.as_str();
        for expr in exprs.iter() {
            result += " ";
            result += expr.accept(self).as_str();
        }
        result += ")";
        return result;
    }
    fn format_expression(&mut self, name: &String, expr: &Expr) -> String {
        let mut result = String::new();
        result += "(";
        result += name.as_str();
        result += " ";
        result += expr.accept(self).as_str();
        result += ")";
        return result;
    }
}
