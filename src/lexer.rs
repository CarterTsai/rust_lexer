// interface

pub trait Gettok {
    fn gettok(&self) -> i64;
}

pub trait GetNextToken {
    fn get_next_token(&self) -> i64;
}

// struct
#[derive(Debug)]
pub struct Lexer {
    current_token: i64
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer{
            current_token: 0
        }
    }
}

impl Gettok for Lexer {
    fn gettok(&self) -> i64 {
        return self.current_token.clone();
    }
}

impl GetNextToken for Lexer {
    fn get_next_token(&self) -> i64 {
        return self.current_token.clone();
    }
}

pub mod token;