use std::{env, fs, io::{self, Write}, process::ExitCode};

enum TokenType {
      LeftParen, RightParen, LeftBrace, RightBrace,
      Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
      Bang, BangEqual,
      Equal, EqualEqual,
      Greater, GreaterEqual,
      Less, LessEqual,
      Identifier, String, Number,
      And, Class, Else, False, Fun, For, If, Nil, Or,
      Print, Return, Super, This, True, Var, While,
      Eof
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

enum LiteralType {
    String, Number
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
    line: u32,
}

impl Token {
    fn eof_token(line: u32) -> Self {
        Token {
            r#type: TokenType::Eof,
            lexme: "".to_string(),
            literal: None,
            line,
        }
    }
    fn to_string(&self) -> String {
        return format!("{} {} {}", self.r#type, self.lexme, self.literal);
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32
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
            line: 0
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof_token(self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().expect("usize to u32 casting error")
    }

    fn scan_token(&mut self) {

    }
}

fn run(source: String) {

}

fn run_prompt() {
    let mut input = String::new();
    loop {
        io::stdout().write_all(b"> ").expect("Unable to write to stdout!");
        io::stdout().flush().expect("Could not flush buffer!");
        io::stdin().read_line(&mut input).expect("Unable to parse from stdin!");
    }
}

fn run_file(path: String) {
    let content = fs::read_to_string(path);
    match content {
        Ok(_) => println!("Running file..."),
        Err(e) => eprintln!("Error reading file: {}", e)
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => println!("Running script..."),
        _ => return ExitCode::from(64),
    }

    return ExitCode::SUCCESS;
}
