mod lexer;
mod token;

use lexer::Lexer;

pub fn main() {
    let file_content = std::fs::read_to_string("tmp.txt")
        .expect("Should have been able to read the file");
    let tokens = Lexer::parse(&file_content);
    for token in tokens {
        print!("{token:?} ");
    }
    println!("");
}
