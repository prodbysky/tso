mod ast;

use clap::Parser;
use lalrpop_util::lalrpop_mod;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Write;
use thiserror::Error;

lalrpop_mod!(grammar);

fn main() {
    let config = Config::parse();

    match config.mode {
        Mode::Repl => repl(),
        Mode::Python => {
            let input = std::fs::read_to_string(config.input_file.unwrap()).unwrap();
            let program = grammar::ProgramParser::new().parse(&input).unwrap();
            std::fs::write(config.output_file.unwrap(), transpile_to_python(&program)).unwrap()
        }
        Mode::Interpret => {
            let program = grammar::ProgramParser::new()
                .parse(&config.input_file.unwrap())
                .unwrap();
            match interpret(&program) {
                Ok(exit_code) => println!("Program quit succesfully with: {exit_code}"),
                Err(e) => println!("An error occured during interpretation: {e:?}"),
            }
            println!("Program quit succesfully with: {:?}", interpret(&program));
        }
    }
}

#[derive(Debug, Default, Clone, clap::ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
enum Mode {
    /// Transpile to python
    Python,
    /// Run the interpreter
    Interpret,
    /// Run the REPL
    #[default]
    Repl,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Repl => write!(f, "repl"),
            Self::Python => write!(f, "python"),
            Self::Interpret => write!(f, "interpret"),
        }
    }
}

/// The single executable tso language compiler/interpreter/transpiler
#[derive(Parser, Debug)]
struct Config {
    /// The mode of the program
    #[arg(default_value_t=Mode::Repl)]
    mode: Mode,

    // The input file to run/transpile
    #[arg(
        short,
        required_if_eq("mode", "python"),
        required_if_eq("mode", "interpret")
    )]
    input_file: Option<String>,

    // The output file to write to when python mode is selected
    #[arg(short, required_if_eq("mode", "python"))]
    output_file: Option<String>,
}

#[derive(Debug, Error)]
enum InterpretationError {
    #[error("Tried to access an undefined variable: {name}")]
    UndefinedVariable { name: String },
}

type InterpretationResult<T> = Result<T, InterpretationError>;

fn repl() {
    let config = rustyline::Config::builder()
        .tab_stop(4)
        .bell_style(rustyline::config::BellStyle::Visible)
        .indent_size(4)
        .completion_type(rustyline::CompletionType::Circular)
        .build();
    let mut rl = rustyline::DefaultEditor::with_config(config).unwrap();
    let mut vars = HashMap::new();

    let stmt_parser = grammar::ProgramParser::new();

    loop {
        let line = rl.readline("]> ");
        match line {
            Ok(line) => match line.as_str() {
                ".exit" => return,
                ".state" => println!("{:#?}", vars),
                _ => {
                    match stmt_parser.parse(&line) {
                        Ok(stmt) => {
                            for st in stmt {
                                match interpret_single_statement(&st, &mut vars) {
                                    Err(e) => println!("An interpretation error occured: {e:?}"),
                                    Ok(_) => {}
                                }
                            }
                        }
                        Err(_) => println!("Invalid input statement"),
                    };
                }
            },
            Err(rustyline::error::ReadlineError::Io(err)) => {
                println!("An io error occured: {err:?}")
            }
            Err(
                rustyline::error::ReadlineError::Eof
                | rustyline::error::ReadlineError::Errno(_)
                | rustyline::error::ReadlineError::Interrupted,
            ) => {
                break;
            }
            e => {
                println!("Something else bad occured: {e:?}")
            }
        }
    }
}

fn interpret_single_statement(
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

fn interpret(program: &[ast::Statement]) -> InterpretationResult<i32> {
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
