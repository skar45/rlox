use crate::errors::scanner_errors::*;
use crate::token::*;

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    start_column: usize,
    current_column: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.reserve(4096);
        Scanner {
            source: source.chars().collect(),
            tokens,
            start: 0,
            current: 0,
            line: 0,
            start_column: 0,
            current_column: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.start_column = self.current_column;
            if let Some(c) = self.advance() {
                match c {
                    '(' => self.add_token(TokenType::LeftParen),
                    ')' => self.add_token(TokenType::RightParen),
                    '{' => self.add_token(TokenType::RightBrace),
                    '}' => self.add_token(TokenType::LeftBrace),
                    ',' => self.add_token(TokenType::Comma),
                    '.' => self.add_token(TokenType::Dot),
                    '+' => self.add_token(TokenType::Plus),
                    '-' => self.add_token(TokenType::Minus),
                    ';' => self.add_token(TokenType::Semicolon),
                    '*' => self.add_token(TokenType::Star),
                    '!' => {
                        match self.char_match('=') {
                            true => self.add_token(TokenType::BangEqual),
                            false => self.add_token(TokenType::Bang),
                        };
                    }
                    '=' => {
                        match self.char_match('=') {
                            true => self.add_token(TokenType::Equal),
                            false => self.add_token(TokenType::EqualEqual),
                        };
                    }
                    '<' => {
                        match self.char_match('=') {
                            true => self.add_token(TokenType::LessEqual),
                            false => self.add_token(TokenType::Less),
                        };
                    }
                    '>' => {
                        match self.char_match('=') {
                            true => self.add_token(TokenType::GreaterEqual),
                            false => self.add_token(TokenType::Greater),
                        };
                    }
                    '/' => {
                        match self.char_match('/') {
                            true => {
                                while self.peek() != '\n' && !self.is_at_end() {
                                    self.increment_current(1);
                                }
                            }
                            false => {
                                match self.char_match('*') {
                                    true => self.process_block_comments()?,
                                    false => self.add_token(TokenType::Slash),
                                };
                            }
                        };
                    }
                    '\n' => self.increment_line(),
                    '\t' | '\r' | ' ' => continue,
                    '"' => self.process_string_literal()?,
                    rest => {
                        if Scanner::char_is_num(rest) {
                            self.process_numeric_literal()?
                        } else if Scanner::char_is_alpha(rest) {
                            self.process_identifier()?
                        } else {
                            let token = rest.clone();
                            self.line += 1;
                            return Err(self.invalid_token(token));
                        }
                    }
                }
            }
        }
        self.tokens.push(Token::eof_token(self.line));
        Ok(())
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.current_column = 0;
    }

    fn increment_current(&mut self, value: usize) {
        self.current += value;
        self.current_column += value;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if let Some(c) = self.source.get(self.current) {
            *c
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        if let Some(c) = self.source.get(self.current + 1) {
            *c
        } else {
            '\0'
        }
    }

    fn char_match(&mut self, comp: char) -> bool {
        if let Some(c) = self.source.get(self.current) {
            if *c == comp {
                self.increment_current(1);
                return true;
            } else {
                return false;
            }
        }
        false
    }

    fn char_is_num(comp: &char) -> bool {
        (*comp >= '0') && (*comp <= '9')
    }

    fn char_is_alpha(comp: &char) -> bool {
        (*comp >= 'a' && *comp <= 'z') || (*comp >= 'A' && *comp <= 'Z') || (*comp == '_')
    }

    fn char_is_alphanum(comp: &char) -> bool {
        Scanner::char_is_num(comp) || Scanner::char_is_alpha(comp)
    }

    fn process_string_literal(&mut self) -> Result<(), ScannerError> {
        // store column in case the source ends in new line
        let mut prev_column = self.current_column;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                prev_column = self.current_column;
                self.increment_line();
            };
            self.increment_current(1);
        }

        if self.is_at_end() {
            self.current_column = prev_column;
            return Err(self.unterminated_string());
        };

        self.increment_current(1);

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, LiteralValue::Str(value));
        Ok(())
    }

    fn process_numeric_literal(&mut self) -> Result<(), ScannerError> {
        while Scanner::char_is_num(&self.peek()) {
            self.advance();
        }
        if self.peek_next() == '.' {
            self.advance();
            while Scanner::char_is_num(&self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>();
        match value {
            Ok(v) => self.add_token_literal(TokenType::Number, LiteralValue::Num(v)),
            Err(_) => return Err(self.invalid_token(self.source[self.start])),
        }
        Ok(())
    }

    fn process_identifier(&mut self) -> Result<(), InvalidToken> {
        while Scanner::char_is_alphanum(&self.peek()) {
            self.advance();
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let token = TokenType::match_token(value.as_str());
        match token {
            TokenType::True => self.add_token_literal(token, LiteralValue::Bool(true)),
            TokenType::False => self.add_token_literal(token, LiteralValue::Bool(false)),
            TokenType::Nil => self.add_token_literal(token, LiteralValue::Nil),
            _ => self.add_token(token),
        }
        Ok(())
    }

    fn process_block_comments(&mut self) -> Result<(), ScannerError> {
        // store column in case the source ends in new line
        let mut prev_column = self.current_column;
        let mut nested = 1;
        while (nested != 0) && !self.is_at_end() {
            match self.peek() {
                '\n' => {
                    prev_column = self.current_column;
                    self.increment_line();
                    self.increment_current(1);
                }
                '*' => {
                    if self.peek_next() == '/' {
                        nested -= 1;
                        self.increment_current(2);
                    } else {
                        self.increment_current(1);
                    };
                }
                '/' => {
                    if self.peek_next() == '*' {
                        nested += 1;
                        self.increment_current(2);
                    } else {
                        self.increment_current(1);
                    };
                }
                _ => self.increment_current(1),
            };
        }

        if nested > 0 {
            self.current_column = prev_column;
            Err(self.unterminated_comment())
        } else {
            Ok(())
        }
    }

    fn invalid_token(&self, token: char) -> ScannerError {
        ScannerError::invalid_token(
            self.line,
            self.current_column,
            token.to_string(),
            Some(self.get_line_text()),
        )
    }

    fn unterminated_string(&self) -> ScannerError {
        ScannerError::unterminated_string(
            self.line,
            self.current_column,
            Some(self.get_line_text()),
        )
    }

    fn unterminated_comment(&self) -> ScannerError {
        ScannerError::unterminated_comment(
            self.line,
            self.current_column,
            Some(self.get_line_text()),
        )
    }

    fn get_line_text(&self) -> String {
        let start_index = if self.line > 1 {
            self.start - self.start_column
        } else {
            0
        };
        self.source[start_index..self.current].iter().collect()
    }

    fn advance(&mut self) -> Option<&char> {
        let current = self.current;
        self.increment_current(1);
        self.source.get(current)
    }

    fn add_token(&mut self, r#type: TokenType) {
        let lexme = self.source[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: None,
            line: self.line,
            column: self.start_column,
        });
    }

    fn add_token_literal(&mut self, r#type: TokenType, literal: LiteralValue) {
        let lexme = self.source[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: Some(literal),
            line: self.line,
            column: self.start_column,
        });
    }
}
