use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::collections::HashSet;

struct SyntaticError {
    token : Option<Token>,
    line : Option<i32>,
}

impl SyntaticError {
    fn dummy()
    -> SyntaticError
    {
        SyntaticError {
            token : None,
            line : None
        }
    }

    fn is_dummy(&self)
    -> bool
    {
        return self.token.is_none();
    }

    fn 
}

impl std::ops::BitAnd for SyntaticError {
    type Output = SyntaticError;
    fn bitand(self, other) -> SyntaticError {
        if self.is_dummy() {
            return other;
        } 
        return self;
    }
}

pub struct Grammar {
    tokens : &Vec<Token>,
    head : i32,

    rules : HashMap<String, 
    terminals : 
    error : Option<String>,
    
}

impl Grammar {
    fn new(tokens : &Vec<Token>)
    -> Grammar
    {
        Grammar {
            tokens : tokens,
            head : 0
        } 
    }

    fn check_token(&self)
    -> Token
    {
        return self.tokens[self.head];
    }

    fn error(&self)
    {
        let token = self.check_token();
        panic!(format!("SYNTATIC ERROR on line {} : Unexpected token {}",
            token.line, token.lexeme));
    }

    fn match(&self,
        expected : &TokenType)
    {
        if self.check_token()._type != token_type {
            self.error();
        }
        head += 1;
    }

    fn Program(&self) {
        self.Function();
        self.FunctionSequence();
    }

    fn FunctionSequence(&self) {
        if self.check_token()._type == TokenType::Function {
            self.Function();
            self.FunctionSequence();
        }
    }

    fn Function(&self) {
        self.match(TokenType::Function);
        self.FunctionName();
        self.match(TokenType::LBracket);
        self.ParameterList();
        self.match(TokenType::RBracket);
        self.FunctionReturnType();
        self.Block();
    }

    fn FunctionName(&self) {
        match self.check_token()._type {
            TokenType::Id => self.match(TokenType::Id),
            TokenType::Main => self.match(TokenType::Main),
            _ => self.error()
        }
    }

    fn ParameterList(&self) {
        if self.get_token()._type == TokenType::Id {
            self.match(TokenType::Id);
            self.match(TokenType::Colon);
            self.Type();
            self.ParameterList_()
        }
    }

    fn ParameterList_(&self) {
        if self.get_token()._type == TokenType::Comma {
            self.match(TokenType::Comma);
            self.match(TokenType::Id);
            self.match(TokenType::Colon);
            self.Type();
            self.ParameterList_();
        }
    }

    fn FunctionReturnType(&self) {
        if self.get_token()._type == TokenType::Arrow {
            self.match(TokenType::Arrow);
            self.Type();
        }
    }

    fn Block(&self) {
        self.match(TokenType::LBrace);
        self.Sequence();
        self.match(TokenType::RBrace);
    }

    fn Sequence(&self) {
        match self.check_token()._type {
            TokenType::Let => {
                self.Declaration();
                self.Sequence();
            },
            TokenType::Id | TokenType::If | TokenType::While 
                    | TokenType::Println | TokenType::Return => {
                self.Command();
                self.Sequence();
            },
            _ => {}
        }
    }

    fn Declaration(&self) {
        self.match(TokenType::Let);
        self.VarList();
        self.match(TokenType::Colon);
        self.Type();
        self.match(TokenType::PComma);
    }

    fn VarList(&self) {
        self.match(TokenType::Id);
        self.VarList_();
    }

    fn VarList_(&self) {
        if self.check_token()._type == TokenType::Id {
            self.match(TokenType::Id);
            self.VarList_();
        }
    }

    fn Type(&self) {
        match self.check_token()._type {
            TokenType::Int => self.match(TokenType::Int),
            TokenType::Float => self.match(TokenType::Float),
            TokenType::Char => self.match(TokenType::Char),
            _ => self.error();
        }
    }

    fn Command(&self) {
        match self.check_token()._type {
            TokenType::Id => {
                self.match(TokenType::Id);
                self.AttrOrCall();
            },
            TokenType::If => self.IfCommand(),
            TokenType::While => {
                self.match(TokenType::While);
                self.Expr();
                self.Block();
            },
            TokenType::Println => {
                self.match(TokenType::Println);
                self.match(TokenType::LBracket);
                self.match(TokenType::FormatString);
                self.match(TokenType::Comma);
                self.ArgList();
                self.match(TokenType::RBracket);
                self.match(TokenType::PComma);
            },
            TokenType::Return => {
                self.match(TokenType::Return);
                self.Expr();
                self.match(TokenType::PComma);
            },
            _ => self.error()
        }
    }

    fn AttrOrCall(&self) {
        match self.check_token()._type {
            TokenType::Attr => {
                self.match(TokenType::Attr);
                self.Expr();
                self.match(TokenType::PComma);
            },
            TokenType::LBracket => {
                self.match(TokenType::LBracket);
                self.ArgList();
                self.match(TokenType::RBracket);
                self.match(TokenType::PComma);
            },
            _ => self.error();
        }
    }

    fn IfCommand(&self) {
        match self.check_token()._type {
            TokenType::If => {
                self.match(TokenType::If);
                self.Expr();
                self.Block();
                self.ElseCommand();
            },
            TokenType::LBrace => self.Block(),
            _ => self.error();
        }
    }

    fn ElseCommand(&self) {
        match self.check_token()._type {
            TokenType::Else => {
                self.match(TokenType::Else);
                self.IfCommand();
            },
            _ => {}
        }
    }
}

pub fn check(tokens: &Vec<Token>) {}
