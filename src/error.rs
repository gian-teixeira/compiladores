use std::io::Write;

pub fn init() {
    let mut file = std::fs::File::create("log").unwrap();
}

pub fn raise(error_string : String) {
    let mut file = std::fs::File::create("log").unwrap();
    writeln!(&mut file, "{error_string}").unwrap();
    println!("Error raised. Check the logs");
    std::process::exit(1);
}
