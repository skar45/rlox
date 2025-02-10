use crate::{
    ast::{
        expr::{Assign, Binary, Expr, Grouping, Literal, Logical, Unary, Variable},
        stmt::{ExprStmt, IfStmt, Stmt, VarStmt, WhileStmt},
    },
    environment::Environment,
    token::{LiteralValue, TokenType},
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
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

    fn eval_variable(&self, expr: &Variable) -> LiteralValue {
        match self.environment.get(&expr.name.lexme) {
            Some(v) => v.clone(),
            None => LiteralValue::Nil,
        }
    }

    fn eval_assign(&mut self, expr: &Assign) -> LiteralValue {
        if !self.environment.check(&expr.name.lexme) {
            todo!("runtime error")
        }
        let value = self.evaluate(&expr.value);
        if let Err(_) = self.environment.assign(expr.name.lexme.clone(), value) {
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
            },
            TokenType::And => {
                let eval_left = self.evaluate(&expr.left);
                if self.is_truthy(&eval_left) {
                    self.evaluate(&expr.right)
                } else {
                    LiteralValue::Bool(false)
                }
            },
            _ => todo!("interpreter error")
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> LiteralValue {
        match expr {
            Expr::Literal(l) => self.eval_literal(l),
            Expr::Binary(b) => self.eval_binary(b),
            Expr::Unary(u) => self.eval_unary(u),
            Expr::Grouping(g) => self.eval_group(g),
            Expr::Variable(v) => self.eval_variable(v),
            Expr::Assign(a) => self.eval_assign(a),
            Expr::Logical(l) => self.eval_logical(l)
        }
    }

    fn eval_expression_stmt(&mut self, stmt: &ExprStmt) {
        self.evaluate(&stmt.expr);
    }

    fn eval_print_stmt(&mut self, stmt: &ExprStmt) {
        let value = self.evaluate(&stmt.expr);
        println!("{}", value);
    }

    fn eval_var_stmt(&mut self, stmt: &VarStmt) {
        let value = self.evaluate(&stmt.initializer);
        self.environment.define(stmt.name.lexme.clone(), value);
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment) {
        let prev_environment = std::mem::replace(&mut self.environment, env);
        for stmt in stmts {
            self.execute(&stmt);
        }
        self.environment = prev_environment;
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
        let condition = self.evaluate(&stmt.condition); 
        while self.is_truthy(&condition) {
            let body = &stmt.body;
            self.execute(body);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expresssion(e) => self.eval_expression_stmt(e),
            Stmt::Print(p) => self.eval_print_stmt(p),
            Stmt::Var(v) => self.eval_var_stmt(v),
            Stmt::Block(b) => self.execute_block(&b.statements, Environment::new()),
            Stmt::IfStmt(i) => self.execute_if_stmt(i),
            Stmt::WhileStmt(w) => self.execute_while_stmt(w),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement)
        }
    }
}
