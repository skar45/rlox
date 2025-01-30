use crate::{
    ast::ast::Expr, errors::parser_errors::ParserError, token::{LiteralType, Token, TokenType}
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

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            token_type == self.peek().r#type
        }
    }

    fn primary(&mut self) -> ParseResult {
        match self.advance().r#type {
            TokenType::True => Ok(Expr::literal(LiteralType::Bool(true))),
            TokenType::False => Ok(Expr::literal(LiteralType::Bool(false))),
            TokenType::Nil => Ok(Expr::literal(LiteralType::Nil)),
            TokenType::Number | TokenType::String => {
                let value = Option::expect(
                    self.previous().literal.as_ref(),
                    "literal value not defined",
                );
                Ok(Expr::literal(value.clone()))
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                match self.advance().r#type {
                    TokenType::RightParen => Ok(Expr::grouping(expr?)),
                    _ => {
                        let token = self.previous();
                        let err = ParserError::missing_right_paren(token.line, token.column);
                        Err(err)
                    },
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
        match self.advance().r#type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.previous().clone();
                let right = self.unary();
                Ok(Expr::unary(operator, right?))
            }
            _ => self.primary(),
        }
    }

    fn factor(&mut self) -> ParseResult {
        let mut expr = self.unary()?;
        loop {
            match self.advance().r#type {
                TokenType::Slash | TokenType::Star => {
                    let operator = self.previous().clone();
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
            match self.advance().r#type {
                TokenType::Plus | TokenType::Minus => {
                    let operator = self.previous().clone();
                    let right = self.factor();
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
            match self.advance().r#type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let operator = self.previous().clone();
                    let right = self.term();
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
            match self.advance().r#type {
                TokenType::Bang | TokenType::EqualEqual => {
                    let operator = self.previous().clone();
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
