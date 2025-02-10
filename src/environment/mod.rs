use std::collections::HashMap;

use crate::token::LiteralValue;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn add_enclosing(&mut self, enclosing: Environment) {
        self.enclosing = Some(Box::new(enclosing));
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: LiteralValue) -> Result<(), ()> {
        match &mut self.enclosing {
            Some(e) => e.assign(name, value),
            None => {
                if self.values.contains_key(&name) {
                    self.values.insert(name, value);
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&LiteralValue> {
        match &self.enclosing {
            Some(e) => e.get(name),
            None => self.values.get(name),
        }
    }

    pub fn check(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
