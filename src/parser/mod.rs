use crate::{
    ast::{expr::Expr, stmt::Stmt},
    errors::parser_errors::ParserError,
    token::{LiteralValue, Token, TokenType},
};

type ParseExprResult = Result<Expr, ParserError>;
type ParseStmtResult = Result<Stmt, ParserError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        match self.peek().r#type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            match self.previous().r#type {
                TokenType::Semicolon => return,
                _ => {
                    match self.peek().r#type {
                        TokenType::Class
                        | TokenType::Fun
                        | TokenType::Var
                        | TokenType::For
                        | TokenType::If
                        | TokenType::While
                        | TokenType::Print
                        | TokenType::Return => return,
                        _ => self.advance(),
                    };
                }
            };
        }
    }

    fn missing_paren(&self) -> ParserError {
        ParserError::missing_right_paren(self.previous().line, self.previous().column)
    }

    fn missing_literal(&self) -> ParserError {
        ParserError::missing_literal(
            self.previous().line,
            self.previous().column,
            self.previous().lexme.clone(),
        )
    }

    fn expr_error(&self, msg: &str) -> ParserError {
        ParserError::invalid_expression(
            self.previous().line,
            self.previous().column,
            msg.to_string(),
        )
    }

    fn stmt_error(&self, msg: &str) -> ParserError {
        ParserError::invalid_stmt(
            self.previous().line,
            self.previous().column,
            msg.to_string(),
        )
    }

    fn primary(&mut self) -> ParseExprResult {
        match self.advance().r#type {
            TokenType::True => Ok(Expr::literal(LiteralValue::Bool(true))),
            TokenType::False => Ok(Expr::literal(LiteralValue::Bool(false))),
            TokenType::Nil => Ok(Expr::literal(LiteralValue::Nil)),
            TokenType::Number | TokenType::String => match &self.previous().literal {
                Some(v) => Ok(Expr::literal(v.clone())),
                None => Err(self.missing_literal()),
            },
            TokenType::LeftParen => {
                let expr = self.expression();
                match self.previous().r#type {
                    TokenType::RightParen => Ok(Expr::grouping(expr?)),
                    _ => Err(self.missing_paren()),
                }
            }
            TokenType::Identifier => Ok(Expr::variable(self.previous().clone())),
            _ => Err(self.expr_error("Invalid expression.")),
        }
    }

    fn unary(&mut self) -> ParseExprResult {
        match self.peek().r#type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.advance().clone();
                let right = self.unary();
                Ok(Expr::unary(operator, right?))
            }
            _ => self.primary(),
        }
    }

    fn factor(&mut self) -> ParseExprResult {
        let mut expr = self.unary()?;
        loop {
            match self.peek().r#type {
                TokenType::Slash | TokenType::Star => {
                    let operator = self.advance().clone();
                    let right = self.unary();
                    expr = Expr::binary(expr, operator, right?);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn term(&mut self) -> ParseExprResult {
        let mut expr = self.factor()?;
        loop {
            match self.peek().r#type {
                TokenType::Plus | TokenType::Minus => {
                    let operator = self.advance().clone();
                    let right = self.factor();
                    self.advance();
                    expr = Expr::binary(expr, operator, right?);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> ParseExprResult {
        let mut expr = self.term()?;
        loop {
            match self.peek().r#type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let operator = self.advance().clone();
                    let right = self.term();
                    self.advance();
                    expr = Expr::binary(expr, operator, right?);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn equality(&mut self) -> ParseExprResult {
        let mut expr = self.comparison()?;
        loop {
            match self.peek().r#type {
                TokenType::Bang | TokenType::EqualEqual => {
                    let operator = self.advance().clone();
                    let right = self.comparison();
                    expr = Expr::binary(expr, operator, right?);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn expression(&mut self) -> ParseExprResult {
        self.equality()
    }

    fn print_statment(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        match self.advance().r#type {
            TokenType::Semicolon => Ok(Stmt::print(expr)),
            _ => Err(self.stmt_error("missing \";\" after value")),
        }
    }

    fn expression_statment(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        match self.advance().r#type {
            TokenType::Semicolon => Ok(Stmt::expression(expr)),
            _ => Err(self.stmt_error("missing \";\" after expression!")),
        }
    }

    fn statement(&mut self) -> ParseStmtResult {
        match self.peek().r#type {
            TokenType::Print => {
                self.advance();
                Ok(self.print_statment()?)
            }
            _ => self.expression_statment(),
        }
    }

    fn var_declaration(&mut self) -> ParseStmtResult {
        match self.peek().r#type {
            TokenType::Identifier => {
                let name = self.advance().clone();
                let initializer = match self.peek().r#type {
                    TokenType::Equal => {
                        self.advance();
                        Some(self.expression()?)
                    }
                    _ => None,
                };
                match self.advance().r#type {
                    TokenType::Semicolon => Ok(Stmt::var(name, initializer)),
                    _ => Err(self.stmt_error("missing \";\" after vraible name.")),
                }
            }
            _ => Err(self.stmt_error("expect a variable name")),
        }
    }

    fn declaration(&mut self) {
        let stmt_result = match self.peek().r#type {
            TokenType::Var => {
                self.advance();
                self.var_declaration()
            }
            _ => self.statement(),
        };

        if let Err(_) = stmt_result {
            self.synchronize();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }
}
