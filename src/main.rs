mod lexer;
use lexer::*;
use lexer::token as token;
use lexer::token::Token as TokenEnum;

fn main() {
  token::map(TokenEnum::TokDef);
  let lexer = Lexer::new();
  dbg!(lexer.get_next_token());
}
