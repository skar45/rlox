use std::{error::Error, fmt::Display};

use super::ReportError;
use rlox_macros::{rlox_error, rlox_error_enum};

#[derive(Debug)]
#[rlox_error]
pub struct VariableError {}

#[rlox_error_enum]
pub enum ResolverError {
    Variable(VariableError),
}

impl ResolverError {
    pub fn resolve_var_error(line: usize, column: usize, msg: String) -> Self {
        ResolverError::Variable(VariableError { line, column, msg })
    }
}
