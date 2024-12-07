#![allow(non_snake_case)]

use crate::token::Token;
use crate::token::TokenType;

pub struct Grammar {
    tokens: Vec<Token>,
    head: usize,
}

impl Grammar {
    fn new(tokens: Vec<Token>) -> Grammar {
        Grammar {
            tokens: tokens,
            head: 0,
        }
    }

    pub fn analyze(token_list: &Vec<Token>) {
        let tokens = token_list.clone();
        let mut helper = Grammar::new(tokens);
        helper.Program();
    }

    fn check_token(&mut self) -> Token {
        return self.tokens[self.head].clone();
    }

    fn error(&mut self) {
        let token = self.check_token();
        panic!(
            "SYNTATIC ERROR : Unexpected token {1} [line {0}]",
            token.line, token.lexeme
        );
    }

    fn Match(&mut self, expected: TokenType) {
        if self.check_token()._type != expected {
            println!("{:?} {:?}", self.check_token()._type, expected);
            self.error();
        }
        //println!("{:?}", self.check_token());
        self.head += 1;
    }

    fn Program(&mut self) {
        self.Function();
        self.FunctionSequence();
    }

    fn FunctionSequence(&mut self) {
        match self.check_token()._type {
            TokenType::Function => {
                self.Function();
                self.FunctionSequence();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn Function(&mut self) {
        self.Match(TokenType::Function);
        self.FunctionName();
        self.Match(TokenType::LBracket);
        self.ParameterList();
        self.Match(TokenType::RBracket);
        self.FunctionReturnType();
        self.Block();
    }

    fn FunctionName(&mut self) {
        match self.check_token()._type {
            TokenType::Id => self.Match(TokenType::Id),
            TokenType::Main => self.Match(TokenType::Main),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn ParameterList(&mut self) {
        match self.check_token()._type {
            TokenType::Id => {
                self.Match(TokenType::Id);
                self.Match(TokenType::Colon);
                self.Type();
                self.ParameterList_()
            }
            TokenType::EOF | _ => {}
        }
    }

    fn ParameterList_(&mut self) {
        match self.check_token()._type {
            TokenType::Comma => {
                self.Match(TokenType::Comma);
                self.Match(TokenType::Id);
                self.Match(TokenType::Colon);
                self.Type();
                self.ParameterList_();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn FunctionReturnType(&mut self) {
        match self.check_token()._type {
            TokenType::Arrow => {
                self.Match(TokenType::Arrow);
                self.Type();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn Block(&mut self) {
        self.Match(TokenType::LBrace);
        self.Sequence();
        self.Match(TokenType::RBrace);
    }

    fn Sequence(&mut self) {
        match self.check_token()._type {
            TokenType::Let => {
                self.Declaration();
                self.Sequence();
            }
            TokenType::Id
            | TokenType::If
            | TokenType::While
            | TokenType::Println
            | TokenType::Return => {
                self.Command();
                self.Sequence();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn Declaration(&mut self) {
        self.Match(TokenType::Let);
        self.VarList();
        self.Match(TokenType::Colon);
        self.Type();
        self.Match(TokenType::PComma);
    }

    fn VarList(&mut self) {
        self.Match(TokenType::Id);
        self.VarList_();
    }

    fn VarList_(&mut self) {
        match self.check_token()._type {
            TokenType::Comma => {
                self.Match(TokenType::Comma);
                self.Match(TokenType::Id);
                self.VarList_();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn Type(&mut self) {
        match self.check_token()._type {
            TokenType::Int => self.Match(TokenType::Int),
            TokenType::Float => self.Match(TokenType::Float),
            TokenType::Char => self.Match(TokenType::Char),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn Command(&mut self) {
        match self.check_token()._type {
            TokenType::Id => {
                self.Match(TokenType::Id);
                self.AttrOrCall();
            }
            TokenType::If => self.IfCommand(),
            TokenType::While => {
                self.Match(TokenType::While);
                self.Expr();
                self.Block();
            }
            TokenType::Println => {
                self.Match(TokenType::Println);
                self.Match(TokenType::LBracket);
                self.Match(TokenType::FormatString);
                self.Match(TokenType::Comma);
                self.ArgList();
                self.Match(TokenType::RBracket);
                self.Match(TokenType::PComma);
            }
            TokenType::Return => {
                self.Match(TokenType::Return);
                self.Expr();
                self.Match(TokenType::PComma);
            }
            TokenType::EOF | _ => self.error(),
        }
    }

    fn AttrOrCall(&mut self) {
        match self.check_token()._type {
            TokenType::Attr => {
                self.Match(TokenType::Attr);
                self.Expr();
                self.Match(TokenType::PComma);
            }
            TokenType::LBracket => {
                self.Match(TokenType::LBracket);
                self.ArgList();
                self.Match(TokenType::RBracket);
                self.Match(TokenType::PComma);
            }
            TokenType::EOF | _ => self.error(),
        }
    }

    fn IfCommand(&mut self) {
        match self.check_token()._type {
            TokenType::If => {
                self.Match(TokenType::If);
                self.Expr();
                self.Block();
                self.ElseCommand();
            }
            TokenType::LBrace => self.Block(),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn ElseCommand(&mut self) {
        match self.check_token()._type {
            TokenType::Else => {
                self.Match(TokenType::Else);
                self.IfCommand();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn Expr(&mut self) {
        self.Rel();
        self.ExprOpc();
    }

    fn ExprOpc(&mut self) {
        match self.check_token()._type {
            TokenType::EQ | TokenType::NEQ => {
                self.OpEqual();
                self.Rel();
                self.ExprOpc();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn OpEqual(&mut self) {
        match self.check_token()._type {
            TokenType::EQ => self.Match(TokenType::EQ),
            TokenType::NEQ => self.Match(TokenType::NEQ),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn Rel(&mut self) {
        self.Add();
        self.RelOpc();
    }

    fn RelOpc(&mut self) {
        match self.check_token()._type {
            TokenType::LT | TokenType::LE | TokenType::GT | TokenType::GE => {
                self.OpRel();
                self.Add();
                self.RelOpc();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn OpRel(&mut self) {
        match self.check_token()._type {
            TokenType::LT => self.Match(TokenType::LT),
            TokenType::LE => self.Match(TokenType::LE),
            TokenType::GT => self.Match(TokenType::GT),
            TokenType::GE => self.Match(TokenType::GE),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn Add(&mut self) {
        self.Term();
        self.AddOpc();
    }

    fn AddOpc(&mut self) {
        match self.check_token()._type {
            TokenType::Plus | TokenType::Minus => {
                self.OpAdd();
                self.Term();
                self.AddOpc();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn OpAdd(&mut self) {
        match self.check_token()._type {
            TokenType::Plus => self.Match(TokenType::Plus),
            TokenType::Minus => self.Match(TokenType::Minus),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn Term(&mut self) {
        self.Factor();
        self.TermOpc();
    }

    fn TermOpc(&mut self) {
        match self.check_token()._type {
            TokenType::Mult | TokenType::Div => {
                self.OpMult();
                self.Factor();
                self.TermOpc();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn OpMult(&mut self) {
        match self.check_token()._type {
            TokenType::Mult => self.Match(TokenType::Mult),
            TokenType::Div => self.Match(TokenType::Div),
            TokenType::EOF | _ => self.error(),
        }
    }

    fn Factor(&mut self) {
        match self.check_token()._type {
            TokenType::Id => {
                self.Match(TokenType::Id);
                self.FunctionCall();
            }
            TokenType::IntConst => self.Match(TokenType::IntConst),
            TokenType::FloatConst => self.Match(TokenType::FloatConst),
            TokenType::CharConst => self.Match(TokenType::CharConst),
            TokenType::LBracket => {
                self.Match(TokenType::LBracket);
                self.Expr();
                self.Match(TokenType::RBracket);
            }
            TokenType::EOF | _ => self.error(),
        }
    }

    fn FunctionCall(&mut self) {
        match self.check_token()._type {
            TokenType::LBracket => {
                self.Match(TokenType::LBracket);
                self.ArgList();
                self.Match(TokenType::RBracket);
            }
            TokenType::EOF | _ => {}
        }
    }

    fn ArgList(&mut self) {
        match self.check_token()._type {
            TokenType::Id | TokenType::IntConst | TokenType::FloatConst | TokenType::CharConst => {
                self.Arg();
                self.ArgList_();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn ArgList_(&mut self) {
        match self.check_token()._type {
            TokenType::Comma => {
                self.Match(TokenType::Comma);
                self.Arg();
                self.ArgList_();
            }
            TokenType::EOF | _ => {}
        }
    }

    fn Arg(&mut self) {
        match self.check_token()._type {
            TokenType::Id => {
                self.Match(TokenType::Id);
                self.FunctionCall();
            }
            TokenType::IntConst => self.Match(TokenType::IntConst),
            TokenType::FloatConst => self.Match(TokenType::FloatConst),
            TokenType::CharConst => self.Match(TokenType::CharConst),
            TokenType::EOF | _ => self.error(),
        }
    }
}
