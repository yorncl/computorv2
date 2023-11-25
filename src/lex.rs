use crate::tok::{*, self};


fn unexpected_char(c : char, index : usize) -> Result<(Vec<Token>), String> {
    Err(format!("Unexpected character '{}' at index {}", c, index))
}

fn punct(tok : Punctuation) -> Token { Token::Punctuation(tok) }
fn delim(tok : Delimiter) -> Token { Token::Delimiter(tok) }
fn liter(tok : Literal) -> Token { Token::Literal(tok) }

pub fn tokenize(input: &String) -> Result<Vec<Token>, String>
{
    let mut index : usize = 0;
    let mut it = input.chars().peekable();

    let mut tokens : Vec<Token> = vec![];
    while let Some(c) = it.peek()
    {
        match c {
           '\n' => break,
           '=' => { tokens.push(punct(Punctuation::Equal)); it.next(); index += 1; },
           '+' => { tokens.push(punct(Punctuation::Plus)); it.next(); index += 1;},
           '-' => { tokens.push(punct(Punctuation::Minus)); it.next(); index += 1;},
           '*' => {
               it.next();
               index+=1;
               match it.peek()
               {
                   Some(c) => {
                        if *c == '*' {
                            tokens.push(punct(Punctuation::MatrixTimes));
                            it.next();
                            index += 1;
                        }
                        else
                        {
                            tokens.push(punct(Punctuation::Times));
                        }
                   },
                   None => tokens.push(punct(Punctuation::Times))
               }
           }
           '%' => { tokens.push(punct(Punctuation::Modulo)); it.next(); index += 1;},
           '/' => { tokens.push(punct(Punctuation::Divide)); it.next(); index += 1;},
           '^' => { tokens.push(punct(Punctuation::Power)); it.next(); index += 1;},
           ';' => { tokens.push(punct(Punctuation::SemiColon)); it.next(); index += 1;},
           ',' => { tokens.push(punct(Punctuation::Comma)); it.next(); index += 1;},
           '?' => { tokens.push(punct(Punctuation::Interrogation)); it.next(); index += 1;},
           '(' => { tokens.push(delim(Delimiter::OpenParenthese)); it.next(); index += 1;},
           ')' => { tokens.push(delim(Delimiter::CloseParenthese)); it.next(); index += 1;},
           '[' => { tokens.push(delim(Delimiter::OpenBracket)); it.next(); index += 1;},
           ']' => { tokens.push(delim(Delimiter::CloseBracket)); it.next(); index += 1;},
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
                                        return unexpected_char(*nc, index + 1);
                                    }
                                    tokens.push(liter(Literal::Complex(value)))
                                },
                                None => tokens.push(liter(Literal::Complex(value)))
                            }
                        }
                        else {
                            return unexpected_char(*c, index);
                        }
                    }
                    else {
                        tokens.push(liter(Literal::Number(value)));
                    }
                }
                else 
                {
                    tokens.push(liter(Literal::Number(value)))
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
               tokens.push(Token::Identifier(chars));
            }
           }
        }
   }
   Ok(tokens)
}


#[cfg(test)]
mod test
{
    use super::*;


    #[test]
    fn test_integer_simple()
    {
        let pattern = String::from("239487329");
        let repr = Vec::from([Token::Literal(Literal::Number(239487329))]);
        let tokens = tokenize(&pattern).unwrap();
        assert_eq!(tokens, repr);
    }

    #[test]
    fn test_complex_simple()
    {
        let pattern = String::from("239487329i");
        let repr = Vec::from([Token::Literal(Literal::Complex(239487329))]);
        let tokens = tokenize(&pattern).unwrap();
        assert_eq!(tokens, repr);
    }

    #[test]
    fn test_mult_simple()
    {
        let pattern = String::from("6*42");

        let repr : Vec<Token> = vec![Token::Literal(Literal::Number(6)),
                                Token::Punctuation(Punctuation::Times),
                                Token::Literal(Literal::Number(42))];
        let tokens = tokenize(&pattern).unwrap();
        assert_eq!(tokens, repr);
    }

}
