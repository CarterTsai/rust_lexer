mod lexer;
use lexer::*;
use lexer::token::Token as TokenEnum;

fn main_loop() {
  let lexer = Lexer::new();
  dbg!(lexer.get_next_token());
  let cur_token: TokenEnum;

  cur_token = lexer.get_next_token();

  // loop {
  match cur_token {
      TokenEnum::TokEof => println!("TokEof!"),
      TokenEnum::TokExtern => println!("TokExtern!"),
      TokenEnum::TokIdentifier => println!("TokIdentifier!"),
      TokenEnum::TokNumber => println!("TokNumber!"),
      TokenEnum::TokDef => println!("TokDef!"),
  }
  // }
}

fn main() {
  main_loop();
}
