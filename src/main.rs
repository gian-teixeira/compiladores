mod lexer;
mod token;

use lexer::Lexer;
use std::env;

pub fn main() {
    let args : Vec<String> = env::args().collect();
    let tokens = Lexer::parse(&args[1].as_str());
    for token in tokens {
        println!("{token:?}");
    }
}
