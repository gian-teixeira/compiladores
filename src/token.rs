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

impl Token {
    pub fn get_keyword_table()
    -> HashMap<String, Token>
    {
        HashMap::from([
            (String::from("int"), Token::Int)
        ])
    }
}
