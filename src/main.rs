use crate::lexer::Lexer;

mod lexer;
mod token;
const SOURCE: &str = include_str!("../syntax/example/new/syntax.ns");

fn main() {
    println!("{}", SOURCE);

    let mut lex = Lexer::new(SOURCE);
    let token = lex.tokenize();
    println!("{:?}", token);
}
