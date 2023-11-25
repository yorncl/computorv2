use std::io;

mod lex;
mod parser;
mod tok;

fn main() -> Result<(), u32> {
    let stdin = io::stdin();
    let mut buff : String = String::default();
    let mut parser = parser:: Parser::new();
    loop {
        stdin.read_line(&mut buff).unwrap();
        let line = buff.strip_suffix('\n').unwrap();
        dbg!(line);
        match lex::tokenize(&buff)
        {
            Ok(tokens) => {
                for tok in tokens.iter() {
                    println!("{:?}", tok);
                }
                // let ast = parser::parse(tokens);
            },
            Err(err) => { eprintln!("Error : {}", err); continue }
        }
        buff.clear();
    }
}
