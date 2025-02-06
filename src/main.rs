mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

fn main() {
    let src = "exit();";
    let program = grammar::ProgramParser::new().parse(src).unwrap();
    println!("Program quit with: {}", interpret(&program));
}

fn interpret(program: &[ast::Statement]) -> i32 {
    fn eval_expression(expr: &ast::Expression) -> i32 {
        match expr {
            ast::Expression::Number(v) => *v,
            ast::Expression::BinaryExpression {
                left,
                operator,
                right,
            } => match operator {
                ast::BinaryOperator::Plus => eval_expression(left) + eval_expression(right),
                ast::BinaryOperator::Minus => eval_expression(left) - eval_expression(right),
                ast::BinaryOperator::Mul => eval_expression(left) * eval_expression(right),
                ast::BinaryOperator::Div => eval_expression(left) / eval_expression(right),
            },
        }
    }

    for stmt in program {
        match stmt {
            ast::Statement::Exit(v) => return eval_expression(v),
        }
    }
    0
}
