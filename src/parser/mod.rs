use crate::{
    ast::{
        expr::Expr,
        stmt::{BreakStmt, ContStmt, ForStmtInitializer, Stmt},
    },
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

    fn missing_semicolon(&self) -> ParserError {
        ParserError::missing_semicolon(self.previous().line, self.previous().column)
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
                let expr = self.expression()?;
                match self.advance().r#type {
                    TokenType::RightParen => Ok(Expr::grouping(expr)),
                    _ => Err(self.missing_paren()),
                }
            }
            TokenType::Identifier => Ok(Expr::variable(self.previous().clone())),
            _ => Err(self.expr_error(format!("Invalid token {}", self.previous().lexme).as_str())),
        }
    }

    fn call(&mut self) -> ParseExprResult {
        let mut expr = self.primary()?;
        loop {
            if self.peek().r#type != TokenType::LeftParen {
                break;
            };
            self.advance();
            let mut args = Vec::new();
            args.reserve(255);
            loop {
                if self.peek().r#type == TokenType::RightParen {
                    break;
                }
                if args.len() > 255 {
                    return Err(self.stmt_error("Can't have more than 255 arguments"));
                }
                args.push(self.expression()?);
                if self.peek().r#type == TokenType::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
            if self.peek().r#type != TokenType::RightParen {
                return Err(self.stmt_error("missing \")\" for function call"));
            }
            let paren = self.advance().clone();
            expr = Expr::call(expr.to_string(), paren, args);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseExprResult {
        match self.peek().r#type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.advance().clone();
                let right = self.unary();
                Ok(Expr::unary(operator, right?))
            }
            _ => self.call(),
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
                    let right = self.factor()?;
                    expr = Expr::binary(expr, operator, right);
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
                    let right = self.term()?;
                    self.advance();
                    expr = Expr::binary(expr, operator, right);
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
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let operator = self.advance().clone();
                    let right = self.comparison()?;
                    expr = Expr::binary(expr, operator, right);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn and(&mut self) -> ParseExprResult {
        let mut expr = self.equality()?;
        loop {
            match self.peek().r#type {
                TokenType::And => {
                    let operator = self.advance().clone();
                    let right = self.comparison();
                    expr = Expr::logical(expr, operator, right?);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn or(&mut self) -> ParseExprResult {
        let mut expr = self.and()?;
        loop {
            match self.peek().r#type {
                TokenType::Or => {
                    let operator = self.advance().clone();
                    let right = self.comparison();
                    expr = Expr::logical(expr, operator, right?);
                }
                _ => break,
            }
        }
        return Ok(expr);
    }

    fn assignment(&mut self) -> ParseExprResult {
        let expr = self.or()?;
        match self.peek().r#type {
            TokenType::Equal => {
                self.advance();
                match expr {
                    Expr::Variable(v) => Ok(Expr::assign(v.name.clone(), self.assignment()?)),
                    _ => Err(self.expr_error("invalid var assignment")),
                }
            }
            _ => Ok(expr),
        }
    }

    fn expression(&mut self) -> ParseExprResult {
        self.assignment()
    }

    fn print_statment(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        match self.advance().r#type {
            TokenType::Semicolon => Ok(Stmt::print(expr)),
            _ => Err(self.missing_semicolon()),
        }
    }

    fn expression_statement(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        match self.advance().r#type {
            TokenType::Semicolon => Ok(Stmt::expression(expr)),
            _ => Err(self.missing_semicolon()),
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            match self.peek().r#type {
                TokenType::RightBrace => {
                    self.advance();
                    break;
                }
                _ => {
                    statements.push(self.declaration()?);
                }
            }
        }

        if self.previous().r#type != TokenType::RightBrace {
            Err(self.expr_error("expected \"}\" after block"))
        } else {
            Ok(statements)
        }
    }

    fn if_statment(&mut self) -> ParseStmtResult {
        if self.peek().r#type != TokenType::LeftParen {
            return Err(self.stmt_error("missing \"(\""));
        }
        self.advance();
        let condition = self.expression()?;
        if self.peek().r#type != TokenType::RightParen {
            return Err(self.stmt_error("missing \")\""));
        }
        self.advance();
        let then_branch = self.statement()?;
        let else_branch = match self.previous().r#type {
            TokenType::Else => {
                self.advance();
                let ret = Some(self.statement()?);
                self.advance();
                ret
            }
            _ => None,
        };
        Ok(Stmt::if_stmt(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> ParseStmtResult {
        match self.peek().r#type {
            TokenType::LeftParen => {
                self.advance();
                let condition = self.expression()?;
                match self.peek().r#type {
                    TokenType::RightParen => {
                        self.advance();
                        let body = self.statement()?;
                        Ok(Stmt::while_stmt(condition, body))
                    }
                    _ => Err(self.stmt_error("missing \")\"")),
                }
            }
            _ => Err(self.stmt_error("missing \"(\"")),
        }
    }

    fn for_statement(&mut self) -> ParseStmtResult {
        if self.advance().r#type != TokenType::LeftParen {
            return Err(self.stmt_error("missing \"(\" after \"for\""));
        }

        let initializer = match self.peek().r#type {
            TokenType::Var => {
                self.advance();
                match self.var_declaration()? {
                    Stmt::Var(v) => Some(ForStmtInitializer::VarDecl(v)),
                    _ => return Err(self.stmt_error("invalid for loop initialization")),
                }
            }
            TokenType::Semicolon => {
                self.advance();
                None
            }
            _ => match self.expression_statement()? {
                Stmt::Expresssion(e) => Some(ForStmtInitializer::ExprStmt(e)),
                _ => return Err(self.stmt_error("invalid for loop initialization")),
            },
        };

        let condition = match self.peek().r#type {
            TokenType::Semicolon => None,
            _ => Some(self.expression()?),
        };

        if self.previous().r#type != TokenType::Semicolon {
            return Err(self.stmt_error("missing \";\" after loop condition"));
        };

        let afterthought = match self.peek().r#type {
            TokenType::RightParen => None,
            _ => Some(self.expression()?),
        };

        if self.advance().r#type != TokenType::RightParen {
            return Err(self.stmt_error("missing \")\" after loop construct"));
        };

        let body = self.statement()?;

        Ok(Stmt::for_stmt(body, initializer, condition, afterthought))
    }

    fn fn_statement(&mut self) -> ParseStmtResult {
        if self.peek().r#type != TokenType::Identifier {
            return Err(self.stmt_error("expected function name"));
        }
        let name = self.advance().clone();
        if self.peek().r#type != TokenType::LeftParen {
            return Err(self.stmt_error("expected \"(\" after function name"));
        }
        self.advance();
        let mut params = Vec::new();
        loop {
            let token = self.advance();
            match token.r#type {
                TokenType::RightParen => break,
                TokenType::Comma => continue,
                TokenType::Identifier => {
                    if params.len() < 256 {
                        params.push(token.clone());
                    } else {
                        return Err(self.stmt_error("cannot have more than 255 arguments"));
                    }
                }
                _ => return Err(self.stmt_error("invalid function param")),
            }
        }
        if self.peek().r#type != TokenType::LeftBrace {
            return Err(self.stmt_error("expected \"{\" before function body"));
        }
        self.advance();
        let body = self.block()?;
        Ok(Stmt::fn_stmt(name, params, body))
    }

    fn return_statement(&mut self) -> ParseStmtResult {
        let keyword = self.previous().clone();
        let mut value = None;
        if self.peek().r#type != TokenType::Semicolon {
            value = Some(self.expression()?);
        }
        if self.peek().r#type != TokenType::Semicolon {
            return Err(self.missing_semicolon());
        }
        self.advance();
        Ok(Stmt::return_stmt(keyword, value))
    }

    fn break_statement(&mut self) -> ParseStmtResult {
        if self.peek().r#type != TokenType::Semicolon {
            return Err(self.missing_semicolon());
        }
        self.advance();
        Ok(Stmt::BreakStmt(BreakStmt {}))
    }

    fn continue_statement(&mut self) -> ParseStmtResult {
        if self.peek().r#type != TokenType::Semicolon {
            return Err(self.missing_semicolon());
        }
        self.advance();
        Ok(Stmt::ContStmt(ContStmt {}))
    }

    fn statement(&mut self) -> ParseStmtResult {
        match self.peek().r#type {
            TokenType::Print => {
                self.advance();
                Ok(self.print_statment()?)
            }
            TokenType::LeftBrace => {
                self.advance();
                Ok(Stmt::block(self.block()?))
            }
            TokenType::If => {
                self.advance();
                Ok(self.if_statment()?)
            }
            TokenType::While => {
                self.advance();
                Ok(self.while_statement()?)
            }
            TokenType::For => {
                self.advance();
                Ok(self.for_statement()?)
            }
            TokenType::Fun => {
                self.advance();
                Ok(self.fn_statement()?)
            }
            TokenType::Return => {
                self.advance();
                Ok(self.return_statement()?)
            }
            TokenType::Break => {
                self.advance();
                Ok(self.break_statement()?)
            }
            TokenType::Continue => {
                self.advance();
                Ok(self.continue_statement()?)
            }
            _ => self.expression_statement(),
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
                    _ => Err(self.missing_semicolon()),
                }
            }
            _ => Err(self.stmt_error("expected a variable name")),
        }
    }

    fn declaration(&mut self) -> ParseStmtResult {
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

        return stmt_result;
    }

    pub fn parse(&mut self) -> (Vec<Stmt>, Vec<ParserError>) {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut parse_errors: Vec<ParserError> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(v) => statements.push(v),
                Err(e) => parse_errors.push(e),
            }
        }
        (statements, parse_errors)
    }
}
