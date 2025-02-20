use std::mem;

use crate::{
    ast::{
        expr::{Assign, Binary, Call, Expr, Grouping, Literal, Logical, Unary, Variable},
        stmt::{
            BlockStmt, ExprStmt, FnStmt, ForStmt, ForStmtInitializer, IfStmt, ReturnStmt, Stmt,
            VarStmt, WhileStmt,
        },
    },
    environment::{Environment, RcEnvironment},
    token::{LiteralValue, TokenType},
};

pub struct Interpreter {
    environment: RcEnvironment,
    global: RcEnvironment,
}

impl Interpreter {
    pub fn new(env: RcEnvironment) -> Self {
        Interpreter {
            environment: env.clone(),
            global: env,
        }
    }
    fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Bool(b) => *b,
            LiteralValue::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, v1: &LiteralValue, v2: &LiteralValue) -> bool {
        match (v1, v2) {
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            (LiteralValue::Nil, _) => false,
            (_, LiteralValue::Nil) => false,
            (LiteralValue::Num(n1), LiteralValue::Num(n2)) => n1 == n2,
            (LiteralValue::Str(s1), LiteralValue::Str(s2)) => s1 == s2,
            (LiteralValue::Bool(b1), LiteralValue::Bool(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn eval_literal(&self, expr: &Literal) -> LiteralValue {
        expr.value.clone()
    }

    fn eval_group(&mut self, expr: &Grouping) -> LiteralValue {
        self.evaluate(&expr.expression)
    }

    fn eval_unary(&mut self, expr: &Unary) -> LiteralValue {
        let right = self.evaluate(&expr.right);

        match expr.operator.r#type {
            TokenType::Minus => match right {
                LiteralValue::Num(n) => LiteralValue::Num(-n),
                _ => LiteralValue::Nil,
            },
            TokenType::Bang => LiteralValue::Bool(self.is_truthy(&right)),
            _ => LiteralValue::Nil,
        }
    }

    fn eval_binary(&mut self, expr: &Binary) -> LiteralValue {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        match expr.operator.r#type {
            TokenType::Minus => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Num(n1 - n2),
                _ => LiteralValue::Nil,
            },
            TokenType::Slash => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Num(n1 / n2),
                _ => LiteralValue::Nil,
            },
            TokenType::Star => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Num(n1 * n2),
                _ => LiteralValue::Nil,
            },
            TokenType::Plus => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Num(n1 + n2),
                (LiteralValue::Str(s1), LiteralValue::Str(s2)) => {
                    LiteralValue::Str(format!("{}{}", s1, s2))
                }
                _ => LiteralValue::Nil,
            },
            TokenType::Greater => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Bool(n1 > n2),
                _ => LiteralValue::Nil,
            },
            TokenType::GreaterEqual => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Bool(n1 >= n2),
                _ => LiteralValue::Nil,
            },
            TokenType::Less => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Bool(n1 < n2),
                _ => LiteralValue::Nil,
            },
            TokenType::LessEqual => match (left, right) {
                (LiteralValue::Num(n1), LiteralValue::Num(n2)) => LiteralValue::Bool(n1 <= n2),
                _ => LiteralValue::Nil,
            },
            TokenType::BangEqual => LiteralValue::Bool(!self.is_equal(&left, &right)),
            TokenType::EqualEqual => LiteralValue::Bool(self.is_equal(&left, &right)),
            _ => LiteralValue::Nil,
        }
    }

    fn eval_variable(&mut self, expr: &Variable) -> LiteralValue {
        match Environment::get_var(&mut self.environment, &expr.name.lexme) {
            Some(v) => v.clone(),
            None => LiteralValue::Nil,
        }
    }

    fn eval_assign(&mut self, expr: &Assign) -> LiteralValue {
        {
            let check = Environment::check(&mut self.environment, &expr.name.lexme);
            if !check {
                todo!("runtime error")
            }
        }
        let value = self.evaluate(&expr.value);
        if let Err(_) =
            Environment::assign_var(&mut self.environment, expr.name.lexme.clone(), value)
        {
            todo!("runtime error")
        }
        LiteralValue::Nil
    }

    fn eval_logical(&mut self, expr: &Logical) -> LiteralValue {
        match expr.operator.r#type {
            TokenType::Or => {
                let eval_left = self.evaluate(&expr.left);
                if self.is_truthy(&eval_left) {
                    eval_left
                } else {
                    self.evaluate(&expr.right)
                }
            }
            TokenType::And => {
                let eval_left = self.evaluate(&expr.left);
                if self.is_truthy(&eval_left) {
                    self.evaluate(&expr.right)
                } else {
                    LiteralValue::Bool(false)
                }
            }
            _ => todo!("interpreter error"),
        }
    }

    fn eval_call(&mut self, expr: &Call) -> LiteralValue {
        let mut args = Vec::new();
        let mut ret_val = LiteralValue::Nil;
        for arg in &expr.args {
            args.push(self.evaluate(&arg));
        }
        let rlox_fn = Environment::get_fn(&mut self.environment, &expr.callee);
        if let Some(fun) = rlox_fn {
            let mut env = Environment::new();
            Environment::add_enclosing(&mut env, self.environment);
            let prev = mem::replace(&mut self.environment, env);
            for (i, param) in fun.params.iter().enumerate() {
                Environment::define_var(
                    &mut self.environment,
                    param.lexme.clone(),
                    args[i].clone(),
                );
            }
            for stmt in &fun.body {
                match stmt {
                    Stmt::ReturnStmt(r) => {
                        ret_val = self.execute_return_stmt(&r);
                        break;
                    }
                    s => self.execute(&s),
                }
            }
            let mut d_env = mem::replace(&mut self.environment, prev);
            unsafe {
                let _ = mem::drop(Box::from_raw(d_env.as_mut()));
            }
        } else {
            todo!("interpreter error")
        }
        ret_val
    }

    fn evaluate(&mut self, expr: &Expr) -> LiteralValue {
        match expr {
            Expr::Literal(l) => self.eval_literal(l),
            Expr::Binary(b) => self.eval_binary(b),
            Expr::Unary(u) => self.eval_unary(u),
            Expr::Grouping(g) => self.eval_group(g),
            Expr::Variable(v) => self.eval_variable(v),
            Expr::Assign(a) => self.eval_assign(a),
            Expr::Logical(l) => self.eval_logical(l),
            Expr::Call(c) => self.eval_call(c),
        }
    }

    fn eval_expression_stmt(&mut self, stmt: &ExprStmt) {
        self.evaluate(&stmt.expr);
    }

    fn eval_print_stmt(&mut self, stmt: &ExprStmt) {
        let value = self.evaluate(&stmt.expr);
        println!("{}", value.to_string());
    }

    fn define_var_stmt(&mut self, stmt: &VarStmt) {
        let value = self.evaluate(&stmt.initializer);
        Environment::define_var(&mut self.environment, stmt.name.lexme.clone(), value);
    }

    fn execute_block(&mut self, stmt: &BlockStmt) {
        let mut new_env = Environment::new();
        Environment::add_enclosing(&mut new_env, self.environment);
        let prev = mem::replace(&mut self.environment, new_env);
        for s in &stmt.statements {
            self.execute(s);
        }
        let mut d_env = mem::replace(&mut self.environment, prev);
        unsafe {
            let _ = mem::drop(Box::from_raw(d_env.as_mut()));
        }
    }

    fn execute_if_stmt(&mut self, stmt: &IfStmt) {
        let condition = self.evaluate(&stmt.condition);
        if self.is_truthy(&condition) {
            self.execute(&stmt.then_branch);
        } else {
            if let Some(else_stmt) = &stmt.else_branch {
                self.execute(&else_stmt);
            }
        }
    }

    fn execute_while_stmt(&mut self, stmt: &WhileStmt) {
        let mut condition = self.evaluate(&stmt.condition);
        while self.is_truthy(&condition) {
            let body = &stmt.body;
            self.execute(body);
            condition = self.evaluate(&stmt.condition);
        }
    }

    fn execute_for_stmt(&mut self, stmt: &ForStmt) {
        if let Some(i) = &stmt.initializer {
            match &i {
                ForStmtInitializer::VarDecl(v) => self.define_var_stmt(v),
                ForStmtInitializer::ExprStmt(e) => self.eval_expression_stmt(e),
            }
        }
        if let Some(c) = &stmt.condition {
            let mut condition = self.evaluate(c);
            while self.is_truthy(&condition) {
                let body = &stmt.body;
                self.execute(body);
                if let Some(a) = &stmt.afterthought {
                    self.evaluate(a);
                }
                condition = self.evaluate(c);
            }
        }
    }

    fn declare_fn(&mut self, stmt: &FnStmt) {
        Environment::define_fn(&mut self.environment, stmt.name.lexme.clone(), stmt.clone());
    }

    fn execute_return_stmt(&mut self, stmt: &ReturnStmt) -> LiteralValue {
        match &stmt.value {
            Some(v) => self.evaluate(v),
            None => LiteralValue::Nil,
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expresssion(e) => self.eval_expression_stmt(e),
            Stmt::Print(p) => self.eval_print_stmt(p),
            Stmt::Var(v) => self.define_var_stmt(v),
            Stmt::Block(b) => self.execute_block(b),
            Stmt::IfStmt(i) => self.execute_if_stmt(i),
            Stmt::WhileStmt(w) => self.execute_while_stmt(w),
            Stmt::ForStmt(f) => self.execute_for_stmt(f),
            Stmt::FnStmt(f) => self.declare_fn(f),
            Stmt::ReturnStmt(r) => {
                self.execute_return_stmt(r);
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement)
        }
    }
}
