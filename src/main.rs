use std::io;

mod lex;
mod parser;
mod tok;

fn main() -> Result<(), u32> {
    let stdin = io::stdin();
    let mut buff : String = String::default();
    let mut lexer = lex::Lexer::new();
    let mut parser = parser::Parser::new();
    loop {
        stdin.read_line(&mut buff).unwrap();
        let line = buff.strip_suffix('\n').unwrap();
        dbg!(line);
        match lexer.tokenize(&buff)
        {
            Ok(()) => {
                for tok in lexer.tokens.iter() {
                    println!("{:?}", tok);
                }
            },
            Err(err) => eprintln!("Error while lexing : {}", err)
        }

        buff.clear();
        lexer.flush();
    }
}
