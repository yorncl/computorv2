// TODO maybe redundant with token types ?
pub enum Binary { 
    Plus,
    Minus,
    Times,
    Div,
    Mod,
    Pow
}

pub enum Unary { 
    Minus
}

pub enum Node {
   Expresssion {
       child: Option<Box<Node>>
   },
   Term {
       op: Binary,
       left: Option<Box<Node>>,
       right: Option<Box<Node>>
   },
   Factor {
       op: Binary,
       left: Option<Box<Node>>,
       right: Option<Box<Node>>
   },
   Unary {
       op: Unary,
       child: Option<Box<Node>>
   },
   Exponent {
       left: Option<Box<Node>>,
       right: Option<Box<Node>>
   },
   Literal(i64)
}
