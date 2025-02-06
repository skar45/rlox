use crate::{
    ast::Expr,
    errors::parser_errors::ParserError,
    token::{LiteralValue, Token, TokenType},
};

type ParseResult = Result<Expr, ParserError>;

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

    // fn synchronize(&mut self) {
    // self.advance();
    // while !self.is_at_end() {
    // match self.previous().r#type {
    // TokenType::Semicolon => return,
    // _ => {
    // match self.peek().r#type {
    // TokenType::Class
    // | TokenType::Fun
    // | TokenType::Var
    // | TokenType::For
    // | TokenType::If
    // | TokenType::While
    // | TokenType::Print
    // | TokenType::Return => return,
    // _ => self.advance(),
    // };
    // }
    // };
    // }
    // }

    fn primary(&mut self) -> ParseResult {
        match self.advance().r#type {
            TokenType::True => Ok(Expr::literal(LiteralValue::Bool(true))),
            TokenType::False => Ok(Expr::literal(LiteralValue::Bool(false))),
            TokenType::Nil => Ok(Expr::literal(LiteralValue::Nil)),
            TokenType::Number | TokenType::String => {
                match &self.previous().literal {
                    Some(v) => Ok(Expr::literal(v.clone())),
                    None => {
                        let token = self.previous();
                        Err(ParserError::missing_literal(token.line, token.column, token.lexme.clone()))
                    }
                }
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                match self.advance().r#type {
                    TokenType::RightParen => Ok(Expr::grouping(expr?)),
                    _ => {
                        let token = self.previous();
                        let err = ParserError::missing_right_paren(token.line, token.column);
                        Err(err)
                    }
                }
            }
            _ => {
                let token = self.previous();
                let err = ParserError::invalid_expression(token.line, token.column);
                Err(err)
            }
        }
    }

    fn unary(&mut self) -> ParseResult {
        match self.peek().r#type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.advance().clone();
                let right = self.unary();
                Ok(Expr::unary(operator, right?))
            }
            _ => self.primary(),
        }
    }

    fn factor(&mut self) -> ParseResult {
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

    fn term(&mut self) -> ParseResult {
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

    fn comparison(&mut self) -> ParseResult {
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

    fn equality(&mut self) -> ParseResult {
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

    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    pub fn parse(&mut self) -> ParseResult {
        self.expression()
    }
}
