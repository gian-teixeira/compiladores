mod grammar;
mod lexer;
mod token;
mod error;

use crate::grammar::Grammar;
use std::io::Write;

pub fn main() {
    error::init();

    let args: Vec<String> = std::env::args().collect();
    let tokens = lexer::parse(&args[1].as_str());

    let mut file = std::fs::File::create("tokens").unwrap();
    for token in &tokens {
        writeln!(&mut file, "{:?}", token);
    }
    
    Grammar::analyze(&tokens);
}
