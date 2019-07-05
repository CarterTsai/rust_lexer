mod lexer;
use lexer::*;
use lexer::token::Token as TokenEnum;

fn main_loop() {

  let mut lexer = Lexer::new();
  let mut cur_tokens = lexer.get_next_token();

  while let Some(cur_token) = cur_tokens.pop() {
    match cur_token {
      TokenEnum::End => println!("End!"),
      TokenEnum::Identifier => println!("TokIdentifier!"),
      TokenEnum::Integer => println!("Integer!"),
      TokenEnum::Keyword => println!("Keyword!"),
      TokenEnum::Whitespace => println!("Whitespace"),
      TokenEnum::Operator   => println!("Operator"),
      TokenEnum::Unknow     => println!("Unknow"),
      TokenEnum::Bracket    => println!("Bracket"),
      TokenEnum::Comma      => println!("Comma"),
      TokenEnum::Quotation  => println!("Quotation"),
    }
  }
}

fn main() {
  main_loop();
}
