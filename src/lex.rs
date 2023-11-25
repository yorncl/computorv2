use crate::tok::{*, self};


fn unexpected_char(c : char, index : usize) -> Result<(Vec<Token>), String> {
    Err(format!("Unexpected character '{}' at index {}", c, index))
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, String>
{
    let mut index : usize = 0;
    let mut it = input.chars().peekable();

    let mut tokens : Vec<Token> = vec![];
    while let Some(c) = it.peek()
    {
        match c {
           '\n' => break,
           '=' => { tokens.push(Token::Equal); it.next(); index += 1; },
           '+' => { tokens.push(Token::Plus); it.next(); index += 1;},
           '-' => { tokens.push(Token::Minus); it.next(); index += 1;},
           '*' => {
               it.next();
               index+=1;
               match it.peek()
               {
                   Some(c) => {
                        if *c == '*' {
                            tokens.push(Token::MatrixTimes);
                            it.next();
                            index += 1;
                        }
                        else
                        {
                            tokens.push(Token::Times);
                        }
                   },
                   None => tokens.push(Token::Times)
               }
           }
           '%' => { tokens.push(Token::Modulo); it.next(); index += 1;},
           '/' => { tokens.push(Token::Divide); it.next(); index += 1;},
           '^' => { tokens.push(Token::Power); it.next(); index += 1;},
           ';' => { tokens.push(Token::SemiColon); it.next(); index += 1;},
           ',' => { tokens.push(Token::Comma); it.next(); index += 1;},
           '?' => { tokens.push(Token::Interrogation); it.next(); index += 1;},
           '(' => { tokens.push(Token::OpenParenthese); it.next(); index += 1;},
           ')' => { tokens.push(Token::CloseParenthese); it.next(); index += 1;},
           '[' => { tokens.push(Token::OpenBracket); it.next(); index += 1;},
           ']' => { tokens.push(Token::CloseBracket); it.next(); index += 1;},
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
                                    tokens.push(Token::Complex(value))
                                },
                                None => tokens.push(Token::Complex(value))
                            }
                        }
                        else {
                            return unexpected_char(*c, index);
                        }
                    }
                    else {
                        tokens.push(Token::Number(value));
                    }
                }
                else 
                {
                    tokens.push(Token::Number(value))
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
        let repr = Vec::from([Token::Number(239487329)]);
        let tokens = tokenize(&pattern).unwrap();
        assert_eq!(tokens, repr);
    }

    #[test]
    fn test_complex_simple()
    {
        let pattern = String::from("239487329i");
        let repr = Vec::from([Token::Complex(239487329)]);
        let tokens = tokenize(&pattern).unwrap();
        assert_eq!(tokens, repr);
    }

    #[test]
    fn test_mult_simple()
    {
        let pattern = String::from("6*42");

        let repr : Vec<Token> = vec![Token::Number(6), Token::Times, Token::Number(42)];
        let tokens = tokenize(&pattern).unwrap();
        assert_eq!(tokens, repr);
    }

}
