use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "nero",
    author = "arsa",
    version,
    about = "A simple tool for fetching HTTP requests"
)]
pub struct NeroArgs {
    // /// Enable verbose logging
    // #[arg(short, long)]
    // pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Execute request from file
    Run {
        /// Path to request file
        file: String,
    },

    /// Execute request from line (WIP)
    Fetch {
        /// HTTP method
        #[arg(short, long, default_value = "GET")]
        method: String,

        /// Request timeout (seconds)
        #[arg(short, long)]
        timeout: Option<u64>,

        /// Target URL (positional argument)
        url: String,
    },

    /// Compile file request to native (WIP)
    Compile {
        /// Input file
        file: String,
    },
}
