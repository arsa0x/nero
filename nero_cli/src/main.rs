use clap::Parser;
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
    let tokens = nero_core::lexer::Lexer::new(source).tokenize().unwrap();
    let ast = nero_core::parser::Parser::new(tokens).parse().unwrap();

    let mut interpreter = nero_core::interpreter::Interpreter::new();

    for stmt in &ast {
        let _ = interpreter.eval_stmt(stmt).unwrap();
    }

    let mut executor = nero_core::executor::Executor::new(interpreter);

    for stmt in &ast {
        let _ = executor.execute(stmt).await;
    }
}
