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

    match config.mode() {
        cli_config::Mode::Repl => repl::repl(),
        cli_config::Mode::Python => {
            let input = std::fs::read_to_string(config.input().unwrap()).unwrap();
            let program = grammar::ProgramParser::new().parse(&input).unwrap();
            std::fs::write(
                config.output().unwrap(),
                python::transpile_to_python(&program),
            )
            .unwrap()
        }
        cli_config::Mode::Interpret => {
            let program = grammar::ProgramParser::new()
                .parse(&config.input().unwrap())
                .unwrap();
            match interpreter::interpret(&program) {
                Ok(exit_code) => println!("Program quit succesfully with: {exit_code}"),
                Err(e) => println!("An error occured during interpretation: {e:?}"),
            }
        }
    }
}
