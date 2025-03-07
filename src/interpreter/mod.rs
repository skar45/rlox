use std::mem;

use crate::{
    ast::{expr::*, stmt::*},
    environment::Environment,
    errors::interpreter_errors::RuntimeError,
    token::{LiteralValue, TokenType},
};

type EvalExprResult = Result<LiteralValue, RuntimeError>;
type EvalStmtResult = Result<(), RuntimeError>;

enum LoopState {
    Break,
    Continue,
}

pub struct Interpreter {
    current_env: Environment,
    loop_state: Option<LoopState>,
}

impl Interpreter {
    pub fn new(env: Environment) -> Self {
        Interpreter {
            current_env: env,
            loop_state: None,
        }
    }

    fn value_error(&self, message: &str) -> RuntimeError {
        RuntimeError::value_error(0, 0, message.to_string())
    }

    fn expression_error(&self, message: &str) -> RuntimeError {
        RuntimeError::expression_error(0, 0, message.to_string())
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

    fn eval_literal(&self, expr: &Literal) -> EvalExprResult {
        Ok(expr.value.clone())
    }

    fn eval_group(&mut self, expr: &Grouping) -> EvalExprResult {
        self.evaluate(&expr.expression)
    }

    fn eval_unary(&mut self, expr: &Unary) -> EvalExprResult {
        let right = self.evaluate(&expr.right)?;

        Ok(match expr.operator.r#type {
            TokenType::Minus => match right {
                LiteralValue::Num(n) => LiteralValue::Num(-n),
                _ => LiteralValue::Nil,
            },
            TokenType::Bang => LiteralValue::Bool(self.is_truthy(&right)),
            _ => LiteralValue::Nil,
        })
    }

