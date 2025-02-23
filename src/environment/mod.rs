use std::{collections::HashMap, ptr::NonNull};

use crate::{ast::stmt::FnStmt, token::LiteralValue};

pub type RcEnvironment = NonNull<Environment>;

pub struct Environment {
    var_values: HashMap<String, LiteralValue>,
    fn_dcls: HashMap<String, FnStmt>,
    pub enclosing: Option<RcEnvironment>,
}

impl Environment {
    pub fn new() -> RcEnvironment {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Environment {
                var_values: HashMap::new(),
                fn_dcls: HashMap::new(),
                enclosing: None,
            })))
        }
    }

    pub fn add_enclosing(env: &mut RcEnvironment, enclosing: RcEnvironment) {
        unsafe {
            let mut_env = env.as_mut();
            mut_env.enclosing = Some(enclosing);
        }
    }

    pub fn define_var(env: &mut RcEnvironment, name: String, value: LiteralValue) {
        unsafe {
            let mut_env = env.as_mut();
            mut_env.var_values.insert(name, value);
        }
    }

    pub fn assign_var(
        env: &mut RcEnvironment,
        name: String,
        value: LiteralValue,
    ) -> Result<(), ()> {
        unsafe {
            let mut_env = env.as_mut();
            if mut_env.var_values.contains_key(&name) {
                mut_env.var_values.insert(name, value);
                Ok(())
            } else {
                match mut_env.enclosing {
                    Some(mut e) => Environment::assign_var(&mut e, name, value),
                    None => Err(()),
                }
            }
        }
    }

    pub fn define_fn(env: &mut RcEnvironment, name: String, value: FnStmt) {
        unsafe {
            let mut_env = env.as_mut();
            mut_env.fn_dcls.insert(name, value);
        }
    }

    pub fn get_var<'a>(env: &mut RcEnvironment, name: &str) -> Option<&'a LiteralValue> {
        unsafe {
            let mut_env = env.as_mut();
            match mut_env.var_values.get(name) {
                Some(v) => Some(v),
                None => mut_env
                    .enclosing
                    .and_then(|mut e| Environment::get_var(&mut e, name)),
            }
        }
    }

    pub fn get_fn<'a>(env: &mut RcEnvironment, name: &str) -> Option<&'a FnStmt> {
        unsafe {
            let mut_env = env.as_mut();
            match mut_env.fn_dcls.get(name) {
                Some(v) => Some(v),
                None => mut_env
                    .enclosing
                    .and_then(|mut e| Environment::get_fn(&mut e, name)),
            }
        }
    }

    pub fn check(env: &mut RcEnvironment, name: &str) -> bool {
        unsafe {
            let mut_env = env.as_mut();
            match mut_env.var_values.contains_key(name) {
                true => true,
                false => match mut_env.enclosing {
                    Some(mut e) => Environment::check(&mut e, name),
                    None => false,
                },
            }
        }
    }
}
