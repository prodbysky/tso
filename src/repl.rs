use crate::{grammar, interpreter};

pub fn repl() {
    let config = rustyline::Config::builder()
        .tab_stop(4)
        .bell_style(rustyline::config::BellStyle::Visible)
        .indent_size(4)
        .completion_type(rustyline::CompletionType::Circular)
        .build();
    let mut rl = match rustyline::DefaultEditor::with_config(config) {
        Ok(rl) => rl,
        Err(e) => {
            println!("Failed to initialize repl: {e:?}");
            return;
        }
    };
    let mut interpreter = interpreter::Interpreter::new();

    let stmt_parser = grammar::ProgramParser::new();

    loop {
        let line = rl.readline("]> ");
        match line {
            Ok(line) => match line.as_str() {
                ".exit" => return,
                ".state" => println!("{:#?}", interpreter.state()),
                ".help" => {
                    println!("Tso repl help");
                    println!("  .exit - quit the repl");
                    println!("  .state - print the current state of the repl");
                    println!("  .help - print this help message");
                }
                _ => {
                    match stmt_parser.parse(&line) {
                        Ok(stmt) => {
                            for st in stmt {
                                match interpreter.interpret_single_statement(&st) {
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
