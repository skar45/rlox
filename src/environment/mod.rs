use std::{collections::HashMap, ptr::NonNull};

use crate::{class::RloxClass, token::RloxValue};

pub type NonNullCtx = NonNull<EnvCtx>;

pub struct Environment {
    ctx: NonNullCtx,
}

pub struct EnvCtx {
    rlox_vars: HashMap<String, RloxValue>,
    rlox_classes: HashMap<String, RloxClass>,
    pub enclosing: Option<NonNullCtx>,
}

impl Environment {
    pub fn new() -> Environment {
        let ctx = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(EnvCtx {
                rlox_vars: HashMap::new(),
                rlox_classes: HashMap::new(),
                enclosing: None,
            })))
        };
        Environment { ctx }
    }

    pub fn add_enclosing(&mut self, enclosing: &Environment) {
        let mut env = self.ctx;
        unsafe {
            let mut_env = env.as_mut();
            mut_env.enclosing = Some(enclosing.ctx.clone());
        }
    }

    pub fn get_at<'a>(
        &mut self,
        distance: usize,
        name: String,
    ) -> Result<Option<&'a RloxValue>, ()> {
        unsafe {
            let mut env = self.ctx.as_mut();
            for _ in 0..distance {
                match env.enclosing {
                    Some(mut e) => env = e.as_mut(),
                    None => return Err(()),
                }
            }
            Ok(env.rlox_vars.get(&name))
        }
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: String,
        value: RloxValue,
    ) -> Result<(), ()> {
        unsafe {
            let mut env = self.ctx.as_mut();
            for _ in 0..distance {
                match env.enclosing {
                    Some(mut e) => env = e.as_mut(),
                    None => return Err(()),
                }
            }
            env.rlox_vars.insert(name, value);
            Ok(())
        }
    }

    pub fn define_var(&mut self, name: String, value: RloxValue) {
        let mut env = self.ctx;
        unsafe {
            let mut_env = env.as_mut();
            mut_env.rlox_vars.insert(name, value);
        }
    }

    pub fn assign_var(&mut self, name: String, value: RloxValue) -> Result<(), ()> {
        let mut env = self.ctx;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                if mut_env.rlox_vars.contains_key(&name) {
                    mut_env.rlox_vars.insert(name, value);
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

    pub fn define_class(&mut self, name: String, value: RloxClass) {
        let mut env = self.ctx;
        unsafe {
            let mut_env = env.as_mut();
            mut_env.rlox_classes.insert(name, value);
        }
    }

    pub fn get_var<'a>(&mut self, name: &str) -> Option<&'a RloxValue> {
        let mut env = self.ctx;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                match mut_env.rlox_vars.get(name) {
                    Some(v) => return Some(v),
                    None => match mut_env.enclosing {
                        Some(e) => env = e,
                        None => return None,
                    },
                }
            }
        }
    }

    pub fn get_class(&mut self, name: &str) -> Option<&RloxClass> {
        let mut env = self.ctx;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                match mut_env.rlox_classes.get(name) {
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
        let mut env = self.ctx;
        unsafe {
            loop {
                let mut_env = env.as_mut();
                match mut_env.rlox_vars.contains_key(name) {
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
            let _ = Box::from_raw(self.ctx.as_ptr());
        }
    }
}
