use rlox_macros::rlox_error;
use std::{error::Error, fmt::Display, usize};

#[derive(Debug)]
#[rlox_error]
pub struct MalformedExpression {}

impl Error for MalformedExpression {}

impl Display for MalformedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
#[rlox_error]
pub struct NoLiteralValue {}

impl Error for NoLiteralValue {}

impl Display for NoLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "missing value for token, {}", self.msg)
    }
}

#[derive(Debug)]
pub enum ParserError {
    ExprError(MalformedExpression),
    StmtError(MalformedExpression),
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
        ParserError::StmtError(MalformedExpression {
            line,
            column,
            msg: "missing ; after statement".to_string(),
        })
    }

    pub fn invalid_expression(line: usize, column: usize, msg: String) -> Self {
        ParserError::ExprError(MalformedExpression { line, column, msg })
    }

    pub fn invalid_stmt(line: usize, column: usize, msg: String) -> Self {
        ParserError::StmtError(MalformedExpression { line, column, msg })
    }

    pub fn missing_literal(line: usize, column: usize, msg: String) -> Self {
        ParserError::ValueError(NoLiteralValue { line, column, msg })
    }
}

impl From<MalformedExpression> for ParserError {
    fn from(value: MalformedExpression) -> Self {
        ParserError::ExprError(value)
    }
}

impl From<NoLiteralValue> for ParserError {
    fn from(value: NoLiteralValue) -> Self {
        ParserError::ValueError(value)
    }
}
