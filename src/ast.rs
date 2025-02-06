#[derive(Debug)]
pub enum Expression {
    Number(i32),
    BinaryExpression {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Div,
    Mul,
}
