use crate::token::Token;
use crate::token::TokenType;
use regex::Regex;

enum State {
    Initial,
    Name,
    Integer,
    Float
}

pub struct Parser {}
impl Parser {
    pub fn parse(
        buffer : &String,
        token_type : TokenType)
    -> Option<Token>
    {
        let _type = match token_type {
            Token::TypeInfer => match &buffer[..] {
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
            },
            _ => token_type
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
    pub fn transition(
        &mut self,
        state : State,
        symbol : char,
        check_previous : bool)
    {
        let last_token = None;

        if check_previous {
            current_token = Parser::parse(&self.buffer, TokenType::Infer);
        }

        self.buffer.append(symbol);

        let new_token = Parser::parse(&self.buffer, TokenType::Infer);
        if new_token
    }

    pub fn change_state(
        &mut self,
        next : State,
        token_type : Option<TokenType>,
        to_append : Option<char>)
    {
        if to_append.is_some() {
            let c = to_append.unwrap();
            if !c.is_whitespace() {
                buffer.push(c);
            }
        }

        if token_type.is_some() {
            let token = Parser::parse(&buffer, token_type.unwrap()); 
            if token.is_some() {
                self.tokens.push(token.unwrap());
                self.buffer = String::new();
            }
            else {
                panic!("Buffer parser error");
            }
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

            match self.state {
                // Base point and single component operators
                State::Initial => {
                    if c.is_alphabetic() {
                        self.change_state(State::Name, None, Some(c));
                    }
                    else if c.is_numeric() {
                        self.change_state(State::Integer, None, Some(c));
                    }
                    else {
                        match String::from("(){}[]+*/;,").find(c) {
                            Some(_) => self.change_state(State::Initial, TokenType::Infer, Some(c)),
                            None => match String::from("-=!><").find(c) {
                                Some(_) => self.change_state(State::Initial, None, Some(c)),
                                None => {}
                            }
                        }
                    }
                }

                State::DoubleOperator => {
                    let single_token = Parser::parse(&buffer, TokenType::Infer);
                    buffer.push(c);
                    let double_token = Parser::parse(&buffer, TokenType::Infer);
                    
                    let token = if double_token.is_some() { double_token }
                    else if single_token { 

                    if double_token.is_some() {

                    }
                    match single_token {
                        Some(T) => {
                            self.tokens.push(T);
                            self.buffer = String::new();
                        }
                        None => 
                    self.change_state(State::Initial, TokenType::Infer, Some(c));
                }

                // Names
                State::Name => {
                    if !c.is_alphanumeric() && c != '_' {
                        self.change_state(State::Initial, None, c);
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
                        self.change_state(
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
