pub mod token;
use token::Token;
use std::io;
use std::io::Write;

// struct
#[derive(Debug, Copy, Clone)]
pub struct Lexer {
    current_token: Token
}

// interface

pub trait Gettok {
    fn gettok(&mut self) -> Token;
}

pub trait GetNextToken {
    fn get_next_token(&mut self) -> Token;
}

pub trait Copy {
    fn copy(&self) -> Lexer;
}

pub trait Clone {
    fn clone(&self) -> Lexer;
}

// Implement

impl Lexer {
    pub fn new() -> Lexer {
        Lexer{
            current_token: Token::TokEof
        }
    }
}

impl Gettok for Lexer {
    fn gettok(&mut self) -> Token {
        
        print!("Ready >> ");
        let _ = io::stdout().flush();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }

        return self.current_token.clone();
    }
}

impl GetNextToken for Lexer {
    fn get_next_token(&mut self) -> Token {
        self.current_token = self.gettok();
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