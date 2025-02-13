use std::{error::Error, fmt::Display, usize};

#[derive(Debug)]
pub struct MalformedExpression {
    pub line: usize,
    pub column: usize,
    pub msg: String,
}

impl Error for MalformedExpression {}

impl Display for MalformedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub struct NoLiteralValue {
    pub line: usize,
    pub column: usize,
    pub token: String,
}

impl Error for NoLiteralValue {}

impl Display for NoLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "missing value for token, {}", self.token)
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

    pub fn missing_semicolon(line: usize, column: usize ) -> Self {
        ParserError::StmtError(MalformedExpression {
            line,
            column,
            msg: "missing ; after statement".to_string()
        })
    }

    pub fn invalid_expression(line: usize, column: usize, msg: String) -> Self {
        ParserError::ExprError(MalformedExpression { line, column, msg })
    }

    pub fn invalid_stmt(line: usize, column: usize, msg: String) -> Self {
        ParserError::StmtError(MalformedExpression { line, column, msg })
    }

    pub fn missing_literal(line: usize, column: usize, token: String) -> Self {
        ParserError::ValueError(NoLiteralValue {
            line,
            column,
            token,
        })
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
