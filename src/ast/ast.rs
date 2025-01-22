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

enum Expr {
    Literal(Literal),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
}

impl Expr {
    fn accept(&self) -> String {
        match self {
            Expr::Literal(l) => l.value.to_string(),
            Expr::Binary(b) => parenthize_expr!(&b.operator.lexme, b.left, b.right),
            Expr::Unary(u) => parenthize_expr!(&u.operator.lexme, u.right),
            Expr::Grouping(g) => parenthize_expr!("group", g.expression),
        }
    }
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

#[test]
pub fn liter_expr() {
    let l = Expr::Literal(Literal {
        value: LiteralType::Num(2.0),
    });
    assert_eq!("2", l.accept());
}

#[test]
pub fn group_expr() {
    let l1 = Expr::Literal(Literal {
        value: LiteralType::Num(4.0),
    });
    let l2 = Expr::Literal(Literal {
        value: LiteralType::Num(15.0),
    });
    let l3 = Expr::Literal(Literal {
        value: LiteralType::Num(9.0),
    });
    let b1 = Expr::Binary(Binary {
        left: Box::new(l1),
        operator: Token {
            r#type: TokenType::Star,
            lexme: "*".to_string(),
            literal: None,
        },
        right: Box::new(l2),
    });
    let g = Expr::Grouping(Grouping {
        expression: Box::new(b1),
    });
    let b2 = Expr::Binary(Binary {
        left: Box::new(l3),
        operator: Token {
            r#type: TokenType::Plus,
            lexme: "+".to_string(),
            literal: None,
        },
        right: Box::new(g),
    });
    assert_eq!("(+ 9 (group (* 4 15)))", b2.accept());
}
