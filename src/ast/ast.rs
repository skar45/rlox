#![allow(dead_code)]
use crate::token::*;

macro_rules! parenthize_expr {
    ($name:expr, $($exprs:expr),*) => {
        {
            let mut ret = String::new();
            ret.push_str("(");
            ret.push_str($name);
            $(
                ret.push_str(" ");
                ret.push_str(&$exprs.accept());
            )*
            ret.push_str(")");
            ret
        }
    };
}

pub enum Expr {
    Literal(Literal),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
}

struct Literal {
    value: LiteralType,
}

struct Grouping {
    expression: Box<Expr>,
}

struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Expr {
    pub fn accept(&self) -> String {
        match self {
            Expr::Literal(l) => l.value.to_string(),
            Expr::Binary(b) => parenthize_expr!(&b.operator.lexme, b.left, b.right),
            Expr::Unary(u) => parenthize_expr!(&u.operator.lexme, u.right),
            Expr::Grouping(g) => parenthize_expr!("group", g.expression),
        }
    }

    pub fn literal(literal_type: LiteralType) -> Self {
        Expr::Literal(Literal {
            value: literal_type,
        })
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }
}

#[test]
pub fn liter_expr() {
    let l = Expr::literal(LiteralType::Num(2.0));
    assert_eq!("2", l.accept());
}

#[test]
pub fn group_expr() {
    let multiplication = Token {
        r#type: TokenType::Star,
        lexme: "*".to_string(),
        literal: None,
    };
    let addition = Token {
        r#type: TokenType::Plus,
        lexme: "+".to_string(),
        literal: None,
    };
    let l1 = Expr::literal(LiteralType::Num(4.0));
    let l2 = Expr::literal(LiteralType::Num(15.0));
    let l3 = Expr::literal(LiteralType::Num(9.0));
    let b1 = Expr::binary(l1, multiplication, l2);
    let g = Expr::grouping(b1);
    let b2 = Expr::binary(l3, addition, g);
    assert_eq!("(+ 9 (group (* 4 15)))", b2.accept());
}
