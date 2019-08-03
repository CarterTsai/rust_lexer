pub mod token;
use token::Token;
use std::io;
use std::io::Write;
use std::io::Read;

// struct
#[derive(Debug, Copy, Clone)]
pub struct Lexer {
    current_token: Token,
}

// interface

pub trait Tokenize {
    fn tokenize(self, chars: &str) -> Vec<String>;
}

pub trait ReadFromTokens {
    fn read_from_tokens(&mut self, tokens: &mut Vec<String>) -> Vec<String>;
}

pub trait Atom {
    fn atom(&mut self, token: &str) -> Vec<String>;
}

pub trait Parse {
    fn parse(&mut self) -> Vec<Token>;
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
        Lexer {
            current_token: Token::End
        }
    }
}

impl Tokenize for Lexer {
    fn tokenize(self, chars: &str) -> Vec<String> {
        let replace_char: &str = &chars.replace("(", "( ").replace(")", " )");
        let mut data: Vec<String> = Vec::new();
        let x = replace_char.split(" ");
        for d in x  {
            data.push(d.to_string());
        }
        data
    }
}

impl ReadFromTokens for Lexer {
    fn read_from_tokens(&mut self, tokens: &mut Vec<String>) -> Vec<String> {
        let collect_tokens: Vec<String> = vec![];

        if tokens.len() == 0 {
            return collect_tokens;
        }

        let token = tokens.drain(0..1).next().unwrap();
        
        if token == "(" {
            println!("{:?}", token);
        } else if token == ")" {
            return collect_tokens;
        } else {
            return self.atom(&token);
        }

        collect_tokens
    }
}

impl Atom for Lexer {
    fn atom(&mut self, _token: &str) -> Vec<String> {
        let collect_tokens: Vec<String> = vec![];
        return collect_tokens;
    }
}

impl Parse for Lexer {
    fn parse(&mut self) -> Vec<Token> {
        let mut collect_tokens: Vec<Token> = vec![];
        print!("Ready >> ");
        let _ = io::stdout().flush();

        let mut input = [0];

        while let Ok(_) = io::stdin().read(&mut input) {
            let input_char = input[0] as char;
            println!("CHAR {:?}", input[0] as char);

            // Digit
            if input_char.is_digit(10) {
                collect_tokens.push(Token::Integer);
            }
            // EOF
            if input_char == ';' {
                collect_tokens.push(Token::End);
                return collect_tokens;
            }
        }

        return collect_tokens;
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