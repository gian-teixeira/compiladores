mod lexer;
mod token;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tokens = lexer::parse(&args[1].as_str());
    for token in tokens {
        println!("{token:?}");
    }
}
