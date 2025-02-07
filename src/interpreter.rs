use crate::ast;
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum InterpretationError {
    #[error("Tried to access an undefined variable: {name}")]
    UndefinedVariable { name: String },
}

pub type InterpretationResult<T> = Result<T, InterpretationError>;

pub fn interpret_single_statement(
    stmt: &ast::Statement,
    vars: &mut HashMap<String, ast::Expression>,
) -> InterpretationResult<Option<i32>> {
    fn eval_expression(
        expr: &ast::Expression,
        vars: &HashMap<String, ast::Expression>,
    ) -> InterpretationResult<i32> {
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

    match stmt {
        ast::Statement::Exit(v) => return Ok(Some(eval_expression(v, &vars)?)),
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
    Ok(None)
}

pub fn interpret(program: &[ast::Statement]) -> InterpretationResult<i32> {
    let mut vars = HashMap::new();

    for stmt in program {
        match interpret_single_statement(&stmt, &mut vars) {
            Ok(None) => {}
            Ok(Some(exit_code)) => return Ok(exit_code),
            Err(e) => return Err(e),
        }
    }
    Ok(0)
}
