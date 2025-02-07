use crate::{grammar, interpreter};
use std::collections::HashMap;

pub fn repl() {
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
                                match interpreter::interpret_single_statement(&st, &mut vars) {
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
