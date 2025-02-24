use std::{error::Error, fmt::Display};

use rlox_macros::{rlox_error, rlox_error_enum};
use super::ReportError;

#[derive(Debug)]
#[rlox_error]
pub struct InvalidToken {}

#[derive(Debug)]
#[rlox_error]
pub struct UnterminatedString {}


#[derive(Debug)]
#[rlox_error("unterminated comment")]
pub struct UnterminatedComment {}

#[rlox_error_enum]
pub enum ScannerError {
    TokenError(InvalidToken),
    StringError(UnterminatedString),
    CommentError(UnterminatedComment),
}


impl ScannerError {
    pub fn invalid_token(
        line: usize,
        column: usize,
        msg: String,
    ) -> Self {
        ScannerError::TokenError(InvalidToken {
            msg,
            line,
            column,
        })
    }

    pub fn unterminated_string(line: usize, column: usize, msg: String) -> Self {
        ScannerError::StringError(UnterminatedString {
            line,
            column,
            msg
        })
    }

    pub fn unterminated_comment(line: usize, column: usize, msg: String) -> Self {
        ScannerError::CommentError(UnterminatedComment {
            line,
            column,
            msg,
        })
    }
}
