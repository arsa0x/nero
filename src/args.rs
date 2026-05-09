// src/main.rs

use lexopt::prelude::*;
use std::path::PathBuf;

const HELP: &str = r#"A simple tool for fetching HTTP requests

Usage: nero <COMMAND>

Commands:
  run   Execute request from file
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
"#;

#[derive(Debug)]
enum Command {
    Run { file: PathBuf, test: bool },

    Help,
    Version,
}

fn parse_args() -> Result<Command, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();

    match parser.next()? {
        Some(Value(cmd)) if cmd == "run" => parse_run(parser),

        Some(Value(cmd)) if cmd == "help" => Ok(Command::Help),

        Some(Long("help")) | Some(Short('h')) => Ok(Command::Help),

        Some(Long("version")) | Some(Short('V')) => Ok(Command::Version),

        Some(arg) => Err(arg.unexpected()),

        None => Ok(Command::Help),
    }
}

fn parse_run(mut parser: lexopt::Parser) -> Result<Command, lexopt::Error> {
    let mut test = false;
    let mut file: Option<PathBuf> = None;

    while let Some(arg) = parser.next()? {
        match arg {
            Long("test") => {
                test = true;
            }

            Value(value) => {
                file = Some(value.into());
            }

            _ => return Err(arg.unexpected()),
        }
    }

    let file = file.ok_or("missing file argument")?;

    Ok(Command::Run { file, test })
}

fn print_help() {
    println!("{HELP}");
}

fn main() {
    match parse_args() {
        Ok(command) => match command {
            Command::Run { file, test } => {
                println!("Running file: {}", file.display());

                if test {
                    println!("Test mode enabled");
                }

                // Execute runtime here
            }

            Command::Help => {
                print_help();
            }

            Command::Version => {
                println!("nero v{}", env!("CARGO_PKG_VERSION"));
            }
        },

        Err(err) => {
            eprintln!("error: {err}");
            eprintln!();
            print_help();

            std::process::exit(1);
        }
    }
}
