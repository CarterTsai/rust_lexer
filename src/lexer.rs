pub mod token;
use token::Token;
// interface

pub trait Gettok {
    fn gettok(&self) -> Token;
}

pub trait GetNextToken {
    fn get_next_token(&self) -> Token;
}

pub trait Copy {
    fn copy(&self) -> Lexer;
}

pub trait Clone {
    fn clone(&self) -> Lexer;
}

// struct
#[derive(Debug, Copy, Clone)]
pub struct Lexer {
    current_token: Token
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer{
            current_token: Token::TokEof
        }
    }
}

impl Gettok for Lexer {
    fn gettok(&self) -> Token {
        return self.current_token.clone();
    }
}

impl GetNextToken for Lexer {
    fn get_next_token(&self) -> Token {
        return self.current_token.clone();
    }
}

impl Copy for Lexer { 
    fn copy(&self) -> Lexer {
        *self
    }
}

impl Clone for Lexer {
    fn clone(&self) -> Lexer {
        *self
    }
}