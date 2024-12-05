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

struct Builder<'a> {
    source: String,
    it: std::str::Chars<'a>,
    begin: usize,
    end: usize,
    nline: i32,
    symbol: char,
    tokens: Vec<Token>,
    state: State,
    should_stop: bool,
    should_go_next: bool,
}

impl Builder<'_> {
    pub fn go_next(&mut self) {
        if self.symbol == '\n' {
            self.nline += 1;
        }
        self.end += 1;
        match self.it.next() {
            Some(c) => self.symbol = c,
            None => self.should_stop = true,
        }
    }

    pub fn push(&mut self, token_type: TokenType, include_current: bool) -> bool {
        let _end = if include_current {
            self.end + 1
        } else {
            self.end
        };
        let lexeme = &self.source[self.begin.._end];
        match Token::new(lexeme, token_type, self.nline) {
            Some(t) => {
                self.tokens.push(t);
                self.begin = _end;
                self.should_go_next = include_current;
                true
            }
            None => false,
        }
    }
}

pub fn parse(filename: &str) -> Vec<Token> {
    let binding = std::fs::read_to_string(filename).unwrap_or_else(|err| {
        println!("Error reading the file {0} : {1}", filename, err);
        std::process::exit(1);
    });

    let source = binding.clone();
    let mut it = binding.as_str().chars();
    let symbol = it.next().unwrap();

    let mut builder = Builder {
        source: source,
        it: it,
        begin: 0,
        end: 0,
        nline: 1,
        symbol: symbol,
        tokens: Vec::new(),
        state: State::Base,
        should_stop: false,
        should_go_next: true,
    };

    while builder.end < builder.source.len() {
        builder.should_go_next = true;

        if builder.symbol.is_whitespace() && builder.begin == builder.end {
            builder.begin += 1;
            builder.push(TokenType::Infer, true);
            builder.go_next();
            continue;
        }

        match builder.state {
            State::Base => {
                if String::from("()[]{}+*/,;!:").find(builder.symbol).is_some() {
                    builder.push(TokenType::Infer, true);
                } else if String::from("-=><").find(builder.symbol).is_some() {
                    builder.state = State::DoubleOperator;
                } else if builder.symbol.is_alphabetic() {
                    builder.state = State::Name;
                } else if builder.symbol.is_numeric() {
                    builder.state = State::IntConst;
                } else if builder.symbol == '\'' {
                    builder.state = State::CharConst;
                } else if builder.symbol == '\"' {
                    builder.state = State::FormatString;
                }
            }

            State::DoubleOperator => {
                if !builder.push(TokenType::Infer, true) {
                    builder.push(TokenType::Infer, false);
                }
                builder.state = State::Base;
            }

            State::Name => {
                if !builder.symbol.is_alphanumeric() && builder.symbol != '_' {
                    builder.push(TokenType::Infer, false);
                    builder.state = State::Base;
                }
            }

            State::IntConst => {
                if builder.symbol == '.' {
                    builder.state = State::FloatConst;
                } else if !builder.symbol.is_numeric() {
                    builder.push(TokenType::IntConst, false);
                    builder.state = State::Base;
                }
            }

            State::FloatConst => {
                if !builder.symbol.is_numeric() {
                    builder.push(TokenType::FloatConst, false);
                    builder.state = State::Base;
                }
            }

            State::CharConst => {
                builder.go_next();
                if builder.symbol != '\'' {
                    panic!("Expected |\'|");
                }
                builder.push(TokenType::CharConst, true);
                builder.state = State::Base;
            }

            State::FormatString => {
                if builder.symbol == '\"' {
                    builder.push(TokenType::FormatString, true);
                    builder.state = State::Base;
                }
            }

            _ => {}
        }

        if builder.should_go_next {
            builder.go_next();
        }
    }

    builder
        .tokens
        .push(Token::new("", TokenType::EOF, -1).unwrap());
    builder.tokens
}
