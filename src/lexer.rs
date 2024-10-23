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
    CharConst
}
    
pub struct Lexer<'a> {
    source : String,
    it : std::str::Chars<'a>,
    begin : usize,
    end : usize,
    nline : i32,
    symbol : char,
    tokens : Vec<Token>,
    state : State,
    should_stop : bool,
    should_go_next : bool
}

impl Lexer<'_> {
    pub fn go_next(&mut self)
    {
        if self.symbol == '\n' {
            self.nline += 1;
        }
        self.end += 1;
        match self.it.next() {
            Some(c) => self.symbol = c,
            None => self.should_stop = true
        }
    }

    pub fn push(
        &mut self,
        token_type : TokenType,
        include_current : bool)
    -> bool
    {
        let _end = if include_current { self.end+1 } else { self.end };
        let lexeme = &self.source[self.begin.._end];
        match Token::new(lexeme, token_type, self.nline) {
            Some(t) => {
                self.tokens.push(t);
                self.begin = _end;
                self.should_go_next = include_current;
                true
            },
            None => false
        }
    }

    pub fn parse(filename : &str)
    -> Vec<Token>
    {
        let binding = std::fs::read_to_string(filename)
            .unwrap_or_else(|err| {
                println!("Error reading the file {0} : {1}", filename, err);
                std::process::exit(1);
            });

        let source = binding.clone();
        let mut it = binding.as_str().chars();
        let symbol = it.next().unwrap();

        let mut lexer = Lexer {
            source : source,
            it : it,
            begin : 0,
            end : 0,
            nline : 1,
            symbol : symbol,
            tokens : Vec::new(),
            state : State::Base,
            should_stop : false,
            should_go_next : true
        };

        while lexer.end < lexer.source.len() {
            lexer.should_go_next = true;
            
            if lexer.symbol.is_whitespace() && lexer.begin == lexer.end {
                lexer.begin += 1;
                lexer.push(TokenType::Infer, true);
                lexer.go_next();
                continue;
            }
            
            match lexer.state {
                State::Base => {
                    if String::from("()[]{}+*/,;!:").find(lexer.symbol).is_some() {
                        lexer.push(TokenType::Infer, true);
                    }
                    else if String::from("-=><").find(lexer.symbol).is_some() {
                        lexer.state = State::DoubleOperator;    
                    }
                    else if lexer.symbol.is_alphabetic() {
                        lexer.state = State::Name;
                    }
                    else if lexer.symbol.is_numeric() {
                        lexer.state = State::IntConst;
                    }
                    else if lexer.symbol == '\'' {
                        lexer.state = State::CharConst;
                    }
                }

                State::DoubleOperator => {
                    if !lexer.push(TokenType::Infer, true) {
                        lexer.push(TokenType::Infer, false);
                    }
                    lexer.state = State::Base;
                }

                State::Name => {
                    if !lexer.symbol.is_alphanumeric() && lexer.symbol != '_' {
                        lexer.push(TokenType::Infer, false);
                        lexer.state = State::Base;
                    }
                }

                State::IntConst => {
                    if lexer.symbol == '.' {
                        lexer.state = State::FloatConst;
                    }
                    else if !lexer.symbol.is_numeric() {
                        lexer.push(TokenType::IntConst, false);    
                        lexer.state = State::Base;
                    }                    
                }

                State::FloatConst => {
                    if !lexer.symbol.is_numeric() {
                        lexer.push(TokenType::FloatConst, false);    
                        lexer.state = State::Base;
                    }                    
                }

                State::CharConst => {
                    lexer.go_next();
                    if lexer.symbol != '\'' {
                        panic!("Expected |\'|");
                    }
                    lexer.push(TokenType::CharConst, true);
                    lexer.state = State::Base;
                }

                _ => {}
            }

            if lexer.should_go_next {
                lexer.go_next();
            }
        }
        
        lexer.tokens
    }
}
