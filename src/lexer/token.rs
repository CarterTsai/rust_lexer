#[allow(dead_code)]
pub enum Token {
  // xxx
  TokEof = -1,
  // commands
  TokDef = -2,
  // xx
  TokExtern = -3,
  // primary
  TokIdentifier = -4,
  // xx
  TokNumber = -5,
}

pub fn map(event: Token) {
  match event {
    Token::TokEof => println!("-1"),
    Token::TokDef => println!("-2"),
    Token::TokExtern => println!("-3"),
    Token::TokIdentifier => println!("-4"),
    Token::TokNumber => println!("-5"),
  }
}