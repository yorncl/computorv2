use crate::tok::*;
use crate::ast::*;

pub struct Parser<'a>
{
    index : usize,
    tokens : &'a Vec<Token>
}

impl<'a> Parser<'a>
{
    pub fn new(tokens : &'a Vec<Token>) -> Parser<'a>
    {
        Parser {index : 0, tokens : tokens}
    }

    pub fn parse(&mut self) -> Node
    {
        self.statement()
    }

    fn check(&self, t : Token) -> bool
    {
        if self.tokens[self.index] == t 
        {
            true
        } 
        else 
        {
            false
        }
    }

    fn curr(&self) -> &Token
    {
        &self.tokens[self.index]
    }

    fn consume(&mut self)
    {
        self.index += 1;
    }

    fn consume_n(&mut self, n : usize)
    {
        self.index += n;
    }

    fn statement(&mut self) -> Node
    {
        self.expression()
    }

    fn expression(&mut self) -> Node
    {
        self.term()
    }

    fn term(&mut self) -> Node
    {
        let u = self.factor();
        while (self.check(Token::Plus) || self.check(Token::Minus))
        {
        }
        Node::Term { op: Binary::Plus, left: None, right: None }
    }

    fn factor(&mut self) -> Node
    {
        Node::Factor { op: Binary::Times, left: None, right: None }
    }
}
