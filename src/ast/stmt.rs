use crate::token::{LiteralValue, Token};

use super::expr::Expr;

pub enum Stmt {
    Expresssion(ExprStmt),
    Print(ExprStmt),
    Var(VarStmt),
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Expr,
}

pub struct ExprStmt {
    pub expr: Expr,
}

impl Stmt {
    pub fn var(name: Token, initializer: Option<Expr>) -> Self {
        Stmt::Var(VarStmt {
            name,
            initializer: match initializer {
                Some(v) => v,
                None => Expr::literal(LiteralValue::Nil),
            },
        })
    }

    pub fn print(expr: Expr) -> Self {
        Stmt::Print(ExprStmt { expr })
    }

    pub fn expression(expr: Expr) -> Self {
        Stmt::Expresssion(ExprStmt { expr })
    }
}
