use clap::Parser;

use crate::{cli::Cli, utils::Utils};

mod cli;
mod utils;
// use nero_core::{
//     ast::Stmt, lexer::Lexer, parser::Parser, resolver::Resolver, semantic::SemanticChecker,
// };
// use nero_requests::executor::Executor;

// const EXAMPLE: &str = include_str!("../../../dev/sample/simple_get.ns");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        cli::Commands::Compile { file } => {
            println!("compile {}", file)
        }
        cli::Commands::Fetch {
            method,
            #[allow(unused)]
            timeout,
            url,
        } => {
            println!("fetching {} {}", method, url);
        }
        cli::Commands::Run { file } => {
            let _ = Utils::run_file(&file).await?;
        }
    }

    Ok(())
}
