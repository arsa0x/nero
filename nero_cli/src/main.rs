use clap::Parser;
use nero_core::lexer::Lexer;
use std::fs;

mod args;

#[tokio::main]
async fn main() {
    let cli = args::NeroArgs::parse();
    match cli.command {
        args::Commands::Run { file } => {
            let source = fs::read_to_string(file);
            match source {
                Ok(s) => exec(&s).await,
                Err(_) => println!("File not found"),
            }
        }
    }
}

async fn exec(source: &str) {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let pars = nero_core::parser::Parser::new(tokens).parse().unwrap();
    for par in pars {
        // let inter = nero_core::interpreter::Interpreter::new().eval_stmt(&par);
        let _ = nero_core::executor::Executor::new().execute(&par).await;
    }
}
