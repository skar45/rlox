use crate::ast::stmt::FnStmt;

pub struct CallableFn {
    declaration: FnStmt,
}

impl CallableFn {
    pub fn new(declaration: FnStmt) -> Self {
        CallableFn { declaration }
    }
}
