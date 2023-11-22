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

pub struct Lexer
{
    pub tokens: Vec<Token>,
}

impl Lexer
{
    fn push_punct(&mut self, tok : Punctuation) { self.tokens.push(Token::Punctuation(tok)); }
    fn push_delim(&mut self, tok : Delimiter) { self.tokens.push(Token::Delimiter(tok)); }
    fn push_literal(&mut self, tok : Literal) { self.tokens.push(Token::Literal(tok)); }
    fn push_id(&mut self, id : String) { self.tokens.push(Token::Identifier(id)); }

    fn unexpected_char(c : char, index : usize) -> Result<(), String> { Err(format!("Unexpected character '{}' at index {}", c, index)) }

    pub fn flush(&mut self) {self.tokens = Vec::new();}

    pub fn new() -> Lexer 
    {
        Lexer {tokens : vec![]}
    }

    pub fn tokenize(&mut self, input: &String) -> Result<(), String>
    {
        let mut index : usize = 0;
        let mut it = input.chars().peekable();
        while let Some(c) = it.peek()
        {
            match c {
               '\n' => return Ok(()),
               '=' => { self.push_punct(Punctuation::Equal); it.next(); index += 1; },
               '+' => { self.push_punct(Punctuation::Plus); it.next(); index += 1;},
               '-' => { self.push_punct(Punctuation::Minus); it.next(); index += 1;},
               '*' => { self.push_punct(Punctuation::Times); it.next(); index += 1;},
               '/' => { self.push_punct(Punctuation::Divide); it.next(); index += 1;},
               '^' => { self.push_punct(Punctuation::Power); it.next(); index += 1;},
               ';' => { self.push_punct(Punctuation::SemiColon); it.next(); index += 1;},
               ',' => { self.push_punct(Punctuation::Comma); it.next(); index += 1;},
               '?' => { self.push_punct(Punctuation::Interrogation); it.next(); index += 1;},
               '(' => { self.push_delim(Delimiter::OpenParenthese); it.next(); index += 1;},
               ')' => { self.push_delim(Delimiter::CloseParenthese); it.next(); index += 1;},
               '[' => { self.push_delim(Delimiter::OpenBracket); it.next(); index += 1;},
               ']' => { self.push_delim(Delimiter::CloseBracket); it.next(); index += 1;},
               _ => {
                // Parsing Number
                let mut value : i64 = 0;
                if c.is_digit(10) {
                    while let Some(c) = it.peek() {
                        if !c.is_digit(10) {break};
                        value *= 10;
                        value += c.to_digit(10).unwrap() as i64;
                        it.next();
                        index += 1;
                    }
                    if let Some(c) = it.peek() { // TODO refactor, very ugly
                        if c.is_alphanumeric()
                        {
                            if *c == 'i' {
                                it.next();
                                index += 1;
                                match it.peek() {
                                    Some(nc) => {
                                        if nc.is_alphanumeric() {
                                            return Lexer::unexpected_char(*nc, index + 1);
                                        }
                                        self.push_literal(Literal::Complex(value))
                                    },
                                    None => self.push_literal(Literal::Complex(value))
                                }
                            }
                            else {
                                return Lexer::unexpected_char(*c, index);
                            }
                        }
                        else {
                            self.push_literal(Literal::Number(value));
                        }
                    }
                    else 
                    {
                        self.push_literal(Literal::Number(value))
                    }
                }
                // Parsing identifier/keywords
                else if c.is_alphabetic() {
                   let mut chars = String::new();
                   chars.push(*c);
                   while let Some(curr) = it.peek(){
                       if !curr.is_alphanumeric() {break};
                       chars.push(*curr);
                       it.next();
                       index += 1;
                   }
                   self.push_id(chars);
                }
               }
            }
       }
       Ok(())
    }
}


#[cfg(test)]
mod test
{
    use super::*;


    #[test]
    fn test_integer_simple()
    {
        let mut lex = Lexer::new();
        let pattern = String::from("239487329");
        let repr = Vec::from([Token::Literal(Literal::Number(239487329))]);
        let _ = lex.tokenize(&pattern);
        assert_eq!(lex.tokens, repr);
    }

    #[test]
    fn test_complex_simple()
    {
        let mut lex = Lexer::new();
        let pattern = String::from("239487329i");
        let repr = Vec::from([Token::Literal(Literal::Complex(239487329))]);
        let _ = lex.tokenize(&pattern);
        assert_eq!(lex.tokens, repr);
    }

    #[test]
    fn test_mult_simple()
    {
        let mut lex = Lexer::new();
        let pattern = String::from("6*42");

        let repr : Vec<Token> = vec![Token::Literal(Literal::Number(6)),
                                Token::Punctuation(Punctuation::Times),
                                Token::Literal(Literal::Number(42))];
        let _ = lex.tokenize(&pattern);
        assert_eq!(lex.tokens, repr);
    }

}
