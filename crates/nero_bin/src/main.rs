mod cli;
mod cmds;

use clap::Parser;
use cli::args;

use crate::{
    cli::{args::RunOutputType, output::OutputPrint},
    cmds::run::RunCmd,
};

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
        args::Commands::Run { file, output } => {
            let response = RunCmd::from_file(&file).await?;
            match output {
                RunOutputType::Json => OutputPrint::json(&response),
                RunOutputType::Summary => OutputPrint::summary(&response),
                RunOutputType::Table => OutputPrint::table(&response),
            }
        }
    }

    Ok(())
}
