mod ast;
mod environment;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod token;

use std::{
    env, fs,
    io::{self, Write},
    process::{self, ExitCode},
};

use environment::Environment;
use errors::ReportError;
use interpreter::Interpreter;
use lexer::scanner::Scanner;
use parser::Parser;

pub struct Rlox {
    had_error: bool,
}

impl Rlox {
    pub fn new() -> Self {
        Rlox { had_error: false }
    }

    fn run(&mut self, source: String) {
        let line_text = source.split("\n").collect::<Vec<&str>>();
        // Lex
        let mut scanner = Scanner::new(source.clone());
        if let Err(error) = scanner.scan_tokens() {
            self.report_error(&error, line_text[error.get_line()]);
            process::exit(0x41);
        }
        // Parse
        let mut parser = Parser::new(scanner.tokens);
        let (parsed_stmts, parser_errors) = parser.parse();
        for error in parser_errors {
            self.report_error(&error, line_text[error.get_line()]);
        }
        if self.had_error {
            process::exit(0x41)
        };
        // Interpret
        let env = Environment::new();
        let mut interpreter = Interpreter::new(env);
        interpreter.interpret(parsed_stmts);
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

    fn report_error(&mut self, error: &impl ReportError, text: &str) {
        let column = error.get_column();
        let line = error.get_line();
        let message = error.get_msg();
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
