pub mod rlox_errors {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub struct InvalidToken {
        token: String,
        line: usize,
        column: usize,
        line_text: Option<String>,
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

        pub fn get_token(&self) -> &str {
            &self.token
        }

        pub fn get_column(&self) -> usize {
            self.column
        }

        pub fn get_line(&self) -> usize {
            self.line
        }

        pub fn get_line_text(&self) -> Option<&str> {
            self.line_text.as_deref()
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
        line: usize,
        column: usize,
        line_text: Option<String>,
    }

    impl UnterminatedString {
        pub fn new(line: usize, column: usize, line_text: Option<String>) -> Self {
            UnterminatedString {
                line,
                column,
                line_text,
            }
        }

        pub fn get_column(&self) -> usize {
            self.column
        }

        pub fn get_line(&self) -> usize {
            self.line
        }

        pub fn get_line_text(&self) -> Option<&str> {
            self.line_text.as_deref()
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
        line: usize,
        column: usize,
        line_text: Option<String>,
    }

    impl UnterminatedComment {
        pub fn new(line: usize, column: usize, line_text: Option<String>) -> Self {
            UnterminatedComment {
                line,
                column,
                line_text,
            }
        }

        pub fn get_column(&self) -> usize {
            self.column
        }

        pub fn get_line(&self) -> usize {
            self.line
        }

        pub fn get_line_text(&self) -> Option<&str> {
            self.line_text.as_deref()
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
}