    fn eval_binary(&mut self, expr: &Binary) -> EvalExprResult {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        Ok(match expr.operator.r#type {
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
        })
    }

    fn eval_variable(&mut self, expr: &Variable) -> EvalExprResult {
        match self.current_env.get_var(&expr.name.lexme) {
            Some(v) => Ok(v.clone()),
            None => Ok(LiteralValue::Nil),
        }
    }

    fn eval_assign(&mut self, expr: &Assign) -> EvalExprResult {
        let var_name = &expr.name.lexme;
        let check = self.current_env.check(var_name);
        if !check {
            return Err(
                self.value_error(&format!("cannot find variable {} in this scope", var_name))
            );
        }
        let value = self.evaluate(&expr.value)?;
        if let Err(_) = self.current_env.assign_var(var_name.clone(), value) {
            return Err(
                self.value_error(&format!("cannot assign value to {} in this scope", var_name))
            );
        }
        Ok(LiteralValue::Nil)
    }

    fn eval_logical(&mut self, expr: &Logical) -> EvalExprResult {
        match expr.operator.r#type {
            TokenType::Or => {
                let eval_left = self.evaluate(&expr.left)?;
                if self.is_truthy(&eval_left) {
                    Ok(eval_left)
                } else {
                    self.evaluate(&expr.right)
                }
            }
            TokenType::And => {
                let eval_left = self.evaluate(&expr.left)?;
                if self.is_truthy(&eval_left) {
                    self.evaluate(&expr.right)
                } else {
                    Ok(LiteralValue::Bool(false))
                }
            }
            _ => Err(
                self.expression_error(&format!("invalid logical operator {}", expr.operator.lexme))
            )
        }
    }

    fn eval_call(&mut self, expr: &Call) -> EvalExprResult {
        let mut args = Vec::new();
        let mut ret_val = LiteralValue::Nil;
        for arg in &expr.args {
            args.push(self.evaluate(&arg)?);
        }
        let rlox_fn = self.current_env.get_fn(&expr.callee);
        if let Some(fun) = rlox_fn {
            let mut env = Environment::new();
            env.add_enclosing(&self.current_env);
            let prev = mem::replace(&mut self.current_env, env);
            for (i, param) in fun.params.iter().enumerate() {
                self.current_env
                    .define_var(param.lexme.clone(), args[i].clone());
            }
            for stmt in &fun.body {
                match stmt {
                    Stmt::ReturnStmt(r) => {
                        ret_val = self.execute_return_stmt(&r)?;
                        break;
                    }
                    s => self.execute(&s)?,
                }
            }
            let _ = mem::replace(&mut self.current_env, prev);
        } else {
            return Err(
                self.value_error(&format!("cannot find function {} in this scope", &expr.callee))
            );
        }
        Ok(ret_val)
    }

    fn evaluate(&mut self, expr: &Expr) -> EvalExprResult {
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

    fn eval_expression_stmt(&mut self, stmt: &ExprStmt) -> EvalStmtResult {
        self.evaluate(&stmt.expr)?;
        Ok(())
    }

    fn eval_print_stmt(&mut self, stmt: &ExprStmt) -> EvalStmtResult {
        let value = self.evaluate(&stmt.expr)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn define_var_stmt(&mut self, stmt: &VarStmt) -> EvalStmtResult {
        let value = self.evaluate(&stmt.initializer)?;
        self.current_env.define_var(stmt.name.lexme.clone(), value);
        Ok(())
    }

    fn execute_block(&mut self, stmt: &BlockStmt) -> EvalStmtResult {
        let mut new_env = Environment::new();
        new_env.add_enclosing(&self.current_env);
        let prev = mem::replace(&mut self.current_env, new_env);
        for s in &stmt.statements {
            self.execute(s)?;
        }
        let _ = mem::replace(&mut self.current_env, prev);
        Ok(())
    }

    fn execute_if_stmt(&mut self, stmt: &IfStmt) -> EvalStmtResult {
        let condition = self.evaluate(&stmt.condition)?;
        if self.is_truthy(&condition) {
            self.execute(&stmt.then_branch)?;
        } else {
            if let Some(else_stmt) = &stmt.else_branch {
                self.execute(&else_stmt)?;
            }
        }
        Ok(())
    }

    fn execute_while_stmt(&mut self, stmt: &WhileStmt) -> EvalStmtResult {
        let mut condition = self.evaluate(&stmt.condition)?;
        while self.is_truthy(&condition) {
            if let Some(_s) = &self.loop_state {
                self.loop_state = None;
                break;
            }
            let body = &stmt.body;
            self.execute(body)?;
            condition = self.evaluate(&stmt.condition)?;
        }
        Ok(())
    }

    fn execute_for_stmt(&mut self, stmt: &ForStmt) -> EvalStmtResult {
        if let Some(i) = &stmt.initializer {
            match &i {
                ForStmtInitializer::VarDecl(v) => self.define_var_stmt(v)?,
                ForStmtInitializer::ExprStmt(e) => self.eval_expression_stmt(e)?,
            }
        }
        if let Some(c) = &stmt.condition {
            let mut condition = self.evaluate(c)?;
            while self.is_truthy(&condition) {
                if let Some(_s) = &self.loop_state {
                    self.loop_state = None;
                    break;
                }
                let body = &stmt.body;
                self.execute(body)?;
                if let Some(a) = &stmt.afterthought {
                    self.evaluate(a)?;
                }
                condition = self.evaluate(c)?;
            }
        }
        Ok(())
    }

    fn declare_fn(&mut self, stmt: &FnStmt) -> EvalStmtResult {
        Ok(self
            .current_env
            .define_fn(stmt.name.lexme.clone(), stmt.clone()))
    }

    fn execute_return_stmt(&mut self, stmt: &ReturnStmt) -> EvalExprResult {
        Ok(match &stmt.value {
            Some(v) => self.evaluate(v)?,
            None => LiteralValue::Nil,
        })
    }

    fn execute_break_stmt(&mut self, _stmt: &BreakStmt) -> EvalStmtResult {
        self.loop_state = Some(LoopState::Break);
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> EvalStmtResult {
        match stmt {
            Stmt::Expresssion(e) => self.eval_expression_stmt(e),
            Stmt::Print(p) => self.eval_print_stmt(p),
            Stmt::Var(v) => self.define_var_stmt(v),
            Stmt::Block(b) => self.execute_block(b),
            Stmt::IfStmt(i) => self.execute_if_stmt(i),
            Stmt::WhileStmt(w) => self.execute_while_stmt(w),
            Stmt::ForStmt(f) => self.execute_for_stmt(f),
            Stmt::FnStmt(f) => self.declare_fn(f),
            Stmt::BreakStmt(f) => self.execute_break_stmt(f),
            Stmt::ReturnStmt(r) => {
                self.execute_return_stmt(r)?;
                Ok(())
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            let _ = self.execute(&statement);
        }
    }
}
