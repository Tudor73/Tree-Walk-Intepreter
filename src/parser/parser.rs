use crate::scanner::token::{LiteralType, Token};

// #[warn(dead_code)]
// impl Expr {
//     fn new(operator: Token) -> Expr {
//         return Expr {
//             operator,
//             left: None,
//             right: None,
//         };
//     }
// }
#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}
#[derive(Debug)]
pub enum ExprKind {
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralType,
    },
}
pub trait ExprVisitor {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String;
    fn visit_unary_expr(&mut self, expr: &Expr) -> String;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> String;
    fn visit_literal_expr(&mut self, expr: &Expr) -> String;
}

impl Expr {
    pub fn accept<V>(&self, visitor: &mut V) -> String
    where
        V: ExprVisitor,
    {
        match &self.kind {
            ExprKind::Binary {
                operator,
                left,
                right,
            } => visitor.visit_binary_expr(self),
            ExprKind::Unary { operator, right } => visitor.visit_unary_expr(self),
            ExprKind::Grouping { expression } => visitor.visit_grouping_expr(self),
            ExprKind::Literal { value } => visitor.visit_literal_expr(self),
        }
    }
}
struct AstPrinter;
impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        String::from("")
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        String::from("")
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        String::from("")
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        String::from("")
    }
}
impl AstPrinter {
    fn format_expression(&mut self, name: String, exprs: Vec<Expr>) {
        let mut result = String::new();
        result += "(";
        result += name.as_str();
        for expr in exprs.iter() {
            result += " ";
            result += expr.accept(self).as_str();
        }
    }
}
