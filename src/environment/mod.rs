use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::token::LiteralValue;

pub type RcEnvironment = Rc<RefCell<Environment>>;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    pub enclosing: Option<RcEnvironment>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Environment {
                values: HashMap::new(),
                enclosing: None
            }
        ))
    }

    pub fn add_enclosing(&mut self, enclosing: RcEnvironment) {
        self.enclosing = Some(enclosing);
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<(), ()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            self.enclosing.as_ref().map(|e| {
                e.borrow_mut().assign(name, value)
            });
            Err(())
        }
    }

    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        match self.values.get(name) {
            Some(v) => Some(v.clone()),
            None => self.enclosing.as_ref().and_then(|e| {
                e.borrow().get(name)
            })
        }
    }

    pub fn check(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
