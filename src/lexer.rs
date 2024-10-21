use crate::token::Token;
use crate::token::TokenType;
use regex::Regex;

enum State {
    Base(Option<TokenType>),
    Name,
    Integer,
    Float
}

pub struct Parser {}
impl Parser {
    pub fn parse(
        buffer : &String,
        token_type : Option<TokenType>)
    -> Option<Token>
    {
        let _type = match token_type {
            Some(_) => token_type,
            None => match &buffer[..] {
                ")" => Some(TokenType::RBracket),
                "(" => Some(TokenType::LBracket),
                "}" => Some(TokenType::RBrace),
                "{" => Some(TokenType::LBrace),
                "]" => Some(TokenType::RCol),
                "[" => Some(TokenType::LCol),
                "+" => Some(TokenType::Plus),
                "-" => Some(TokenType::Minus),
                "*" => Some(TokenType::Mult),
                "/" => Some(TokenType::Div),
                ";" => Some(TokenType::PComma),
                "," => Some(TokenType::Comma),
                "==" => Some(TokenType::EQ),
                "!=" => Some(TokenType::NEQ),
                ">=" => Some(TokenType::GE),
                "<=" => Some(TokenType::LE),
                "int" => Some(TokenType::Int),
                "float" => Some(TokenType::Float),
                "char" => Some(TokenType::Char),
                _ => {
                    let re = Regex::new(r"[a-zA-Z0-9_]+").unwrap();
                    match re.is_match(buffer) {
                        true => Some(TokenType::Id),
                        false => None
                    }
                }
                // TODO : check if name is valid (not start with weird simble, etc)
            }
        };

        match _type {
            None => None,
            Some(T) => Some(Token {
                lexeme : buffer.clone(),
                _type : T,
                line : -1
            })
        }
    }
}
    
pub struct Lexer {
    state : State,
    buffer : String,
    tokens : Vec<Token>
}

impl Lexer {
    pub fn chage_state(
        &mut self,
        next : State,
        parse_buffer : bool,
        to_append : Option<char>)
    {
        if parse_buffer {
            let token = Parser::parse(&buffer, buffer_type.clone()); 
            if token.is_some() {
                self.tokens.push(token.unwrap());
                self.buffer = String::new();
            }
            else {
                panic!("Buffer parser error");
            }
        }
        if to_append.is_some() {
            buffer.push(to_append.unwrap());
        }
        self.state = next;
    }
    
    pub fn parse(
        &mut self,
        src : &String)
    -> Vec<Token>
    {
        self.state = State::Base(None);
        self.tokens : Vec<Token> = Vec::new();
        self.buffer : String = String::new();
        
        let mut to_buffer : bool;
        
        for c in src.chars() {
            println!("{buffer}");

            to_buffer = true;

            match state {
                // Base point and single component operators
                State::Base(ref buffer_type) => {

                    if c.is_alphabetic() {
                        self.change_state(State::Name, false, Some(c));
                    }
                    else if c.is_numeric() {
                        self.change_state(State::Integer, false, Some(c));
                    }
                    else {
                        match String::from("(){}[]+*/;,").find(c) {
                            Some(_) => self.change_state(State::Base(None), true, c),
                            None => match String::from("-=!><").find(c) {
                                Some(_) => self.change_state(State::Base(None), false, c),
                                None => {}
                            }
                        }
                    }
                }

                // Names
                State::Name => {
                    if !c.is_alphanumeric() && c != '_' {
                        state = State::Base(None, true, c);
                    }
                }

                // Integer
                State::Integer => {
                    if c == '.' {
                        self.change_state(State::Float, false, c);
                    }
                    else if !c.is_numeric() {
                        self.change_state(State::Base(TokenType::IntConst), true, c);
                    }
                }

                // Float
                State::Float => {
                    if !c.is_numeric() {
                        state = State::Base(Some(TokenType::FloatConst));
                    }
                }
            }            
            
            if to_buffer && !c.is_control() && !c.is_whitespace() {
                buffer.push(c);
            }
        }
        
        return tokens;
    }
}
