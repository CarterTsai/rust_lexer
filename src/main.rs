mod lexer;
use lexer::*;
use lexer::token::Token as TokenEnum;

fn main_loop() {

  let mut lexer = Lexer::new();
  let mut cur_tokens = lexer.get_next_token();

  while let Some(cur_token) = cur_tokens.pop() {
    match cur_token {
      TokenEnum::TokEof => println!("TokEof!"),
      TokenEnum::TokExtern => println!("TokExtern!"),
      TokenEnum::TokIdentifier => println!("TokIdentifier!"),
      TokenEnum::TokNumber => println!("TokNumber!"),
      TokenEnum::TokDef => println!("TokDef!"),
    }
  }
}

fn main() {
  main_loop();
}
