use std::{
    env, error::Error, fs, io::{self, Write}, process::{self, ExitCode}
};

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
    String,
    Number,
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
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        tokens.reserve(100);
        Scanner {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 0,
        }
    }


    fn scan_tokens(&mut self) -> Result<(), Box<dyn Error>> {
        let chars = self.source.clone();
        let mut chars = chars.chars();
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(c) = chars.next() {
                self.current += 1;
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
                    _ => return Err(Error::description(""))
                }
            }
        }

        self.tokens.push(Token::eof_token(self.line));
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current
            >= self
                .source
                .len()
                .try_into()
                .expect("usize to u32 casting error")
    }

    fn add_token(&mut self, r#type: TokenType) {
        let lexme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            r#type,
            lexme,
            literal: None,
            line: self.line
        });
    }

}


struct Rlox {
    had_error: bool
}

impl Rlox {
    fn new () -> Self {
        Rlox { had_error: false }
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

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
            Rlox::run(input.clone());
            self.had_error = false;
        }
    }

    fn run_file(&self, path: String) {
        let content = fs::read_to_string(path);
        match content {
            Ok(s) => Rlox::run(s),
            Err(e) => eprintln!("Error reading file: {}", e),
        }

        if self.had_error {
            process::exit(0x41);
        }
    }


    fn error(&mut self, line: i32, message: String) {
        println!("[line \"{}\"] Error: {}", line, message);
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
