use std::{collections::HashMap, ptr::NonNull};

use crate::{ast::stmt::FnStmt, token::LiteralValue};

pub type NonNullScope = NonNull<Scope>;

pub struct Environment {
    scope: NonNullScope,
}

pub struct Scope {
    var_values: HashMap<String, LiteralValue>,
    fn_dcls: HashMap<String, FnStmt>,
    pub enclosing: Option<NonNullScope>,
}

impl Environment {
    pub fn new() -> Environment {
        let scope = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Scope {
                var_values: HashMap::new(),
                fn_dcls: HashMap::new(),
                enclosing: None,
            })))
        };
        Environment { scope }
    }

    pub fn add_enclosing(&mut self, enclosing: &Environment) {
        let mut env = self.scope;
        unsafe {
            let mut_env = env.as_mut();
            mut_env.enclosing = Some(enclosing.scope.clone());
        }
    }

    pub fn define_var(&mut self, name: String, value: LiteralValue) {
        let mut env = self.scope;
        unsafe {
            let mut_env = env.as_mut();
            mut_env.var_values.insert(name, value);
        }
    }

    pub fn assign_var(&mut self, name: String, value: LiteralValue) -> Result<(), ()> {
        let mut env = self.scope;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                if mut_env.var_values.contains_key(&name) {
                    mut_env.var_values.insert(name, value);
                    return Ok(());
                } else {
                    match mut_env.enclosing {
                        Some(e) => env = e,
                        None => return Err(()),
                    }
                }
            }
        }
    }

    pub fn define_fn(&mut self, name: String, value: FnStmt) {
        let mut env = self.scope;
        unsafe {
            let mut_env = env.as_mut();
            mut_env.fn_dcls.insert(name, value);
        }
    }

    pub fn get_var<'a>(&mut self, name: &str) -> Option<&'a LiteralValue> {
        let mut env = self.scope;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                match mut_env.var_values.get(name) {
                    Some(v) => return Some(v),
                    None => match mut_env.enclosing {
                        Some(e) => env = e,
                        None => return None,
                    },
                }
            }
        }
    }

    pub fn get_fn<'a>(&mut self, name: &str) -> Option<&'a FnStmt> {
        let mut env = self.scope;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                match mut_env.fn_dcls.get(name) {
                    Some(v) => return Some(v),
                    None => match mut_env.enclosing {
                        Some(e) => env = e,
                        None => return None,
                    },
                }
            }
        }
    }

    pub fn check(&mut self, name: &str) -> bool {
        let mut env = self.scope;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                match mut_env.var_values.contains_key(name) {
                    true => return true,
                    false => match mut_env.enclosing {
                        Some(e) => env = e,
                        None => return false,
                    },
                }
            }
        }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.scope.as_ptr());
        }
    }
}
