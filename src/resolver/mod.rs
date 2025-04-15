use std::collections::HashMap;

use crate::{ast::{expr::*, stmt::*}, interpreter::Interpreter, token::Token};

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    pub interpreter: Interpreter
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Resolver {
            scopes: Vec::new(),
            interpreter
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() { return };
        let scope = self.scopes.last_mut();
        match scope {
            Some(s) => s.insert(name.lexme.clone(), false),
            None => todo!("error")
        };
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() { return };
        let scope = self.scopes.last_mut();
        match scope {
            Some(s) => s.insert(name.lexme.clone(), true),
            None => todo!("error")
        };
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexme) {
                self.interpreter.resolve(expr, self.scopes.len() - i - 1);
                return;
            }
        }
    }

    fn resolve_block_stmt(&mut self, stmt: &BlockStmt) {
        self.begin_scope();
        for s in &stmt.statements {
            self.resolve_stmt(s);
        }
        self.end_scope();
    }

    fn resolve_var_stmt(&mut self, stmt: &VarStmt) {
        self.declare(&stmt.name);
        self.resolve_expr(&stmt.initializer);
        self.define(&stmt.name);
    }


    fn resolve_fun_stmt(&mut self, stmt: &FnStmt) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.begin_scope();
        for param in &stmt.params {
            self.declare(param);
            self.define(param);
        }
        //resolve(&fun.body);
        self.end_scope();
    }

    fn resolve_expr_stmt(&mut self, stmt: &ExprStmt) {
        self.resolve_expr(&stmt.expr);
    }

    fn resolve_if_stmt(&mut self, stmt: &IfStmt) {
        self.resolve_expr(&stmt.condition);
        if let Some(t) = &stmt.else_branch {
            self.resolve_stmt(t.as_ref());
        }
        if let Some(s) = &stmt.else_branch {
            self.resolve_stmt(s.as_ref());
        }
    }

    fn resolve_return_stmt(&mut self, stmt: &ReturnStmt) {
        if let Some(v) = &stmt.value {
            self.resolve_expr(v);
        }
    }

    fn resolve_while_stmt(&mut self, stmt: &WhileStmt) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(stmt.body.as_ref());
    }

    fn resolve_for_stmt(&mut self, stmt: &ForStmt) {
        if let Some(i) = &stmt.initializer {
            match i {
               ForStmtInitializer::VarDecl(v) => self.resolve_var_stmt(&v),
               ForStmtInitializer::ExprStmt(v) => self.resolve_expr_stmt(&v),
            }
        }
        if let Some(c) = &stmt.condition {
            self.resolve_expr(c);
        }
        self.resolve_stmt(stmt.body.as_ref());
    }

    fn resolve_variable_expr(&mut self, expr: &Variable) {
        let is_init = self.scopes.last().map(|s| s.get(&expr.name.lexme).unwrap_or(&false)).unwrap_or(&false);
        if !self.scopes.is_empty() && !is_init {
            todo!("error");
        }
        self.resolve_local(&Expr::Variable(expr.clone()), &expr.name);
    }

    fn resolve_assign_expr(&mut self, expr: &Assign) {
        self.resolve_expr(expr.value.as_ref());
        self.resolve_local(&Expr::Assign(expr.clone()), &expr.name);
    }

    fn resolve_binary_expr(&mut self, expr: &Binary) {
        self.resolve_expr(expr.right.as_ref());
        self.resolve_expr(expr.left.as_ref());
    }

    fn resolve_call_expr(&mut self, expr: &Call) {
        for arg in &expr.args {
            self.resolve_expr(arg);
        }
    }

    fn resolve_grouping_expr(&mut self, expr: &Grouping) {
        self.resolve_expr(expr.expression.as_ref());
    }

    fn resolve_logical_expr(&mut self, expr: &Logical) {
        self.resolve_expr(expr.right.as_ref());
        self.resolve_expr(expr.left.as_ref());
    }

    fn resolve_unary_expr(&mut self, expr: &Unary) {
        self.resolve_expr(expr.right.as_ref());
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Assign(a) => self.resolve_assign_expr(a),
            Expr::Variable(v) => self.resolve_variable_expr(v),
            Expr::Call(c) => self.resolve_call_expr(c),
            Expr::Unary(u) => self.resolve_unary_expr(u),
            Expr::Binary(b) => self.resolve_binary_expr(b),
            Expr::Logical(l) => self.resolve_logical_expr(l),
            Expr::Grouping(g) => self.resolve_grouping_expr(g),
            Expr::Literal(_) => (),
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Var(v) => self.resolve_var_stmt(v),
            Stmt::Print(p) => self.resolve_expr_stmt(p),
            Stmt::Block(b) => self.resolve_block_stmt(b),
            Stmt::IfStmt(i) => self.resolve_if_stmt(i),
            Stmt::ForStmt(f) => self.resolve_for_stmt(f),
            Stmt::Expresssion(e) => self.resolve_expr_stmt(e),
            _ => ()
        }
    }

    pub fn resolve(&mut self, stmts: &Vec<Stmt>) {
        for s in stmts {
            self.resolve_stmt(s);
        }
    }
}
