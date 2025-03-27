use std::mem;

use crate::{
    ast::{expr::*, stmt::*},
    environment::Environment,
    errors::interpreter_errors::RuntimeError,
    token::{LiteralValue, Token, TokenType},
};

enum ControlFlow {
    Continue,
    Break,
    Return(LiteralValue),
}

enum RuntimeState {
    Cf(ControlFlow),
    RtErr(RuntimeError),
}

impl From<RuntimeError> for RuntimeState {
    fn from(value: RuntimeError) -> Self {
        RuntimeState::RtErr(value)
    }
}

impl From<ControlFlow> for RuntimeState {
    fn from(value: ControlFlow) -> Self {
        RuntimeState::Cf(value)
    }
}

type EvalExprResult = Result<LiteralValue, RuntimeState>;
type EvalStmtResult = Result<(), RuntimeState>;

pub struct Interpreter {
    current_env: Environment,
}

impl Interpreter {
    pub fn new(env: Environment) -> Self {
        Interpreter { current_env: env }
    }

    fn value_error(&self, message: &str, token: &Token) -> RuntimeState {
        let e = RuntimeError::value_error(token.line, token.column, message.to_string());
        RuntimeState::RtErr(e)
    }

    fn expression_error(&self, message: &str, token: &Token) -> RuntimeState {
        let e = RuntimeError::expression_error(token.line, token.column, message.to_string());
        RuntimeState::RtErr(e)
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
            return Err(self.value_error(
                &format!("cannot find variable {} in this scope", var_name),
                &expr.name,
            ));
        }
        let value = self.evaluate(&expr.value)?;
        if let Err(_) = self.current_env.assign_var(var_name.clone(), value) {
            return Err(self.value_error(
                &format!("cannot assign value to {} in this scope", var_name),
                &expr.name,
            ));
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
            _ => Err(self.expression_error(
                &format!("invalid logical operator {}", expr.operator.lexme),
                &expr.operator,
            )),
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
                if let Err(e) = self.execute(stmt) {
                    match e {
                        RuntimeState::Cf(c) => match c {
                            ControlFlow::Return(v) => {
                                ret_val = v;
                                break;
                            },
                            _ => (),
                        },
                        _ => return Err(e),
                    };
                };
            }
            let _ = mem::replace(&mut self.current_env, prev);
        } else {
            return Err(self.value_error(
                &format!("cannot find function {} in this scope", &expr.callee),
                &expr.paren,
            ));
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
            let body = &stmt.body;
            if let Err(e) = self.execute(body) {
                match &e {
                    RuntimeState::Cf(c) => match c {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => (),
                        _ => return Err(e),
                    },
                    _ => return Err(e),
                }
            };
            condition = self.evaluate(&stmt.condition)?;
        }
        Ok(())
    }

    fn execute_for_stmt(&mut self, stmt: &ForStmt) -> EvalStmtResult {
        if let Some(i) = &stmt.initializer {
            match &i {
                ForStmtInitializer::VarDecl(v) => self.define_var_stmt(v)?,
                ForStmtInitializer::ExprStmt(e) => self.eval_expression_stmt(e)?,
            };
        }
        if let Some(c) = &stmt.condition {
            let mut condition = self.evaluate(c)?;
            while self.is_truthy(&condition) {
                let body = &stmt.body;
                if let Err(e) = self.execute(body) {
                    match &e {
                        RuntimeState::Cf(c) => match c {
                            ControlFlow::Break => break,
                            ControlFlow::Continue => (),
                            _ => return Err(e),
                        },
                        _ => return Err(e),
                    }
                };
                if let Some(a) = &stmt.afterthought {
                    self.evaluate(a)?;
                }
                condition = self.evaluate(c)?;
            }
        }
        Ok(())
    }

    fn declare_fn(&mut self, stmt: &FnStmt) -> EvalStmtResult {
        self.current_env
            .define_fn(stmt.name.lexme.clone(), stmt.clone());
        Ok(())
    }

    fn execute_return_stmt(&mut self, stmt: &ReturnStmt) -> EvalStmtResult {
        let val = match &stmt.value {
            Some(v) => self.evaluate(v)?,
            None => LiteralValue::Nil,
        };

        Err(RuntimeState::Cf(ControlFlow::Return(val)))
    }

    fn execute_break_stmt(&mut self, _stmt: &BreakStmt) -> EvalStmtResult {
        Err(RuntimeState::Cf(ControlFlow::Break))
    }

    fn execute_cont_stmt(&mut self, _stmt: &ContStmt) -> EvalStmtResult {
        Err(RuntimeState::Cf(ControlFlow::Continue))
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
            Stmt::ContStmt(f) => self.execute_cont_stmt(f),
            Stmt::ReturnStmt(r) => self.execute_return_stmt(r),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            if let Err(e) = self.execute(&statement) {
                match e {
                    RuntimeState::RtErr(e) => return Err(e),
                    _ => (),
                }
            };
        }
        Ok(())
    }
}
