use crate::ast;
use std::fmt::Write;

pub fn transpile_to_python(program: &[ast::Statement]) -> Result<String, std::fmt::Error> {
    let mut code = String::new();

    for stmt in program {
        match stmt {
            ast::Statement::Exit(v) => writeln!(code, "exit({})", v)?,
            ast::Statement::Let { name, value } => writeln!(code, "{} = {}", name, value)?,
            ast::Statement::Assign { name, value } => writeln!(code, "{} = {}", name, value)?,
        }
    }
    Ok(code)
}
