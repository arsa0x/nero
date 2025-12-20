mod cli;
mod cmds;

use clap::Parser;
use cli::args;

use crate::{cli::output::OutputPrint, cmds::run::RunCmd};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = args::NeroArgs::parse();
    #[allow(unused)]
    match cli.command {
        args::Commands::Compile { file } => {
            println!("WIP");
        }
        args::Commands::Fetch {
            method,
            timeout,
            url,
        } => {
            println!("WIP");
        }
        args::Commands::Run { file } => {
            let response = RunCmd::from_file(&file).await?;
            let _ = OutputPrint::table_summary(response);
        }
    }

    Ok(())
}
