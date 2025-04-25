use crate::class::RloxInstance;

#[derive(Debug, PartialEq, Clone)]
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
    Break,
    Continue,
    Eof,
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
            "break" => TokenType::Break,
            "continue" => TokenType::Break,
            _ => TokenType::Identifier,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RloxValue {
    Str(String),
    Num(f64),
    Bool(bool),
    Instance(RloxInstance),
    Nil,
}

impl std::fmt::Display for RloxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RloxValue::Str(v) => write!(f, "{}", v),
            RloxValue::Num(v) => write!(f, "{}", v),
            RloxValue::Bool(v) => write!(f, "{}", v),
            RloxValue::Nil => write!(f, "Nil"),
            RloxValue::Instance(i) => write!(f, "{}", i)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexme: String,
    pub literal: Option<RloxValue>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn eof_token(line: usize) -> Self {
        Token {
            r#type: TokenType::Eof,
            lexme: "".to_string(),
            literal: None,
            column: 0,
            line,
        }
    }

    fn to_string(&self) -> String {
        match &self.literal {
            Some(l) => format!("{:?} {} {:?}", self.r#type, self.lexme, l),
            None => format!("{:?} {}", self.r#type, self.lexme),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
