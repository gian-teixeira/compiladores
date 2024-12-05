mod grammar;
mod lexer;
mod token;

use crate::grammar::Grammar;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tokens = lexer::parse(&args[1].as_str());

    Grammar::analyze(&tokens);
}
