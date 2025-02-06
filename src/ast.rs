#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    Identifier(String),
    BinaryExpression {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Div,
    Mul,
}

pub type Program = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    Exit(Expression),
    Let { name: String, value: Expression },
}
