mod lexer;
mod token;

use lexer::Lexer;

pub fn main() {
    let tokens = Lexer::parse_file("tmp.txt");
    for token in tokens {
        println!("{token:?}");
    }
}
