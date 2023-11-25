use std::io;

mod lex;
mod parser;
mod tok;
mod ast;

use tok::*;
use ast::*;

fn main() -> Result<(), u32> {
    let stdin = io::stdin();
    let mut buff : String = String::default();
    let  mut tokens : Vec<Token> = vec![];
    loop {
        stdin.read_line(&mut buff).unwrap();
        let line = buff.strip_suffix('\n').unwrap();
        dbg!(line);
        let result = lex::tokenize(&buff);
        if result.is_ok()
        {
            tokens = result.unwrap();
            let mut parser = parser::Parser::new(&tokens);
            let _ast = parser.parse();
        }
        else {
            eprintln!("Error : {}", result.unwrap_err());
            continue;
        }
        buff.clear();
    }
}
