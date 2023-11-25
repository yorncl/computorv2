#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Identifer
    Identifier(String),

    // Punctuation
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
    Interrogation,

    // Literal
    Number(i64),
    Complex(i64),

    // Delimiter
    OpenParenthese,
    CloseParenthese,
    OpenBracket,
    CloseBracket
}
