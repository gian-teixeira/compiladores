use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Token {
    Id(String),
    Int,
    Float,
    Char,
    IntConst(i64),
    FloatConst(f32),
    CharConst(char),
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LCol,
    RCol,
    Comma,
    PComma,
    Attr,
    Not,
    EQ,
    NEQ,
    LT,
    LE,
    GT,
    GE,
    Plus,
    Minus,
    Mult,
    Div
}

pub enum State {
    Base,
    Double(char, Token, Token),
    Name,
    Integer,
    Float
}

pub struct TokenList {
    token_table : HashMap<String,Token>,
    tokens : Vec<Token>
}

impl TokenList {
    pub fn new()
    -> TokenList
    {
        TokenList {
            tokens : Vec::new(),
            token_table : HashMap::from([
                (String::from("int"), Token::Int)
            ])
        } 
    }
    
    pub fn append(
        &mut self,
        token : Token)
    {
        self.tokens.push(token);
    }

    pub fn get_name_token(
        &mut self,
        lexeme : String)
    -> Token
    {
        
        let default = Token::Id(lexeme.clone());
        let token = self.token_table
            .get(&lexeme)
            .unwrap_or(&default);
        return token.clone();
    }
}

pub fn parse(src : &String)
-> TokenList
{
    let mut state = State::Base;
    let mut tokens : TokenList = TokenList::new();
    let mut buffer : String = String::new();
    let mut to_buffer : bool;

    for c in src.chars() {
        println!("{buffer}");

        to_buffer = true;

        match state {
            
            // Base point and unitary tokens
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
                        tokens.append(single_token.unwrap());
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

            State::Double(ref expected, ref result, ref except) => {
                tokens.append(
                    if c == *expected { result.clone() }
                    else { except.clone() }
                );
                state = State::Base;
                to_buffer = false;
            }

            // Names
            State::Name => {
                if !c.is_alphanumeric() && c != '_' {
                    let name_token = tokens.get_name_token(buffer);
                    tokens.append(name_token);
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
                        Ok(int_value) => tokens.append(Token::IntConst(int_value)),
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
                        Ok(float_value) => tokens.append(Token::FloatConst(float_value)),
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

pub fn main() {
    let file_content = std::fs::read_to_string("tmp.txt")
        .expect("Should have been able to read the file");
    let tokens = parse(&file_content);
    for token in tokens.tokens {
        print!("{token:?} ");
    }
    println!("");
}
