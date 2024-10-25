use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Grammar {
    rules : HashMap<String, 
    terminals : 
}

impl Grammar {
    

    fn match(token_type : TokenType)
    {
         
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
