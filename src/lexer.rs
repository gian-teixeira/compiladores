use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug)]
enum State {
    Base,
    Name,
    DoubleOperator,
    Integer,
    Float,
    Char,
    IntConst,
    FloatConst,
    CharConst,
    FormatString,
}

struct LineReader<'a> {
    source: &'a String,
    it: std::str::Chars<'a>,
    symbol: Option<char>,
    begin: usize,
    end: usize,
}

impl LineReader<'_> {
    pub fn new<'a>(source : &'a String) -> LineReader<'a> {
        let mut it = source.chars();
        let symbol = it.next();

        return LineReader {
            source: source,
            it: it,
            symbol: symbol,
            begin: 0,
            end: 0
        };
    }

    pub fn go_next(&mut self) {
        self.end += 1;
        self.symbol = self.it.next();
    }

    pub fn get_symbol(&mut self) -> char {
        return self.symbol.expect("Line Reader : no more symbols");
    }

    pub fn get_lexeme(&mut self, include_current : bool) -> &str {
        let _end = self.end + (include_current as usize);
        let lexeme = &self.source[self.begin.._end];
        return lexeme;
    }

    pub fn clear_buffer(&mut self, include_current : bool) {
        self.begin = self.end + (include_current as usize);
    }
}

struct Builder<'a> {
    nline: i32,
    tokens: &'a mut Vec<Token>,
    state: State,
    should_stop: bool,
    should_go_next: bool,
}

impl Builder<'_> {
    pub fn new<'a>(output : &'a mut Vec<Token>)
    -> Builder<'a> {
        return Builder {
            nline : 0,
            tokens : output,
            state : State::Base,
            should_stop : false,
            should_go_next : true
        };
    }

    pub fn push(&mut self, 
        line_reader : &mut LineReader,
        token_type: TokenType, 
        include_current: bool) -> bool 
    {
        let lexeme = line_reader.get_lexeme(include_current);
        match Token::new(lexeme, token_type, self.nline) {
            Some(t) => {
                //println!("{:?}", t);
                self.tokens.push(t);
                self.should_go_next = include_current;
                line_reader.clear_buffer(include_current);
            }
            None => {
                return false;
            }
        }
        return true;
    }

    pub fn feed(&mut self, 
        line : String) 
    {
        let mut line_reader = LineReader::new(&line);
        self.nline += 1;

        while line_reader.symbol.is_some() {
            let current_symbol = line_reader.symbol.unwrap();
            self.should_go_next = true;

            if line_reader.get_symbol().is_whitespace() && line_reader.begin == line_reader.end {
                line_reader.begin += 1;
                self.push(&mut line_reader, TokenType::Infer, true);
                line_reader.go_next();
                continue;
            }

            match self.state {
                State::Base => {
                    if String::from("()[]{}+*/,;!:").find(line_reader.get_symbol()).is_some() {
                        self.push(&mut line_reader, TokenType::Infer, true);
                    } else if String::from("-=><").find(line_reader.get_symbol()).is_some() {
                        self.state = State::DoubleOperator;
                    } else if line_reader.get_symbol().is_alphabetic() {
                        self.state = State::Name;
                    } else if line_reader.get_symbol().is_numeric() {
                        self.state = State::IntConst;
                    } else if line_reader.get_symbol() == '\'' {
                        self.state = State::CharConst;
                        line_reader.clear_buffer(true);
                    } else if line_reader.get_symbol() == '\"' {
                        self.state = State::FormatString;
                        line_reader.clear_buffer(true);
                    }
                }

                State::DoubleOperator => {
                    if !self.push(&mut line_reader, TokenType::Infer, true) {
                        self.push(&mut line_reader, TokenType::Infer, false);
                    }
                    self.state = State::Base;
                }

                State::Name => {
                    if !line_reader.get_symbol().is_alphanumeric() && 
                            line_reader.get_symbol() != '_' {
                        self.push(&mut line_reader, TokenType::Infer, false);
                        self.state = State::Base;
                    }
                }

                State::IntConst => {
                    if line_reader.get_symbol() == '.' {
                        self.state = State::FloatConst;
                    } else if !line_reader.get_symbol().is_numeric() {
                        self.push(&mut line_reader, TokenType::IntConst, false);
                        self.state = State::Base;
                    }
                }

                State::FloatConst => {
                    if !line_reader.get_symbol().is_numeric() {
                        self.push(&mut line_reader, TokenType::FloatConst, false);
                        self.state = State::Base;
                    }
                }

                State::CharConst => {
                    line_reader.go_next();
                    if line_reader.get_symbol() != '\'' {
                        panic!("Expected |\'|");
                    }
                    self.push(&mut line_reader, TokenType::CharConst, false);
                    line_reader.go_next();
                    line_reader.clear_buffer(false);
                    self.state = State::Base;
                    continue;
                }

                State::FormatString => {
                    if line_reader.get_symbol() == '\"' {
                        self.push(&mut line_reader, TokenType::FormatString, false);
                        line_reader.go_next();
                        line_reader.clear_buffer(false);
                        self.state = State::Base;
                        continue;
                    }
                }

                _ => {}
            }

            if self.should_go_next {
                line_reader.go_next();
            }
        }
    }
}

pub fn parse(filename: &str) 
-> Vec<Token> 
{
    let file_content = std::fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Error reading the file {0} : {1}", filename, err);
        std::process::exit(1);
    });
    let mut tokens : Vec<Token> = Vec::new();
    let mut builder = Builder::new(&mut tokens);

    for line in file_content.lines() {
        builder.feed(line.to_string());
    }

    tokens.push(Token::new("", TokenType::EOF, -1).unwrap());
    return tokens;
}
