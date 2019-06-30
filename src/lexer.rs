pub mod token;
use token::Token;
use std::io;
use std::io::Write;
use std::io::Read;

// struct
#[derive(Debug, Copy, Clone)]
pub struct Lexer {
    current_token: Token
}

// interface

pub trait Gettok {
    fn gettok(&mut self) -> Vec<Token>;
}

pub trait GetNextToken {
    fn get_next_token(&mut self) -> Vec<Token>;
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
    fn gettok(&mut self) -> Vec<Token> {
        let mut collect_tokens: Vec<Token> = vec![];
        print!("Ready >> ");
        let _ = io::stdout().flush();

        let mut input = [0];

        while let Ok(_) = io::stdin().read(&mut input) {
            let input_char = input[0] as char;
            println!("CHAR {:?}", input[0] as char);

            if input_char == '+' { collect_tokens.push(Token::TokIdentifier) };
            if input_char == '-' { collect_tokens.push(Token::TokIdentifier) };
            if input_char == '*' { collect_tokens.push(Token::TokIdentifier) };
            if input_char == '/' { collect_tokens.push(Token::TokIdentifier) };

            // Digit
            if input_char.is_digit(10) {
                collect_tokens.push(Token::TokNumber);
            }
            // EOF
            if input_char == ';' {
                collect_tokens.push(Token::TokEof);
                return collect_tokens;
            }
        }

        return collect_tokens;
    }
}

impl GetNextToken for Lexer {
    fn get_next_token(&mut self) -> Vec<Token> {
        return self.gettok();
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