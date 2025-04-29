use std::rc::Rc;

use crate::ast::stmt::FnStmt;

#[derive(Debug)]
pub struct Callable {
    pub function: Rc<FnStmt>
}

impl Callable {
    pub fn new(fun: FnStmt) -> Self {
        Callable { function: Rc::new(fun) }
    }
}

impl Clone for Callable {
    fn clone(&self) -> Self {
        Callable { function: self.function.clone() }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fun {}", self.function.name.lexme)
    }
}
