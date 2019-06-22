mod lexer_enum;
use lexer_enum::token as l;

fn main() {
  l::map(l::Token::TokDef)
}
