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
            TokenType::Infer => match &buffer[..] {
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
            _ => Some(token_type)
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
//    pub fn transition(
//        &mut self,
//        state : State,
//        symbol : char,
//        check_previous : bool)
//    {
//        let last_token = None;
//
//        if check_previous {
//            current_token = Parser::parse(&self.buffer, TokenType::Infer);
//        }
//
//        self.buffer.append(symbol);
//
//        let new_token = Parser::parse(&self.buffer, TokenType::Infer);
//        if new_token
//    }
//
//    pub fn change_state(
//        &mut self,
//        next : State,
//        token_type : Option<TokenType>,
//        to_append : Option<char>)
//    {
//        if to_append.is_some() {
//            let c = to_append.unwrap();
//            if !c.is_whitespace() {
//                buffer.push(c);
//            }
//        }
//
//        if token_type.is_some() {
//            let token = Parser::parse(&buffer, token_type.unwrap()); 
//            if token.is_some() {
//                self.tokens.push(token.unwrap());
//                self.buffer = String::new();
//            }
//            else {
//                panic!("Buffer parser error");
//            }
//        }
//
//        self.state = next;
//    }
    pub fn parse_file(filename : &str)
    -> Vec<Token>
    {
        let binding = std::fs::read_to_string(filename)
            .unwrap_or_else(|err| {
                eprintln!("Parsing error : {err}");
                std::process::exit(1);
            });
        let source = binding.as_str();
        
        let mut tokens : Vec<Token> = Vec::new();
        let mut state : usize = 0;
        let mut nline : i32 = 0;
        let mut i : usize = 1;
        let mut j : usize = 0;

        while i <= source.len() {
            let symbol = &source[i-1..i];
            println!("{state}");
            
            match state {
                0 => {
                    if String::from("()[]{}+*/,;!").find(symbol).is_some() {
                        let token = Token::new(&source[j..i], TokenType::Infer, nline);
                        match token {
                            Some(T) => {
                                tokens.push(T);
                                j = i;
                            }
                            None => panic!("ERROR_ON_STATE::0")
                        }
                    }
                    else if String::from("-=><").find(symbol).is_some() {
                        state = 1;    
                    }
                    else {
                        j = i;
                    }

                    if symbol == "\n" {
                        nline += 1;
                    }
                }

                1 => {
                    let operator = Token::new(&source[j..i], TokenType::Infer, nline);

                    if operator.is_some() {
                        println!("source {} {} _{}_", j, i-1, &source[j..i]);
                        tokens.push(operator.unwrap());
                        j = i;
                    }
                    else {
                        println!("source {} {} _{}_", j, i-1, &source[j..i-1]);
                        let operator = Token::new(&source[j..i-1], TokenType::Infer, nline);
                        tokens.push(operator.unwrap());
                        j = i-1;
                    }

                    state = 0;
                }

                _ => {}
            }

            i += 1;
        }
        
        tokens
    }
    
//    pub fn parse(
//        &mut self,
//        src : &String)
//    -> Vec<Token>
//    {
//        self.state = State::Base(None);
//        self.tokens : Vec<Token> = Vec::new();
//        self.buffer : String = String::new();
//        
//        let mut to_buffer : bool;
//        
//        for c in src.chars() {
//            println!("{buffer}");
//
//            to_buffer = true;
//
//            match self.state {
//                // Base point and single component operators
//                State::Initial => {
//                    if c.is_alphabetic() {
//                        self.change_state(State::Name, None, Some(c));
//                    }
//                    else if c.is_numeric() {
//                        self.change_state(State::Integer, None, Some(c));
//                    }
//                    else {
//                        match String::from("(){}[]+*/;,").find(c) {
//                            Some(_) => self.change_state(State::Initial, TokenType::Infer, Some(c)),
//                            None => match String::from("-=!><").find(c) {
//                                Some(_) => self.change_state(State::Initial, None, Some(c)),
//                                None => {}
//                            }
//                        }
//                    }
//                }
//
//                State::DoubleOperator => {
//                    let single_token = Parser::parse(&buffer, TokenType::Infer);
//                    buffer.push(c);
//                    let double_token = Parser::parse(&buffer, TokenType::Infer);
//                    
//                    let token = if double_token.is_some() { double_token }
//                    else if single_token { 
//
//                    if double_token.is_some() {
//
//                    }
//                    match single_token {
//                        Some(T) => {
//                            self.tokens.push(T);
//                            self.buffer = String::new();
//                        }
//                        None => 
//                    self.change_state(State::Initial, TokenType::Infer, Some(c));
//                }
//
//                // Names
//                State::Name => {
//                    if !c.is_alphanumeric() && c != '_' {
//                        self.change_state(State::Initial, None, c);
//                        state = State::Base(None, true, c);
//                    }
//                }
//
//                // Integer
//                State::Integer => {
//                    if c == '.' {
//                        self.change_state(State::Float, false, c);
//                    }
//                    else if !c.is_numeric() {
//                        self.change_state(State::Base(TokenType::IntConst), true, c);
//                    }
//                }
//
//                // Float
//                State::Float => {
//                    if !c.is_numeric() {
//                        self.change_state(
//                        state = State::Base(Some(TokenType::FloatConst));
//                    }
//                }
//            }            
//            
//            if to_buffer && !c.is_control() && !c.is_whitespace() {
//                buffer.push(c);
//            }
//        }
//        
//        return tokens;
//    }
}
