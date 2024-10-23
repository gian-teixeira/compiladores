use regex::Regex;

#[derive(Clone, Debug)]
pub enum TokenType {
    Infer,
    Ignore,

    Id,

    Int,
    Float,
    Char,

    IntConst,
    FloatConst,
    CharConst,

    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LCol,
    RCol,
    Colon,
    Comma,
    PComma,
    Attr,
    Not,
    Plus,
    Minus,
    Mult,
    Div,

    EQ,
    NEQ,
    LT,
    LE,
    GT,
    GE,

    If,
    Else,
    While,
    Function,
    Return,
    Main,
    Println,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub lexeme : String,
    pub _type : TokenType,
    pub line : i32
}

impl Token {
    pub fn new(
        buffer : &str,
        token_type : TokenType,
        line : i32)
    -> Option<Token>
    {
        let _type = match token_type {
            TokenType::Infer => match buffer {
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
                ":" => Some(TokenType::Colon),
                ";" => Some(TokenType::PComma),
                "," => Some(TokenType::Comma),
                "=" => Some(TokenType::Attr),
                ">" => Some(TokenType::GT),
                "<" => Some(TokenType::LT),
                "!" => Some(TokenType::NEQ),
                "==" => Some(TokenType::EQ),
                ">=" => Some(TokenType::GE),
                "<=" => Some(TokenType::LE),
                "int" => Some(TokenType::Int),
                "float" => Some(TokenType::Float),
                "char" => Some(TokenType::Char),
                "if" => Some(TokenType::If),
                "else" => Some(TokenType::Else),
                "while" => Some(TokenType::While),
                "fn" => Some(TokenType::Function),
                "return" => Some(TokenType::Return),
                "main" => Some(TokenType::Main),
                "println" => Some(TokenType::Println),
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
            Some(t) => Some(Token {
                lexeme : String::from(buffer),
                _type : t,
                line : line
            })
        }
    }
}
