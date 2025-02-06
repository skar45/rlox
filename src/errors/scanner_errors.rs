use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidToken {
    pub token: String,
    pub line: usize,
    pub column: usize,
    pub line_text: Option<String>,
}

impl Error for InvalidToken {}

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

impl Error for UnterminatedString {}

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

impl Error for UnterminatedComment {}

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

impl From<InvalidToken> for ScannerError {
    fn from(value: InvalidToken) -> Self {
        ScannerError::TokenError(value)
    }
}

impl From<UnterminatedString> for ScannerError {
    fn from(value: UnterminatedString) -> Self {
        ScannerError::StringError(value)
    }
}

impl From<UnterminatedComment> for ScannerError {
    fn from(value: UnterminatedComment) -> Self {
        ScannerError::CommentError(value)
    }
}


impl ScannerError {
    pub fn invalid_token(line: usize, column: usize, token: String, line_text: Option<String>) -> Self {
        ScannerError::TokenError(
        InvalidToken {
            token,
            line,
            column,
            line_text,
        }
        )
    }

    pub fn unterminated_string(line: usize, column: usize, line_text: Option<String>) -> Self {
        ScannerError::StringError(UnterminatedString {
            line,
            column,
            line_text,
        })
    }

    pub fn unterminated_comment(line: usize, column: usize, line_text: Option<String>) -> Self {
        ScannerError::CommentError(
        UnterminatedComment {
            line,
            column,
            line_text,
        }
        )
    }
}
