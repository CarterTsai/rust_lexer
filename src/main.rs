mod lexer;
use lexer::*;
// use lexer::token::Token as TokenEnum;

fn main_loop() {

  let mut lexer = Lexer::new();
  let z = lexer.parse("(begin (define r 10) (* pi (* r r)))");
  println!("Result {:?}", z);
}

fn main() {
  main_loop();
}
