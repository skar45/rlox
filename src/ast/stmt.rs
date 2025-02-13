use crate::token::{LiteralValue, Token};

use super::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expresssion(ExprStmt),
    Print(ExprStmt),
    Var(VarStmt),
    Block(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    FnStmt(FnStmt),
    ReturnStmt(ReturnStmt)
}

#[derive(Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Expr,
}

#[derive(Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub enum ForStmtInitializer {
    VarDecl(VarStmt),
    ExprStmt(ExprStmt),
}

#[derive(Clone)]
pub struct ForStmt {
    pub initializer: Option<ForStmtInitializer>,
    pub condition: Option<Expr>,
    pub afterthought: Option<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub struct FnStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>
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
            body: Box::new(body),
        })
    }

    pub fn for_stmt(
        body: Stmt,
        initializer: Option<ForStmtInitializer>,
        condition: Option<Expr>,
        afterthought: Option<Expr>,
    ) -> Self {
        Stmt::ForStmt(ForStmt {
            initializer,
            condition,
            afterthought,
            body: Box::new(body),
        })
    }

    pub fn fn_stmt(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Stmt::FnStmt(FnStmt { name, params, body })
    }

    pub fn return_stmt(keyword: Token, value: Option<Expr>) -> Self {
        Stmt::ReturnStmt(ReturnStmt {
            keyword,
            value
        })
    }
}
