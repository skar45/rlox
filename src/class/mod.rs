use std::collections::HashMap;

use crate::{callable::Callable, token::RloxValue};

#[derive(Debug, Clone)]
pub struct RloxClass {
    pub name: String,
    pub methods: HashMap<String, Callable>,
    pub params: Vec<String>,
}

impl RloxClass {
    pub fn new(name: String, methods: HashMap<String, Callable>, params: Vec<String>) -> Self {
        RloxClass {
            name,
            methods,
            params,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<&Callable> {
        self.methods.get(name)
    }
}

impl std::fmt::Display for RloxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct RloxInstance {
    pub class: RloxClass,
    pub fields: HashMap<String, RloxValue>,
}

pub enum FieldType<'a> {
    Method(&'a Callable),
    Field(&'a RloxValue),
}

impl RloxInstance {
    pub fn new(class: RloxClass, args: Vec<RloxValue>) -> Self {
        let mut fields = HashMap::new();
        for (i, arg) in args.into_iter().enumerate() {
            fields.insert(class.params[i].to_string(), arg);
        }
        RloxInstance { class, fields }
    }

    pub fn get(&self, name: &str) -> Option<FieldType> {
        if let Some(f) = self.fields.get(name) {
            return Some(FieldType::Field(f));
        }

        if let Some(m) = self.class.find_method(name) {
            return Some(FieldType::Method(m));
        }

        None
    }

    pub fn set(&mut self, name: String, value: RloxValue) -> Option<RloxValue> {
        self.fields.insert(name, value)
    }
}

impl std::fmt::Display for RloxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
