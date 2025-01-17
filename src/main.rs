mod errors;
mod lexer;
mod token;

use std::{
    env,
    process::ExitCode,
    fs,
    io::{self, Write},
    process::{self},
};

use crate::errors::rlox_errors::ScannerError;
use crate::lexer::scanner::*;

pub struct Rlox {
    had_error: bool,
}

impl Rlox {
    pub fn new() -> Self {
        Rlox { had_error: false }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        if let Err(e) = scanner.scan_tokens() {
            match e {
                ScannerError::TokenError(e) => {
                    self.report_error(e.line, e.column, e.line_text.as_deref(), &e.to_string());
                }
                ScannerError::StringError(e) => {
                    self.report_error(e.line, e.column, e.line_text.as_deref(), &e.to_string());
                }
                ScannerError::CommentError(e) => {
                    self.report_error(e.line, e.column, e.line_text.as_deref(), &e.to_string());
                }
            }
            process::exit(0x41);
        }
        for token in scanner.tokens.iter() {
            println!("token {}", token.to_string());
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            io::stdout()
                .write_all(b"> ")
                .expect("Unable to write to stdout!");
            io::stdout().flush().expect("Could not flush buffer!");
            io::stdin()
                .read_line(&mut input)
                .expect("Unable to parse from stdin!");
            self.run(input);
            self.had_error = false;
        }
    }

    pub fn run_file(&mut self, path: String) {
        let content = fs::read_to_string(path);
        match content {
            Ok(s) => self.run(s),
            Err(e) => eprintln!("Error reading file: {}", e),
        }

        if self.had_error {
            process::exit(0x41);
        }
    }

    fn report_error(&mut self, line: usize, column: usize, line_text: Option<&str>, message: &str) {
        if let Some(text) = line_text {
            let l_pad = "    ";
            let mut offset = "".to_string();
            for _ in 2..column {
                offset.push(' ');
            }
            let text_lines: Vec<&str> = text.lines().collect();
            eprintln!("\x1b[37;41m Error \x1b[0m: {}", message);
            println!("{}|", l_pad);
            for i in 1..=text_lines.len() {
                let line_num = (line - text_lines.len()) + i;
                let l_pad = if line_num > 9 { "  " } else { "   " };
                println!("{}{}| {}", line_num, l_pad, text_lines[i - 1]);
            }
            println!("{}| {}^^", l_pad, offset);
        }
        self.had_error = true;
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut rlox = Rlox::new();
    match args.len() {
        1 => rlox.run_prompt(),
        2 => rlox.run_file(args[1].clone()),
        _ => {
            println!("usage: ./rlox [file]");
            return ExitCode::FAILURE;
        }
    }

    return ExitCode::SUCCESS;
}
