mod ast;
mod cli_config;
mod interpreter;
mod python;
mod repl;

use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

fn main() {
    let config = cli_config::Config::parse();

    match (config.mode(), config.input(), config.output()) {
        (cli_config::Mode::Repl, _, _) => repl::repl(),
        (cli_config::Mode::Python, Some(input), Some(output)) => {
            let input = std::fs::read_to_string(input).unwrap();
            let program = grammar::ProgramParser::new().parse(&input).unwrap();
            std::fs::write(output, python::transpile_to_python(&program).unwrap()).unwrap()
        }
        (cli_config::Mode::Interpret, Some(input), _) => {
            let program = grammar::ProgramParser::new().parse(input).unwrap();
            match interpreter::interpret(&program) {
                Ok(exit_code) => println!("Program quit succesfully with: {exit_code}"),
                Err(e) => println!("An error occured during interpretation: {e:?}"),
            }
        }
        _ => unreachable!(),
    }
}
