use core::panic;

use crate::{
    ast::ast::Expr,
    token::{LiteralType, Token, TokenType},
};

struct Parser {
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


    fn primary(&mut self) -> Expr {
        match self.advance().r#type {
            TokenType::True => Expr::literal(LiteralType::Bool(true)),
            TokenType::False => Expr::literal(LiteralType::Bool(false)),
            TokenType::Nil => Expr::literal(LiteralType::Nil),
            TokenType::Number | TokenType::String => {
                let value = Option::expect(self.previous().literal.as_ref(), "literal value not defined");
                Expr::literal(value.clone())
            },
            TokenType::LeftParen => {
                let expr = self.expression();
                match self.advance().r#type {
                    TokenType::RightParen => {
                        Expr::grouping(expr)
                    },
                     _ => panic!("")
                }
            },
            _ => panic!("token cannot be parsed as a primary expression")
        }
    }

    fn unary(&mut self) -> Expr {
        match self.advance().r#type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.previous().clone();
                let right = self.unary();
                Expr::unary(operator, right)
            },
            _ => self.primary()
        }
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        loop {
            match self.advance().r#type {
                TokenType::Slash | TokenType::Star => {
                    let operator = self.previous().clone();
                    let right = self.unary();
                    expr = Expr::binary(expr, operator, right);
                },
                _ => break
            }
        }
        return expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        loop {
            match self.advance().r#type {
                TokenType::Plus | TokenType::Minus => {
                    let operator = self.previous().clone();
                    let right = self.factor();
                    expr = Expr::binary(expr, operator, right);
                },
                _ => break
            }
        }
        return expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        loop {
            match self.advance().r#type {
                TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType:: LessEqual => {
                    let operator = self.previous().clone();
                    let right = self.term();
                    expr = Expr::binary(expr, operator, right);
                },
                _ => break
            }
        }
        return expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        loop {
            match self.advance().r#type {
                TokenType::Bang | TokenType::EqualEqual => {
                    let operator = self.previous().clone();
                    let right = self.comparison();
                    expr = Expr::binary(expr, operator, right);
                },
                _ => break
            }
        }
        return expr
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

}
