use std::{collections::HashMap, mem};

use crate::{
    ast::{expr::*, stmt::*}, callable::Callable, class::{FieldType, RloxClass, RloxInstance}, environment::Environment, errors::interpreter_errors::RuntimeError, token::{RloxValue, Token, TokenType}
};

enum ControlFlow {
    Continue,
    Break,
    Return(RloxValue),
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

impl <'a>From<ControlFlow> for RuntimeState {
    fn from(value: ControlFlow) -> Self {
        RuntimeState::Cf(value)
    }
}

type EvalExprResult = Result<RloxValue, RuntimeState>;
type EvalStmtResult = Result<(), RuntimeState>;

pub struct Interpreter {
    current_env: Environment,
    locals: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn new(env: Environment, locals: HashMap<usize, usize>) -> Self {
        Interpreter {
            current_env: env,
            locals,
        }
    }

    fn value_error(&self, message: &str, token: &Token) -> RuntimeState {
        let e = RuntimeError::value_error(token.line, token.column, message.to_string());
        RuntimeState::RtErr(e)
    }

    fn expression_error(&self, message: &str, token: &Token) -> RuntimeState {
        let e = RuntimeError::expression_error(token.line, token.column, message.to_string());
        RuntimeState::RtErr(e)
    }

    fn is_truthy(&self, value: &RloxValue) -> bool {
        match value {
            RloxValue::Bool(b) => *b,
            RloxValue::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, v1: &RloxValue, v2: &RloxValue) -> bool {
        match (v1, v2) {
            (RloxValue::Nil, RloxValue::Nil) => true,
            (RloxValue::Nil, _) => false,
            (_, RloxValue::Nil) => false,
            (RloxValue::Num(n1), RloxValue::Num(n2)) => n1 == n2,
            (RloxValue::Str(s1), RloxValue::Str(s2)) => s1 == s2,
            (RloxValue::Bool(b1), RloxValue::Bool(b2)) => b1 == b2,
            _ => false,
        }
    }

    fn eval_literal(&self, expr: &Literal) -> EvalExprResult {
        Ok(expr.value.convert())
    }

    fn eval_group(&mut self, expr: &Grouping) -> EvalExprResult {
        self.evaluate(&expr.expression)
    }

    fn eval_unary(&mut self, expr: &Unary) -> EvalExprResult {
        let right = self.evaluate(&expr.right)?;

        Ok(match expr.operator.r#type {
            TokenType::Minus => match right {
                RloxValue::Num(n) => RloxValue::Num(-n),
                _ => RloxValue::Nil,
            },
            TokenType::Bang => RloxValue::Bool(self.is_truthy(&right)),
            _ => RloxValue::Nil,
        })
    }

