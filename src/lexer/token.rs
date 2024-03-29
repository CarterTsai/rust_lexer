#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Token {
    Identifier = 1,
    Keyword    = 2,
    Integer    = 3,
    Whitespace = 4,
    Operator   = 5,
    End        = 6,
    Unknow     = 7,
    Bracket    = 8,
    Comma      = 9,
    Quotation  = 10,
}

pub trait Copy {
    fn copy(&self) -> Token;
}

pub trait Clone {
    fn clone(&self) -> Token;
}

// pub fn map(event: Token) {
//   match event {
//     Token::TokEof => println!("-1"),
//     Token::TokDef => println!("-2"),
//     Token::TokExtern => println!("-3"),
//     Token::TokIdentifier => println!("-4"),
//     Token::TokNumber => println!("-5"),
//   }
// }


impl Copy for Token { 
    fn copy(&self) -> Token {
        *self
    }
}

impl Clone for Token {
    fn clone(&self) -> Token {
        *self
    }
}