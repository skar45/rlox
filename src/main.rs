mod errors;

use std::{
    env, fs,
    io::{self, Write},
    process::{self, ExitCode},
};

use errors::rlox_errors::{InvalidToken, UnterminatedString};

use crate::errors::rlox_errors::ScannerError;

enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

enum LiteralType {
    String(String),
    Number(f64),
}

impl std::fmt::Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct Token {
    r#type: TokenType,
    lexme: String,
    literal: Option<LiteralType>,
    line: usize,
}

impl Token {
    fn eof_token(line: usize) -> Self {
        Token {
            r#type: TokenType::Eof,
            lexme: "".to_string(),
            literal: None,
            line,
        }
    }

    fn to_string(&self) -> String {
        if let Some(literal) = &self.literal {
            return format!("{} {} {}", self.r#type, self.lexme, literal);
        } else {
            return format!("{} {}", self.r#type, self.lexme);
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

struct Scanner {
    chars: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.reserve(128);
        Scanner {
            chars: source.chars().collect(),
            tokens,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn scan_tokens(&mut self) -> Result<(), ScannerError> {
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
                                    self.current += 1
                                }
                            }
                            false => {
                                match self.char_match('*') {
                                    true => {
                                        while self.char_match('\n') {
                                            self.current += 1
                                        }
                                    }
                                    false => self.add_token(TokenType::Slash),
                                };
                            }
                        };
                    }
                    '"' => {
                        self.process_string_literal()?
                    }
                    '\t' | '\r' | ' ' => continue,

                    d => {
                        let token = d.clone();
                        return Err(self.invalid_token(&token).into());
                    }
                }
            }
        }

        self.tokens.push(Token::eof_token(self.line));
        Ok(())
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

    fn char_match(&mut self, comp: char) -> bool {
        if let Some(c) = self.chars.get(self.current) {
            if *c == comp {
                self.current += 1;
                return true;
            } else {
                return false;
            }
        }
        false
    }

    fn process_string_literal(&mut self) -> Result<(), UnterminatedString> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.unterminated_string());
        };

        self.advance();

        let value: String = self.chars[self.start + 1 .. self.current - 1].iter().collect();
        self.add_token_literal(TokenType::String, LiteralType::String(value));
        Ok(())
    }

    fn invalid_token(&self, token: &char) -> InvalidToken {
        InvalidToken::new(self.line, 0, token.to_string())
    }

    fn unterminated_string(&self) -> UnterminatedString {
        UnterminatedString::new(self.line, 0)
    }

    fn advance(&mut self) -> Option<&char> {
        let c = self.chars.get(self.current);
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, r#type: TokenType) {
        let lexme = self.chars[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: None,
            line: self.line,
        });
    }

    fn add_token_literal(&mut self, r#type: TokenType, literal: LiteralType) {
        let lexme = self.chars[self.start..self.current].into_iter().collect();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: Some(literal),
            line: self.line,
        });
    }
}

struct Rlox {
    had_error: bool,
}

impl Rlox {
    fn new() -> Self {
        Rlox { had_error: false }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        if let Err(e) = scanner.scan_tokens() {
            match e {
                ScannerError::TokenError(e) => {
                    let line = e.get_line();
                    let token = e.get_token();
                    self.report_error(line, Some(token));
                }
                ScannerError::StringError(e) => {
                    let line = e.get_line();
                    self.report_error(line, None);
                }
            }
        }

        for token in scanner.tokens.iter() {
            println!("{}", token);
        }
    }

    fn run_prompt(&mut self) {
        let mut input = String::new();
        loop {
            io::stdout()
                .write_all(b"> ")
                .expect("Unable to write to stdout!");
            io::stdout().flush().expect("Could not flush buffer!");
            io::stdin()
                .read_line(&mut input)
                .expect("Unable to parse from stdin!");
            self.run(input.clone());
            self.had_error = false;
        }
    }

    fn run_file(&mut self, path: String) {
        let content = fs::read_to_string(path);
        match content {
            Ok(s) => self.run(s),
            Err(e) => eprintln!("Error reading file: {}", e),
        }

        if self.had_error {
            process::exit(0x41);
        }
    }

    fn report_error(&mut self, line: usize, message: Option<&str>) {
        println!("[line \"{}\"] Error: {}", line, message.unwrap_or(""));
        self.had_error = true;
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut rlox = Rlox::new();
    match args.len() {
        1 => rlox.run_prompt(),
        2 => rlox.run_file(args[1].clone()),
        _ => return ExitCode::FAILURE,
    }

    return ExitCode::SUCCESS;
}