    fn eval_binary(&mut self, expr: &Binary) -> EvalExprResult {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        Ok(match expr.operator.r#type {
            TokenType::Minus => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Num(n1 - n2),
                _ => RloxValue::Nil,
            },
            TokenType::Slash => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Num(n1 / n2),
                _ => RloxValue::Nil,
            },
            TokenType::Star => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Num(n1 * n2),
                _ => RloxValue::Nil,
            },
            TokenType::Plus => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Num(n1 + n2),
                (RloxValue::Str(s1), RloxValue::Str(s2)) => RloxValue::Str(format!("{}{}", s1, s2)),
                _ => RloxValue::Nil,
            },
            TokenType::Greater => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Bool(n1 > n2),
                _ => RloxValue::Nil,
            },
            TokenType::GreaterEqual => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Bool(n1 >= n2),
                _ => RloxValue::Nil,
            },
            TokenType::Less => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Bool(n1 < n2),
                _ => RloxValue::Nil,
            },
            TokenType::LessEqual => match (left, right) {
                (RloxValue::Num(n1), RloxValue::Num(n2)) => RloxValue::Bool(n1 <= n2),
                _ => RloxValue::Nil,
            },
            TokenType::BangEqual => RloxValue::Bool(!self.is_equal(&left, &right)),
            TokenType::EqualEqual => RloxValue::Bool(self.is_equal(&left, &right)),
            _ => RloxValue::Nil,
        })
    }

    fn look_up_variable(&mut self, name: &Token, expr: &Variable) -> EvalExprResult {
        match self.locals.get(&expr.id) {
            Some(d) => {
                let res = self.current_env.get_at(*d, name.lexme.clone());
                match res {
                    Ok(value) => match value {
                        Some(v) => Ok(v.clone()),
                        None => Ok(RloxValue::Nil),
                    },
                    Err(_) => Ok(RloxValue::Nil),
                }
            }
            None => match self.current_env.get_var(&name.lexme) {
                Some(v) => Ok(v.clone()),
                None => Ok(RloxValue::Nil),
            },
        }
    }

    fn eval_variable(&mut self, expr: &Variable) -> EvalExprResult {
        self.look_up_variable(&expr.name, expr)
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

        let distance = self.locals.get(&expr.id);
        match distance {
            Some(d) => {
                if let Err(_) = self.current_env.assign_at(*d, var_name.clone(), value) {
                    return Err(self.value_error(
                        &format!("cannot assign value to {} in this scope", var_name),
                        &expr.name,
                    ));
                }
            }
            None => {
                if let Err(_) = self.current_env.assign_var(var_name.clone(), value) {
                    return Err(self.value_error(
                        &format!("cannot assign value to {} in this scope", var_name),
                        &expr.name,
                    ));
                }
            }
        }
        Ok(RloxValue::Nil)
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
                    Ok(RloxValue::Bool(false))
                }
            }
            _ => Err(self.expression_error(
                &format!("invalid logical operator {}", expr.operator.lexme),
                &expr.operator,
            )),
        }
    }

    fn call(&mut self, args: &Vec<RloxValue>, callable: &Callable) -> EvalExprResult {
        let mut ret_val = RloxValue::Nil;
        let mut env = Environment::new();
        env.add_enclosing(&self.current_env);
        let prev = mem::replace(&mut self.current_env, env);
        let fun_stmt = callable.function.clone();

        for (i, param) in fun_stmt.params.iter().enumerate() {
            self.current_env
                .define_var(param.lexme.clone(), args[i].clone());
        }

        for stmt in &fun_stmt.body {
            if let Err(e) = self.execute(stmt) {
                match e {
                    RuntimeState::Cf(c) => match c {
                        ControlFlow::Return(v) => {
                            ret_val = v;
                            break;
                        }
                        _ => (),
                    },
                    _ => return Err(e),
                };
            };
        }
        self.current_env = prev;
        return Ok(ret_val);
    }

    fn eval_call(&mut self, expr: &Call) -> EvalExprResult {
        let mut args = Vec::new();
        let name = self.evaluate(&expr.callee)?.to_string();
        for arg in &expr.args {
            args.push(self.evaluate(&arg)?);
        }

        if let Some(&ref val) = self.current_env.get_var(&name) {
            match val {
                RloxValue::Callable(c) => return self.call(&args, c),
                _ => {
                    return Err(self.value_error(
                        &format!("cannot find function {} in this scope", name),
                        &expr.paren,
                    ))
                }
            }
        }

        if let Some(class) = self.current_env.get_class(&name) {
            let instance = RloxInstance::new(class.clone(), args);
            return Ok(RloxValue::Instance(instance));
        }

        Err(self.value_error(
            &format!("cannot find function {} in this scope", name),
            &expr.paren,
        ))
    }

    fn call_method(&mut self, instance: &RloxInstance, method: &Callable, method_args: &Option<Vec<Expr>>) -> EvalExprResult {
        let mut env = Environment::new();
        env.add_enclosing(&self.current_env);
        env.define_var("this".to_string(), RloxValue::Instance(instance.clone()));
        let prev = mem::replace(&mut self.current_env, env);
        let mut args = Vec::new();
        if let Some(m_args) = method_args {
            for arg in m_args {
                args.push(self.evaluate(&arg)?);
            }
        }
        let ret_val = self.call(&args, method);
        self.current_env = prev;
        ret_val
    }

    fn eval_get(&mut self, expr: &Get) -> EvalExprResult {
        let object = self.evaluate(&expr.object)?;
        let args = &expr.method_args;
        match object {
            RloxValue::Instance(i) => {
                match i.get(&expr.name.lexme) {
                    Some(v) => match v {
                        FieldType::Field(f) => Ok(f.clone()),
                        FieldType::Method(m) => self.call_method(&i, m, args),
                    },
                    None => Err(self.value_error("undefined property", &expr.name)),
                }
            }
            _ => return Err(self.value_error("only instances have properties", &expr.name)),
        }
    }

    fn eval_set(&mut self, expr: &Set) -> EvalExprResult {
        let object = self.evaluate(&expr.object)?;
        match object {
            RloxValue::Instance(mut i) => {
                let value = self.evaluate(&expr.value)?;
                return match i.set(expr.name.lexme.clone(), value) {
                    Some(v) => Ok(v),
                    None => Err(self.value_error("only instances have properties", &expr.name)),
                };
            }
            _ => return Err(self.value_error("only instances have properties", &expr.name)),
        }
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
            Expr::Get(g) => self.eval_get(g),
            Expr::Set(s) => self.eval_set(s),
            Expr::This(t) => todo!("this"),
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
        let callable = RloxValue::Callable(Callable::new(stmt.clone()));
        self.current_env
            .define_var(stmt.name.lexme.clone(), callable);
        Ok(())
    }

    fn execute_return_stmt(&mut self, stmt: &ReturnStmt) -> EvalStmtResult {
        let val = match &stmt.value {
            Some(v) => self.evaluate(v)?,
            None => RloxValue::Nil,
        };
        Err(RuntimeState::Cf(ControlFlow::Return(val)))
    }

    fn execute_break_stmt(&mut self, _stmt: &BreakStmt) -> EvalStmtResult {
        Err(RuntimeState::Cf(ControlFlow::Break))
    }

    fn execute_cont_stmt(&mut self, _stmt: &ContStmt) -> EvalStmtResult {
        Err(RuntimeState::Cf(ControlFlow::Continue))
    }

    fn execute_class_stmt(&mut self, stmt: &Class) -> EvalStmtResult {
        let name = &stmt.name.lexme;
        let mut methods = HashMap::new();
        let init_params = stmt.params.iter().map(|p| p.lexme.to_string()).collect();
        for method in &stmt.methods {
            let callable = Callable::new(method.clone());
            methods.insert(method.name.lexme.clone(), callable);
        }
        let rlox_class = RloxClass::new(name.clone(), methods, init_params);
        self.current_env.define_class(name.clone(), rlox_class);
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
            Stmt::ContStmt(f) => self.execute_cont_stmt(f),
            Stmt::ReturnStmt(r) => self.execute_return_stmt(r),
            Stmt::Class(c) => self.execute_class_stmt(c),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in &statements {
            if let Err(e) = self.execute(statement) {
                match e {
                    RuntimeState::RtErr(e) => return Err(e),
                    _ => (),
                }
            };
        }
        Ok(())
    }
}
