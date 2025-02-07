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
    Assign { name: String, value: Expression },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(v) => write!(f, "{}", v),
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => match operator {
                BinaryOperator::Plus => {
                    write!(f, "{} + {}", left, right)
                }
                BinaryOperator::Minus => {
                    write!(f, "{} - {}", left, right)
                }
                BinaryOperator::Mul => {
                    write!(f, "{} * {}", left, right)
                }
                BinaryOperator::Div => {
                    write!(f, "{} / {}", left, right)
                }
            },
        }
    }
}
