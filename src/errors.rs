pub mod rlox_errors {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub struct InvalidToken {
        token: String,
        line: usize,
        column: usize,
    }

    impl InvalidToken {
        pub fn new(line: usize, column: usize, token: String) -> Self {
            InvalidToken {
                token,
                line,
                column,
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
    }

    impl UnterminatedString {
        pub fn new(line: usize, column: usize) -> Self {
            UnterminatedString { line, column }
        }

        pub fn get_column(&self) -> usize {
            self.column
        }

        pub fn get_line(&self) -> usize {
            self.line
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

    pub enum ScannerError {
        TokenError(InvalidToken),
        StringError(UnterminatedString),
    }
}
