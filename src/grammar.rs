use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Grammar {
    tokens : Vec<Token>,
    current : i32,

    rules : HashMap<String, 
    terminals : 
    error : Option<String>,
    
}

impl Grammar {
    

    fn match(
        &self,
        expected : &TokenType)
    {
        let token = &self.tokens[self.current];
        if token._type != token_type {
            self.error = String::from(
                format!("SINTATIC ERROR on line {} : Unexpected token {}",
                        token.line, token.lexeme));
            return false; 
        }
    }

    fn Program(&self) 
    {
        self.Function() && self.FunctionSequence()
    }

    fn Function(&self) 
    {
        self.match(TokenType::Function)
    }

    fn FunctionName(&self)
    {
        self.match(TokenType::Id)
        || self.match(TokenType::Main)
    }

    fn FunctionSequence(&self)
    {
        self.Function() && self.FunctionSequence()
        || true
    }

    fn ParameterList(&self)
    {
        self.match(TokenType::Id) && self.match(TokenType::Colon) && self.Type() 
            && self.ParameterList_()
        || true
    }
}

pub fn check(tokens: &Vec<Token>) {}
