use crate::token::Token;
use std::collections::HashMap;

enum State {
    Base(Option<fn(&String) -> Option<Token>>),
    Double(char, Token, Token),
    Name,
    Integer,
    Float
}

pub struct Parser {
    keyword_table : HashMap<String, Token>
}

impl Parser {
    pub fn to_int(buffer : &String)
    -> Option<Token>
    {
        buffer.parse::<i64>().map_or(None, |v| {
            let wrapper = Token::IntConst(v);
            Some(wrapper)
        })
    }
    
    pub fn to_float(buffer : &String)
    -> Option<Token>
    {
        buffer.parse::<f32>().map_or(None, |v| {
            let wrapper = Token::FloatConst(v);
            Some(wrapper)
        })
    }

    pub fn to_single_operator(buffer : &String)
    -> Option<Token>
    {
        match &buffer[..] {
            ")" => Some(Token::LBracket),
            "(" => Some(Token::RBracket),
            "}" => Some(Token::LBrace),
            "{" => Some(Token::RBrace),
            "]" => Some(Token::LCol),
            "[" => Some(Token::RCol),
            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            "*" => Some(Token::Mult),
            "/" => Some(Token::Div),
            ";" => Some(Token::PComma),
            "," => Some(Token::Comma),
            _ => None
        }
    }

    pub fn to_double_operator(buffer : &String)
    -> Option<Token>
    {
        match &buffer[..] {
            "==" => Some(Token::EQ),
            "!=" => Some(Token::NEQ),
            ">=" => Some(Token::GE),
            "<=" => Some(Token::LE),
            _ => None
        }
    }

    pub fn to_key(buffer : &String)
    -> Option<Token>
    {
        let token = match &buffer[..] {
            "int" => Token::Int,
            _ => Token::Id(buffer.clone())
        };
        Some(token)
    }
    
    pub fn parse(src : &String)
    -> Vec<Token>
    {
        let mut state = State::Base(None);
        let mut tokens : Vec<Token> = Vec::new();
        let mut buffer : String = String::new();
        let mut to_buffer : bool;

        for c in src.chars() {
            println!("{buffer}");

            to_buffer = true;

            match state {
                // Base point and single component operators
                State::Base(buffer_parser) => {
                    if buffer_parser.is_some() {
                        let token = buffer_parser.unwrap()(&buffer);
                        tokens.push(token.unwrap());
                        buffer = String::new();
                    }

                    if c.is_alphabetic() {
                        state = State::Name;
                    }
                    else if c.is_numeric() {
                        state = State::Integer;
                    }
                    else {
                        match String::from("(){}[]+-*/;,").find(c) {
                            Some(_) => state = State::Base(Some(Parser::to_single_operator)),
                            None => match String::from("=!><").find(c) {
                                Some(_) => state = State::Base(Some(Parser::to_double_operator)),
                                None => {}
                            }
                        }
                    }
                }

                // Double component operators
                State::Double(ref expected, ref result, ref except) => {
                    tokens.push(
                        if c == *expected { result.clone() }
                        else { except.clone() }
                    );
                    state = State::Base(None);
                    to_buffer = false;
                }

                // Names
                State::Name => {
                    if !c.is_alphanumeric() && c != '_' {
                        state = State::Base(Some(Parser::to_key));
                    }
                }

                // Integer
                State::Integer => {
                    if c == '.' {
                        state = State::Float;
                    }
                    else if !c.is_numeric() {
                        state = State::Base(Some(Parser::to_int));
                    }
                }

                // Float
                State::Float => {
                    if !c.is_numeric() {
                        state = State::Base(Some(Parser::to_float));
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
