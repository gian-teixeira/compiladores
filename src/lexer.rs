use crate::token::Token;
use std::collections::HashMap;

pub enum State {
    Base,
    Double(char, Token, Token),
    Name,
    Integer,
    Float
}

pub fn parse(src : &String)
-> Vec<Token>
{
    let keyword_table = Token::get_keyword_table();

    let mut state = State::Base;
    let mut tokens : Vec<Token> = Vec::new();
    let mut buffer : String = String::new();
    let mut to_buffer : bool;

    for c in src.chars() {
        println!("{buffer}");

        to_buffer = true;

        match state {
            
            // Base point and single component operators
            State::Base => {
                if c.is_alphabetic() {
                    state = State::Name;
                }
                else if c.is_numeric() {
                    state = State::Integer;
                }
                else {
                    to_buffer = false;

                    let single_token = match c {
                        ')' => Some(Token::LBracket),
                        '(' => Some(Token::RBracket),
                        '}' => Some(Token::LBrace),
                        '{' => Some(Token::RBrace),
                        ']' => Some(Token::LCol),
                        '[' => Some(Token::RCol),
                        '+' => Some(Token::Plus),
                        '-' => Some(Token::Minus),
                        '*' => Some(Token::Mult),
                        '/' => Some(Token::Div),
                        ';' => Some(Token::PComma),
                        ',' => Some(Token::Comma),
                        _ => None
                    };

                    if single_token.is_some() {
                        tokens.push(single_token.unwrap());
                        state = State::Base;
                    }
                    else {
                        let double_state = match c {
                            '=' => Some(State::Double('=', Token::EQ, Token::Attr)),
                            '!' => Some(State::Double('=', Token::NEQ, Token::Not)),
                            '>' => Some(State::Double('=', Token::GE, Token::GT)),
                            '<' => Some(State::Double('=', Token::LE, Token::LT)),
                            _ => None
                        };
                        if double_state.is_some() {
                            state = double_state.unwrap();
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
                state = State::Base;
                to_buffer = false;
            }

            // Names
            State::Name => {
                if !c.is_alphanumeric() && c != '_' {
                    let default_token = Token::Id(buffer.clone());
                    let token = keyword_table
                        .get(&buffer)
                        .unwrap_or(&default_token);
                    tokens.push(token.clone());
                    buffer = String::new();
                    state = State::Base;
                }
            }

            // Integer
            State::Integer => {
                if c == '.' {
                    state = State::Float;
                }
                else if !c.is_numeric() {
                    match buffer.parse::<i64>() {
                        Ok(int_value) => tokens.push(Token::IntConst(int_value)),
                        Err(e) => println!("{e}")
                    }
                    buffer = String::new();
                    state = State::Base;
                }
            }

            // Float
            State::Float => {
                if !c.is_numeric() {
                    match buffer.parse::<f32>() {
                        Ok(float_value) => tokens.push(Token::FloatConst(float_value)),
                        Err(e) => println!("{e}")
                    }
                    buffer = String::new();
                    state = State::Base;
                }
            }
        }            
        
        if to_buffer && !c.is_control() && !c.is_whitespace() {
            buffer.push(c);
        }
    }
    
    return tokens;
}

