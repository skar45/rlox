use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

#[derive(Debug)]
struct ClassCtx {
    pub class: RloxClass,
    pub fields: HashMap<String, RloxValue>,
}

#[derive(Debug)]
pub struct RloxInstance {
    ctx: Rc<RefCell<ClassCtx>>
}

pub enum FieldType {
    Method(Callable),
    Field(RloxValue),
}

impl RloxInstance {
    pub fn new(class: RloxClass, args: Vec<RloxValue>) -> Self {
        let mut fields = HashMap::new();
        for (i, arg) in args.into_iter().enumerate() {
            fields.insert(class.params[i].to_string(), arg);
        }
        let ctx = ClassCtx { class, fields };
        RloxInstance { ctx: Rc::new(RefCell::new(ctx)) }
    }

    pub fn get(&self, name: &str) -> Option<FieldType> {
        let ctx = self.ctx.borrow();
        if let Some(f) = ctx.fields.get(name) {
            return Some(FieldType::Field(f.clone()));
        }

        if let Some(m) = ctx.class.find_method(name) {
            return Some(FieldType::Method(m.clone()));
        }

        None
    }

    pub fn set(&mut self, name: String, value: RloxValue) -> Option<RloxValue> {
        let mut ctx = self.ctx.borrow_mut();
        ctx.fields.insert(name, value)
    }
}

impl std::fmt::Display for RloxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.ctx.borrow().class.name)
    }
}

impl Clone for RloxInstance {
    fn clone(&self) -> Self {
        RloxInstance { ctx: self.ctx.clone() }
    }
}


