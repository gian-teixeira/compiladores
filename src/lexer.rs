use std::collections::HashMap;

#[derive(Clone)]
pub enum Token {
    Id(String),
    Int,
    Float,
    Char,
    IntConst,
    FloatConst,
    CharConst,
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
        lexeme : String)
    {
        
        let default = Token::Id(lexeme.clone());
        let token = self.token_table
            .get(&lexeme)
            .unwrap_or(&default);
        self.tokens.push(token.clone());
    }
}

pub fn parse(src : &String)
{
    let mut state : usize = 0;
    let mut tokens : TokenList = TokenList::new();
    let mut buffer : String = String::new();
    
    for c in src.chars() {
        match state {
            0 => {
                if c.is_alphabetic() {
                    state = 1;
                }            
            }
            1 => {
                if !c.is_alphabetic() {
                    tokens.append(buffer);
                    buffer = String::new();
                    if c != ' ' {
                        tokens.append(String::from(c));
                    } 
                }
            }
            _ => todo!()
        }            
    }
}

pub fn main() {
    println!("Hello, World!");
}
