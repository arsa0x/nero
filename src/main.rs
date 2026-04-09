use nero::lexer::Lexer;

const SOURCE: &str = include_str!("../example/requests.ns");

fn main() {
    println!("{}", SOURCE);

    let mut lex = Lexer::new(SOURCE);
    let token = lex.tokenize();
    println!("{:?}", token);
}
