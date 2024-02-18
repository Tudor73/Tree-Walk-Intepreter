use crate::scanner::token::{LiteralType, Token};

use super::{interpreter::RuntimeError, statements::Var};
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Grouping(Grouping),
    Variable(Variable),
    Assign(Assign),
}
#[derive(Debug, Clone)]
pub struct Binary {
    pub operator: Token,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
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

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<T, RuntimeError>;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<T, RuntimeError>;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<T, RuntimeError>;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<T, RuntimeError>;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<T, RuntimeError>;
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<T, RuntimeError>;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        match self {
            Expr::Binary(b) => b.accept(visitor),
            Expr::Unary(b) => b.accept(visitor),
            Expr::Literal(b) => b.accept(visitor),
            Expr::Grouping(b) => b.accept(visitor),
            Expr::Variable(v) => v.accept(visitor),
            Expr::Assign(v) => v.accept(visitor),
        }
    }
}

impl Binary {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_binary_expr(self);
    }
}

impl Unary {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_unary_expr(self);
    }
}
impl Literal {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_literal_expr(self);
    }
}

impl Grouping {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_grouping_expr(self);
    }
}

impl Variable {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_variable_expr(self);
    }
}

impl Assign {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> Result<T, RuntimeError> {
        return visitor.visit_assign_expr(self);
    }
}
// pub struct AstPrinter;
// impl ExprVisitor<String> for AstPrinter {
//     fn visit_binary_expr(&mut self, expr: &Binary) -> Result<String, RuntimeError> {
//         let mut expressions: Vec<&Expr> = vec![];
//         expressions.push(&*expr.left);
//         expressions.push(&*expr.right);
//         return self.format_expressions(&expr.operator.lexeme, &expressions);
//     }

//     fn visit_unary_expr(&mut self, expr: &Unary) -> Result<String, RuntimeError> {
//         return self.format_expression(&expr.operator.lexeme, &expr.right);
//     }

//     fn visit_literal_expr(&mut self, expr: &Literal) -> Result<String, RuntimeError> {
//         match &expr.value {
//             LiteralType::String(s) => return Ok(s.clone()),
//             LiteralType::Float(f) => return Ok(f.to_string()),
//             LiteralType::Bool(b) => return Ok(b.to_string()),
//             LiteralType::Null => return Ok(String::from("null")),
//         }
//     }

//     fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<String, RuntimeError> {
//         return self.format_expression(&String::from("grouping"), &expr.expression);
//     }
//     fn visit_variable_expr(&mut self, expr: &Variable) -> Result<String, RuntimeError> {
//         return self.format_expression(&String::from("variable"), &expr.name);
//     }
// }

// impl AstPrinter {
//     pub fn print(&mut self, expr: Expr) -> Result<String, RuntimeError> {
//         return expr.accept(self);
//     }
//     fn format_expressions(
//         &mut self,
//         name: &String,
//         exprs: &Vec<&Expr>,
//     ) -> Result<String, RuntimeError> {
//         let mut result = String::new();
//         result += "(";
//         result += name.as_str();
//         for expr in exprs.iter() {
//             result += " ";
//             result += expr.accept(self)?.as_str();
//         }
//         result += ")";
//         return Ok(result);
//     }
//     fn format_expression(&mut self, name: &String, expr: &Expr) -> Result<String, RuntimeError> {
//         let mut result = String::new();
//         result += "(";
//         result += name.as_str();
//         result += " ";
//         result += expr.accept(self)?.as_str();
//         result += ")";
//         return Ok(result);
//     }
// }
