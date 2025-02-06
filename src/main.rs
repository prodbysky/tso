mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

fn main() {
    dbg!(grammar::ExpressionParser::new().parse("123 + 9").unwrap());
}
