use std::{error::Error, fmt::Display};

use super::ReportError;
use rlox_macros::{rlox_error, rlox_error_enum};

#[derive(Debug)]
#[rlox_error]
pub struct ValueError {}

#[derive(Debug)]
#[rlox_error]
pub struct ExpresssionError {}

#[rlox_error_enum]
pub enum RuntimeError {
    InvalidValue(ValueError),
    InvalidExpression(ExpresssionError),
}

impl RuntimeError {
    pub fn value_error(line: usize, column: usize, msg: String) -> Self {
        RuntimeError::InvalidValue(ValueError { msg, line, column })
    }

    pub fn expression_error(line: usize, column: usize, msg: String) -> Self {
        RuntimeError::InvalidExpression(ExpresssionError { line, column, msg })
    }
}
