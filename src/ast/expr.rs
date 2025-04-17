use std::usize;

use crate::token::*;

type ExprId = usize;

macro_rules! parenthize_expr {
    ($name:expr, $($exprs:expr),*) => {
        {
            let mut ret = String::new();
            ret.push_str("(");
            ret.push_str($name);
            $(
                ret.push_str(" ");
                ret.push_str(&$exprs.to_string());
            )*
            ret.push_str(")");
            ret
        }
    };
}

#[derive(Clone)]
pub enum Expr {
    Literal(Literal),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Variable {
    pub name: Token,
    pub id: ExprId
}

impl Variable {
    pub fn to_string(&self) -> String {
        self.name.lexme.to_string()
    }
}

#[derive(Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
    pub id: ExprId
}

impl Assign {
    pub fn to_string(&self) -> String {
        parenthize_expr!(&self.name.lexme, self.value)
    }
}

#[derive(Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Clone)]
pub struct Call {
    pub callee: String,
    pub paren: Token,
    pub args: Vec<Expr>,
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Literal(l) => l.value.to_string(),
            Expr::Binary(b) => parenthize_expr!(&b.operator.lexme, b.left, b.right),
            Expr::Unary(u) => parenthize_expr!(&u.operator.lexme, u.right),
            Expr::Grouping(g) => parenthize_expr!("group", g.expression),
            Expr::Variable(v) => v.to_string(),
            Expr::Assign(a) => a.to_string(),
            Expr::Logical(l) => parenthize_expr!(&l.operator.lexme, l.left, l.right),
            Expr::Call(c) => parenthize_expr!("fun", c.callee),
        }
    }

    pub fn literal(literal_type: LiteralValue) -> Self {
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

    pub fn variable(name: Token, id: usize) -> Self {
        Expr::Variable(Variable { name, id })
    }

    pub fn assign(name: Token, value: Expr, id: usize) -> Self {
        Expr::Assign(Assign {
            name,
            id,
            value: Box::new(value),
        })
    }

    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Logical(Logical {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn call(callee: String, paren: Token, args: Vec<Expr>) -> Self {
        Expr::Call(Call {
            callee,
            paren,
            args,
        })
    }
}

#[test]
pub fn liter_expr() {
    let l = Expr::literal(LiteralValue::Num(2.0));
    assert_eq!("2", l.to_string());
}

#[test]
pub fn group_expr() {
    let multiplication = Token {
        column: 0,
        line: 0,
        r#type: TokenType::Star,
        lexme: "*".to_string(),
        literal: None,
    };
    let addition = Token {
        column: 0,
        line: 0,
        r#type: TokenType::Plus,
        lexme: "+".to_string(),
        literal: None,
    };
    let l1 = Expr::literal(LiteralValue::Num(4.0));
    let l2 = Expr::literal(LiteralValue::Num(15.0));
    let l3 = Expr::literal(LiteralValue::Num(9.0));
    let b1 = Expr::binary(l1, multiplication, l2);
    let g = Expr::grouping(b1);
    let b2 = Expr::binary(l3, addition, g);
    assert_eq!("(+ 9 (group (* 4 15)))", b2.to_string());
}
