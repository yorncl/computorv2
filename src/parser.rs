use crate::tok::*;
use std::rc::*;

/*

   Grammar TODO probably not complete and quite wrong

   TERMINALS : IDENTIFIER, LITERAL

   statement
   :^ statement $
   |declaration
   |expression
   |equation 

   expression:
    : '(' expression ')'
    | expression binary_operator expression
    | function_identifier
    | matrix
    | LITERAL
    | IDENTIFIER

    matrix:
    : '[' (expression ';')* expression? ']'

    binary_operator:
    : + - * / ^ % ** 

   declaration
   :var_declaration
   |function_declaration 

   var_declaration
   :IDENTIFIER '=' expression


   function_declaration
   : function_identifier '=' expression

   function_identifier
   :IDENTIFIER '(' (function_param,)*function_param? ')'

   function_param
   :IDENTIFIER
   |LITERAL

   equation
   : expression '=' '?' 
   | polynomial_equation

   polynomial_equation:
   : expression '=' IDENTIFIER '?'  // polynomial


   array
   : '[' initializer_list ']'

*/


pub struct Parser<'a>
{
    index : usize,
    tokens : Option<&'a Vec<Token>>
}

impl<'a> Parser<'a>
{
    pub fn new() -> Parser<'a>
    {
        Parser {index : 0, tokens : None}
    }

    pub fn parse(mut self)
    {
    }

    fn peek()
    {
    }

    fn statement(mut self)
    {
    }

    fn declaration(mut self)
    {
    }

    fn expression(mut self)
    {
    }

}
