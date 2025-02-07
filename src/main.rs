mod ast;

use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::fmt::Write;
use thiserror::Error;

lalrpop_mod!(grammar);

fn main() {
    let src = "let x = 10; x = 5 + 4; exit(x);";
    let program = grammar::ProgramParser::new().parse(src).unwrap();
    println!("Program quit with: {}", interpret(&program).unwrap());
    let python = transpile_to_python(&program);
    std::fs::write("transpiled.py", python).unwrap();
}

#[derive(Debug, Error)]
enum InterpretationError {
    #[error("Tried to access an undefined variable: {name}")]
    UndefinedVariable { name: String },
}

type InterpretationResult = Result<i32, InterpretationError>;

fn interpret(program: &[ast::Statement]) -> InterpretationResult {
    let mut vars = HashMap::new();
    fn eval_expression(
        expr: &ast::Expression,
        vars: &HashMap<String, ast::Expression>,
    ) -> InterpretationResult {
        match expr {
            ast::Expression::Number(v) => Ok(*v),
            ast::Expression::Identifier(name) => eval_expression(
                match vars.get(name) {
                    Some(value) => value,
                    None => {
                        return Err(InterpretationError::UndefinedVariable {
                            name: name.to_string(),
                        })
                    }
                },
                vars,
            ),
            ast::Expression::BinaryExpression {
                left,
                operator,
                right,
            } => match operator {
                ast::BinaryOperator::Plus => {
                    Ok(eval_expression(left, vars)? + eval_expression(right, vars)?)
                }
                ast::BinaryOperator::Minus => {
                    Ok(eval_expression(left, vars)? - eval_expression(right, vars)?)
                }
                ast::BinaryOperator::Mul => {
                    Ok(eval_expression(left, vars)? * eval_expression(right, vars)?)
                }
                ast::BinaryOperator::Div => {
                    Ok(eval_expression(left, vars)? / eval_expression(right, vars)?)
                }
            },
        }
    }

    for stmt in program {
        match stmt {
            ast::Statement::Exit(v) => return Ok(eval_expression(v, &vars)?),
            ast::Statement::Let { name, value } => {
                vars.insert(name.to_string(), value.clone());
            }
            ast::Statement::Assign { name, value } => {
                if vars.contains_key(name) {
                    vars.insert(name.to_string(), value.clone());
                } else {
                    return Err(InterpretationError::UndefinedVariable {
                        name: name.to_string(),
                    });
                }
            }
        }
    }
    Ok(0)
}

impl std::fmt::Display for ast::Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ast::Expression::Number(v) => write!(f, "{}", v),
            ast::Expression::Identifier(ident) => write!(f, "{}", ident),
            ast::Expression::BinaryExpression {
                left,
                operator,
                right,
            } => match operator {
                ast::BinaryOperator::Plus => {
                    write!(f, "{} + {}", left, right)
                }
                ast::BinaryOperator::Minus => {
                    write!(f, "{} - {}", left, right)
                }
                ast::BinaryOperator::Mul => {
                    write!(f, "{} * {}", left, right)
                }
                ast::BinaryOperator::Div => {
                    write!(f, "{} / {}", left, right)
                }
            },
        }
    }
}

fn transpile_to_python(program: &[ast::Statement]) -> String {
    let mut code = String::new();

    for stmt in program {
        match stmt {
            ast::Statement::Exit(v) => writeln!(code, "exit({})", v).unwrap(),
            ast::Statement::Let { name, value } | ast::Statement::Assign { name, value } => {
                writeln!(code, "{} = {}", name, value).unwrap()
            }
        }
    }
    code
}
