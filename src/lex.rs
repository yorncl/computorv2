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

#[derive(Default)]
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

    pub fn flush(&mut self) {self.tokens = Vec::new();}

    fn unexpected_char(c : char, index : usize) -> Result<(), String> { Err(format!("Unexpected character '{}' at index {}", c, index)) }

    // fn read_number() -> Result<(), String> {
    // }

    pub fn tokenize(&mut self, input: &String) -> Result<(), String>
    {
        let mut index : usize = 0;
        let mut it = input.chars().peekable();
        while let Some(c) = it.next()
        {
            match c {
               '\n' => return Ok(()),
               '=' => self.push_punct(Punctuation::Equal),
               '+' => self.push_punct(Punctuation::Plus),
               '-' => self.push_punct(Punctuation::Minus),
               '*' => self.push_punct(Punctuation::Times),
               '/' => self.push_punct(Punctuation::Divide),
               '^' => self.push_punct(Punctuation::Power),
               ';' => self.push_punct(Punctuation::SemiColon),
               ',' => self.push_punct(Punctuation::Comma),
               '?' => self.push_punct(Punctuation::Interrogation),
               '(' => self.push_delim(Delimiter::OpenParenthese),
               ')' => self.push_delim(Delimiter::CloseParenthese),
               '[' => self.push_delim(Delimiter::OpenBracket),
               ']' => self.push_delim(Delimiter::CloseBracket),
               _ => {
                // Parsing Number
                let mut c : char = c;
                if c.is_digit(10) {
                    let mut value : i64 = 0; // TODO implement float support
                    while c.is_digit(10) {
                        value *= 10;
                        value += c.to_digit(10).unwrap() as i64;
                        if it.peek().is_none()
                        {
                            break;
                        }
                        c = it.next().unwrap(); 
                        index += 1;
                    }
                    if c.is_alphabetic() { // TODO refactor, very ugly
                        if c == 'i' {
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
                            return Lexer::unexpected_char(c, index);
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
                   chars.push(c);
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
            index += 1;
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
        let mut lex = Lexer::default();
        let pattern = String::from("239487329");
        let repr = Vec::from([Token::Literal(Literal::Number(239487329))]);
        let _ = lex.tokenize(&pattern);
        assert_eq!(lex.tokens, repr);
    }

    #[test]
    fn test_complex_simple()
    {
        let mut lex = Lexer::default();
        let pattern = String::from("239487329i");
        let repr = Vec::from([Token::Literal(Literal::Complex(239487329))]);
        let _ = lex.tokenize(&pattern);
        assert_eq!(lex.tokens, repr);
    }

}
