#[derive(Clone, Debug)]
pub enum TokenType {
    Infer,

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
    GE
}

#[derive(Clone, Debug)]
pub struct Token {
    pub lexeme : String,
    pub _type : TokenType,
    pub line : i32
}
