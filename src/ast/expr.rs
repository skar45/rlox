use crate::token::{LiteralValue, Token, TokenType};

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
}

#[derive(Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub args: Vec<Expr>,
}

impl Expr {
    pub fn accept(&self) -> String {
        match self {
            Expr::Literal(l) => l.value.to_string(),
            Expr::Binary(b) => parenthize_expr!(&b.operator.lexme, b.left, b.right),
            Expr::Unary(u) => parenthize_expr!(&u.operator.lexme, u.right),
            Expr::Grouping(g) => parenthize_expr!("group", g.expression),
            Expr::Variable(v) => v.name.lexme.clone(),
            Expr::Assign(a) => parenthize_expr!(&a.name.lexme, a.value),
            Expr::Logical(l) => parenthize_expr!(&l.operator.lexme, l.left, l.right),
            Expr::Call(_c) => todo!(),
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

    pub fn variable(name: Token) -> Self {
        Expr::Variable(Variable { name })
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign(Assign {
            name,
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

    pub fn call(callee: Expr, paren: Token, args: Vec<Expr>) -> Self {
        Expr::Call(Call {
            callee: Box::new(callee),
            paren,
            args,
        })
    }
}

#[test]
pub fn liter_expr() {
    let l = Expr::literal(LiteralValue::Num(2.0));
    assert_eq!("2", l.accept());
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
    assert_eq!("(+ 9 (group (* 4 15)))", b2.accept());
}
