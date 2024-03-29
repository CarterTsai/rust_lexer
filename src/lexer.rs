pub mod token;
use token::Token;

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
    fn parse(&mut self, _token: &str) -> Vec<String>;
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
        let replace_char: &str = &chars.replace("(", " ( ").replace(")", " ) ");
        let mut data: Vec<String> = Vec::new();
        let x = replace_char.split(" ");
        for d in x  {
            data.push(d.to_string());
        }
        data
    }
}

impl ReadFromTokens for Lexer {
    fn read_from_tokens(&mut self, mut tokens: &mut Vec<String>) -> Vec<String> {
        let collect_tokens: Vec<String> = vec![];

        if tokens.len() ==1 && tokens[0] == "" {
            return collect_tokens;
        }

        if tokens.len() == 0 {
            return collect_tokens;
        }

        let mut token = tokens.drain(0..1).next().unwrap();
     
        if token == ""{
            token = tokens.drain(0..1).next().unwrap();
        }

        if token == "(" {
            let mut l: Vec<String> = vec![];
            while tokens[0] != ")" {
                l.append(&mut self.read_from_tokens(&mut tokens));
                println!("l is {:?}", l);
                if tokens.len() == 0 {
                    break;
                }
                tokens.drain(0..0);
                if tokens[0] == "" {
                    tokens.drain(0..0);
                }

                if tokens.len() ==0 {
                    break;
                }
            }

            return l;
        } else if token == ")" {
            return collect_tokens;
        } else {
            // println!("token is {}", token);
            return self.atom(&token);
        }
        // collect_tokens
    }
}

impl Atom for Lexer {
    fn atom(&mut self, _token: &str) -> Vec<String> {
        println!("atom token {:?}", _token);
        let mut collect_tokens: Vec<String> = vec![];
        collect_tokens.push(_token.to_string());
        return collect_tokens;
    }
}

impl Parse for Lexer {
    fn parse(&mut self, _token: &str) -> Vec<String> {
        let mut tokens: Vec<String> = self.tokenize(_token); 
        println!("parse string: {:?}", _token);
        let data: Vec<String> = self.read_from_tokens(&mut tokens);
        data
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