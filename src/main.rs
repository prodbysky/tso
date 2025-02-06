mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

fn main() {
    let src = "exit(123);";
    dbg!(grammar::ProgramParser::new().parse(src).unwrap());
}
