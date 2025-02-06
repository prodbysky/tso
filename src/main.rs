mod ast;

use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;

lalrpop_mod!(grammar);

fn main() {
    let src = "let x = 10; exit(x);";
    let program = grammar::ProgramParser::new().parse(src).unwrap();
    println!("Program quit with: {}", interpret(&program));
}

fn interpret(program: &[ast::Statement]) -> i32 {
    let mut vars = HashMap::new();
    fn eval_expression(expr: &ast::Expression, vars: &HashMap<String, ast::Expression>) -> i32 {
        match expr {
            ast::Expression::Number(v) => *v,
            ast::Expression::Identifier(name) => eval_expression(
                vars.get(name)
                    .expect("Tried to access an undefined variable"),
                vars,
            ),
            ast::Expression::BinaryExpression {
                left,
                operator,
                right,
            } => match operator {
                ast::BinaryOperator::Plus => {
                    eval_expression(left, vars) + eval_expression(right, vars)
                }
                ast::BinaryOperator::Minus => {
                    eval_expression(left, vars) - eval_expression(right, vars)
                }
                ast::BinaryOperator::Mul => {
                    eval_expression(left, vars) * eval_expression(right, vars)
                }
                ast::BinaryOperator::Div => {
                    eval_expression(left, vars) / eval_expression(right, vars)
                }
            },
        }
    }

    for stmt in program {
        match stmt {
            ast::Statement::Exit(v) => return eval_expression(v, &vars),
            ast::Statement::Let { name, value } => {
                vars.insert(name.to_string(), value.clone());
            }
        }
    }
    0
}
