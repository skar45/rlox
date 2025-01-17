use crate::errors::rlox_errors::*;
use crate::token::*;

pub struct Scanner {
    chars: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.reserve(128);
        Scanner {
            chars: source.chars().collect(),
            tokens,
            start: 0,
            current: 0,
            line: 0,
            column: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;
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
                    def => {
                        if Scanner::char_is_num(def) {
                            self.process_number_literal()?
                        } else if Scanner::char_is_alpha(def) {
                            self.process_identifier()?
                        } else {
                            let token = def.clone();
                            self.line += 1;
                            let error = self.invalid_token(token);
                            return Err(error.into());
                        }
                    }
                }
            }
        }
        self.tokens.push(Token::eof_token());
        Ok(())
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    fn increment_current(&mut self, value: usize) {
        self.current += value;
        self.column += value;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn peek(&self) -> char {
        if let Some(c) = self.chars.get(self.current) {
            *c
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        if let Some(c) = self.chars.get(self.current + 1) {
            *c
        } else {
            '\0'
        }
    }

    fn char_match(&mut self, comp: char) -> bool {
        if let Some(c) = self.chars.get(self.current) {
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

    fn process_string_literal(&mut self) -> Result<(), UnterminatedString> {
        // store column in case the source ends in new line
        let mut prev_column = self.column;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                prev_column = self.column;
                self.increment_line();
            };
            self.increment_current(1);
        }

        if self.is_at_end() {
            self.column = prev_column;
            return Err(self.unterminated_string());
        };

        self.increment_current(1);

        let value: String = self.chars[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, LiteralType::Str(value));
        Ok(())
    }

    fn process_number_literal(&mut self) -> Result<(), InvalidToken> {
        while Scanner::char_is_num(&self.peek()) {
            self.advance();
        }
        if self.peek_next() == '.' {
            self.advance();
            while Scanner::char_is_num(&self.peek()) {
                self.advance();
            }
        }

        let value = self.chars[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>();
        match value {
            Ok(v) => self.add_token_literal(TokenType::Number, LiteralType::Num(v)),
            Err(_) => return Err(self.invalid_token(self.chars[self.start])),
        }
        Ok(())
    }

    fn process_identifier(&mut self) -> Result<(), InvalidToken> {
        while Scanner::char_is_alphanum(&self.peek()) {
            self.advance();
        }
        let value: String = self.chars[self.start..self.current].iter().collect();
        let token = match value.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "fun" => TokenType::Fun,
            _ => TokenType::Identifier,
        };
        self.add_token(token);
        Ok(())
    }

    fn process_block_comments(&mut self) -> Result<(), UnterminatedComment> {
        // store column in case the source ends in new line
        let mut prev_column = self.column;
        let mut nested = 1;
        while (nested != 0) && !self.is_at_end() {
            match self.peek() {
                '\n' => {
                    prev_column = self.column;
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
            self.column = prev_column;
            Err(self.unterminated_comment())
        } else {
            Ok(())
        }
    }

    fn invalid_token(&self, token: char) -> InvalidToken {
        InvalidToken::new(
            self.line,
            self.column,
            token.to_string(),
            Some(self.get_line_text()),
        )
    }

    fn unterminated_string(&self) -> UnterminatedString {
        UnterminatedString::new(self.line, self.column, Some(self.get_line_text()))
    }

    fn unterminated_comment(&self) -> UnterminatedComment {
        UnterminatedComment::new(self.line, self.column, Some(self.get_line_text()))
    }

    fn get_line_text(&self) -> String {
        let start_index = if self.line > 1 {
            self.current - self.column
        } else {
            0
        };
        self.chars[start_index..self.current].iter().collect()
    }

    fn advance(&mut self) -> Option<&char> {
        let current = self.current;
        self.increment_current(1);
        self.chars.get(current)
    }

    fn add_token(&mut self, r#type: TokenType) {
        let lexme = self.chars[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: None,
        });
    }

    fn add_token_literal(&mut self, r#type: TokenType, literal: LiteralType) {
        let lexme = self.chars[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: Some(literal),
        });
    }
}
