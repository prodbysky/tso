use crate::ast;
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum InterpretationError {
    #[error("Tried to access an undefined variable: {name}")]
    UndefinedVariable { name: String },
}

pub type InterpretationResult<T> = Result<T, InterpretationError>;

pub struct Interpreter {
    vars: HashMap<String, ast::Expression>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn interpret(&mut self, program: &[ast::Statement]) -> InterpretationResult<i32> {
        for stmt in program {
            match self.interpret_single_statement(&stmt) {
                Ok(None) => {}
                Ok(Some(exit_code)) => return Ok(exit_code),
                Err(e) => return Err(e),
            }
        }
        Ok(0)
    }
    pub fn interpret_single_statement(
        &mut self,
        stmt: &ast::Statement,
    ) -> InterpretationResult<Option<i32>> {
        match stmt {
            ast::Statement::Exit(v) => return Ok(Some(self.evaluate_expression(v)?)),
            ast::Statement::Let { name, value } => {
                self.vars.insert(name.to_string(), value.clone());
            }
            ast::Statement::Assign { name, value } => {
                if self.vars.contains_key(name) {
                    self.vars.insert(name.to_string(), value.clone());
                } else {
                    return Err(InterpretationError::UndefinedVariable {
                        name: name.to_string(),
                    });
                }
            }
        }
        Ok(None)
    }

    pub fn state(&self) -> &HashMap<String, ast::Expression> {
        &self.vars
    }

    fn evaluate_expression(&self, expr: &ast::Expression) -> InterpretationResult<i32> {
        match expr {
            ast::Expression::Number(v) => Ok(*v),
            ast::Expression::Identifier(name) => {
                self.evaluate_expression(match self.vars.get(name) {
                    Some(value) => value,
                    None => {
                        return Err(InterpretationError::UndefinedVariable {
                            name: name.to_string(),
                        })
                    }
                })
            }
            ast::Expression::BinaryExpression {
                left,
                operator,
                right,
            } => match operator {
                ast::BinaryOperator::Plus => {
                    Ok(self.evaluate_expression(left)? + self.evaluate_expression(right)?)
                }
                ast::BinaryOperator::Minus => {
                    Ok(self.evaluate_expression(left)? - self.evaluate_expression(right)?)
                }
                ast::BinaryOperator::Mul => {
                    Ok(self.evaluate_expression(left)? * self.evaluate_expression(right)?)
                }
                ast::BinaryOperator::Div => {
                    Ok(self.evaluate_expression(left)? / self.evaluate_expression(right)?)
                }
            },
        }
    }
}
