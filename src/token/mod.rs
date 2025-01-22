#[derive(Debug)]
pub enum TokenType {
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

impl TokenType {
    pub fn match_token(token: &str) -> TokenType {
        match token {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "fun" => TokenType::Fun,
            _ => TokenType::Identifier,
        }
    }
}

#[derive(Debug)]
pub enum LiteralType {
    Str(String),
    Num(f64),
}

impl std::fmt::Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::Str(v) => write!(f, "{}", v),
            LiteralType::Num(v) => write!(f, "{}", v),
        }
    }
}

pub struct Token {
    pub r#type: TokenType,
    pub lexme: String,
    pub literal: Option<LiteralType>,
    // line: usize,
}

impl Token {
    pub fn eof_token() -> Self {
        Token {
            r#type: TokenType::Eof,
            lexme: "".to_string(),
            literal: None,
            // line,
        }
    }

    fn to_string(&self) -> String {
        if let Some(literal) = &self.literal {
            return format!("{:?} {} {:?}", self.r#type, self.lexme, literal);
        } else {
            return format!("{:?} {}", self.r#type, self.lexme);
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
