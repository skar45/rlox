pub mod rlox_errors {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub struct GenericScannerError {
        line: usize,
        column: usize
    }

    impl GenericScannerError {
        pub fn get_column(&self) -> usize {
            self.column
        }

        pub fn get_line(&self) -> usize {
            self.line
        }
    }

    impl Error for GenericScannerError {}

    impl Display for GenericScannerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "scanner error")
        }
    }

    #[derive(Debug)]
    pub struct InvalidToken {
        token: String,
        source: GenericScannerError
    }

    impl InvalidToken {
        pub fn new(line: usize, column: usize, token: String) -> Self {
            InvalidToken {
                token,
                source: GenericScannerError {
                    line,
                    column
                }
            }
        }

        pub fn get_token(&self) -> &str {
            &self.token
        }

        pub fn get_column(&self) -> usize {
            self.source.get_column()
        }

        pub fn get_line(&self) -> usize {
            self.source.get_line()
        }

    }


    impl Error for InvalidToken {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.source)
        }
    }

    impl Display for InvalidToken {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "invalid token: {}", self.token)
        }
    }

    #[derive(Debug)]
    pub struct UnterminatedString {
        source: GenericScannerError
    }

    impl UnterminatedString {
        pub fn new(line: usize, column: usize) -> Self {
            UnterminatedString {
                source: GenericScannerError {
                    line,
                    column
                }
            }
        }
    }

    impl Error for UnterminatedString {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.source)
        }
    }

    impl Display for UnterminatedString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "unterminated string literal")
        }
    }

    pub enum ScannerError {
        InvalidToken(InvalidToken),
        UnterminatedString(UnterminatedString)
    }
}
