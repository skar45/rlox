use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidToken {
    pub token: String,
    pub line: usize,
    pub column: usize,
    pub line_text: Option<String>,
}

impl InvalidToken {
    pub fn new(line: usize, column: usize, token: String, line_text: Option<String>) -> Self {
        InvalidToken {
            token,
            line,
            column,
            line_text,
        }
    }
}

impl Error for InvalidToken {}

impl From<InvalidToken> for ScannerError {
    fn from(value: InvalidToken) -> Self {
        ScannerError::TokenError(value)
    }
}

impl Display for InvalidToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid token: {}", self.token)
    }
}

#[derive(Debug)]
pub struct UnterminatedString {
    pub line: usize,
    pub column: usize,
    pub line_text: Option<String>,
}

impl UnterminatedString {
    pub fn new(line: usize, column: usize, line_text: Option<String>) -> Self {
        UnterminatedString {
            line,
            column,
            line_text,
        }
    }
}

impl Error for UnterminatedString {}

impl From<UnterminatedString> for ScannerError {
    fn from(value: UnterminatedString) -> Self {
        ScannerError::StringError(value)
    }
}

impl Display for UnterminatedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unterminated string literal")
    }
}

#[derive(Debug)]
pub struct UnterminatedComment {
    pub line: usize,
    pub column: usize,
    pub line_text: Option<String>,
}

impl UnterminatedComment {
    pub fn new(line: usize, column: usize, line_text: Option<String>) -> Self {
        UnterminatedComment {
            line,
            column,
            line_text,
        }
    }
}

impl Error for UnterminatedComment {}

impl From<UnterminatedComment> for ScannerError {
    fn from(value: UnterminatedComment) -> Self {
        ScannerError::CommentError(value)
    }
}

impl Display for UnterminatedComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unterminated comment block")
    }
}

pub enum ScannerError {
    TokenError(InvalidToken),
    StringError(UnterminatedString),
    CommentError(UnterminatedComment),
}
