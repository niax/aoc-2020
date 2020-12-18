#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(a, b) => a.evaluate() + b.evaluate(),
            Expr::Mul(a, b) => a.evaluate() * b.evaluate(),
        }
    }
}
