use crate::token::{LiteralValue, Token};

use super::expr::Expr;

pub enum Stmt {
    Expresssion(ExprStmt),
    Print(ExprStmt),
    Var(VarStmt),
    Block(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt)
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Expr,
}

pub struct ExprStmt {
    pub expr: Expr,
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>
}

pub enum ForStmtInitializer {
    VarDecl(VarStmt),
    ExprStmt(ExprStmt)
}

pub struct ForStmt {
    pub initializer: Option<ForStmtInitializer>,
    pub condition: Option<Expr>,
    pub afterthought: Option<Expr>,
    pub body: Box<Stmt>
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

    pub fn block(statements: Vec<Stmt>) -> Self {
        Stmt::Block(BlockStmt { statements })
    }

    pub fn if_stmt(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Stmt::IfStmt(IfStmt {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|v| Box::new(v)),
        })
    }

    pub fn while_stmt(condition: Expr, body: Stmt) -> Self {
        Stmt::WhileStmt(WhileStmt {
            condition,
            body: Box::new(body)
        })
    }

    pub fn for_stmt(body:Stmt, initializer: Option<ForStmtInitializer>, condition: Option<Expr>, afterthought: Option<Expr>) -> Self {
        Stmt::ForStmt(ForStmt {
            initializer,
            condition,
            afterthought,
            body: Box::new(body)
        })
    }
}
