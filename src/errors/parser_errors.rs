use std::{error::Error, fmt::Display, usize};

use super::ReportError;
use rlox_macros::{rlox_error, rlox_error_enum};

#[derive(Debug)]
#[rlox_error]
pub struct MalformedExpression {}

#[derive(Debug)]
#[rlox_error]
pub struct MalformedStatement {}

#[derive(Debug)]
#[rlox_error]
pub struct NoLiteralValue {}

#[derive(Debug)]
#[rlox_error_enum]
pub enum ParserError {
    ExprError(MalformedExpression),
    StmtError(MalformedStatement),
    ValueError(NoLiteralValue),
}

impl ParserError {
    pub fn missing_right_paren(line: usize, column: usize) -> Self {
        ParserError::ExprError(MalformedExpression {
            line,
            column,
            msg: "missing ) at the end of expression".to_string(),
        })
    }

    pub fn missing_semicolon(line: usize, column: usize) -> Self {
        ParserError::StmtError(MalformedStatement {
            line,
            column,
            msg: "missing ; after statement".to_string(),
        })
    }

    pub fn invalid_expression(line: usize, column: usize, msg: String) -> Self {
        ParserError::ExprError(MalformedExpression { line, column, msg })
    }

    pub fn invalid_stmt(line: usize, column: usize, msg: String) -> Self {
        ParserError::StmtError(MalformedStatement { line, column, msg })
    }

    pub fn missing_literal(line: usize, column: usize, msg: String) -> Self {
        ParserError::ValueError(NoLiteralValue { line, column, msg })
    }
}
