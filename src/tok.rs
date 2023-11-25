#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Number(i64),
    Complex(i64)
    // Integer(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Punctuation {
    Equal,
    Plus,
    Minus,
    Times,
    MatrixTimes,
    Modulo,
    Divide,
    Power,
    SemiColon,
    Comma,
    Interrogation
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    OpenParenthese,
    CloseParenthese,
    OpenBracket,
    CloseBracket
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Punctuation(Punctuation),
    Literal(Literal),
    Delimiter(Delimiter)
}
