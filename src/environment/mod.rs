use std::collections::HashMap;

use crate::token::{LiteralValue, Token};

pub struct Environment {
    values: HashMap<String, LiteralValue>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> &LiteralValue {
        match self.values.get(&name.lexme) {
            Some(v) => v,
            None => todo!("error handling")
        }
    }
}


