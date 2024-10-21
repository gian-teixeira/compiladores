mod lexer;
mod token;

use lexer::Parser;

pub fn main() {
    let file_content = std::fs::read_to_string("tmp.txt")
        .expect("Should have been able to read the file");
    let tokens = Parser::parse(&file_content);
    for token in tokens {
        print!("{token:?} ");
    }
    println!("");
}
