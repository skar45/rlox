use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ast::stmt::FnStmt, token::LiteralValue};

pub type RcEnvironment = Rc<RefCell<Environment>>;

pub struct Environment {
    var_values: HashMap<String, LiteralValue>,
    fn_dcls: HashMap<String, FnStmt>,
    pub enclosing: Option<RcEnvironment>,
}

impl Environment {
    pub fn new() -> RcEnvironment {
        Rc::new(RefCell::new(Environment {
            var_values: HashMap::new(),
            fn_dcls: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn add_enclosing(&mut self, enclosing: RcEnvironment) {
        self.enclosing = Some(enclosing);
    }

    pub fn define_var(&mut self, name: String, value: LiteralValue) {
        self.var_values.insert(name, value);
    }

    pub fn assign_var(&mut self, name: String, value: LiteralValue) -> Result<(), ()> {
        if self.var_values.contains_key(&name) {
            self.var_values.insert(name, value);
            Ok(())
        } else {
            self.enclosing
                .as_ref()
                .map(|e| e.borrow_mut().assign_var(name, value));
            Err(())
        }
    }

    pub fn define_fn(&mut self, name: String, value: FnStmt) {
        self.fn_dcls.insert(name, value);
    }

    pub fn assign_fn(&mut self, name: String, value: FnStmt) -> Result<(), ()> {
        if self.fn_dcls.contains_key(&name) {
            self.fn_dcls.insert(name, value);
            Ok(())
        } else {
            self.enclosing
                .as_ref()
                .map(|e| e.borrow_mut().assign_fn(name, value));
            Err(())
        }
    }

    /// Returns a raw pointer to a the value assigned to a variable.
    /// It should be safe for the following reasons:
    /// - The lifetime of the variable is tied to its environment so the poiner should never be invalid as long as `Environment` gets cleaned up properly.
    /// - The lifetime of the outer scope is longer than the inner scope. Outer variables accessed from the innerscope should always be valid.
    /// - Variables are never removed after they are declared.
    fn get_var_unsafe(&self, name: &str) -> Option<*const LiteralValue> {
        match self.var_values.get(name) {
            Some(v) => Some(v),
            None => self
                .enclosing
                .as_ref()
                .and_then(|e| e.borrow().get_var_unsafe(name)),
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&LiteralValue> {
        unsafe { self.get_var_unsafe(name).map(|v| &*v) }
    }

    fn get_fn_unsafe(&self, name: &str) -> Option<*const FnStmt> {
        match self.fn_dcls.get(name) {
            Some(v) => Some(v),
            None => self
                .enclosing
                .as_ref()
                .and_then(|e| e.borrow().get_fn_unsafe(name)),
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<&FnStmt> {
        unsafe { self.get_fn_unsafe(name).map(|v| &*v) }
    }

    pub fn check(&self, name: &str) -> bool {
        self.var_values.contains_key(name)
    }
}
