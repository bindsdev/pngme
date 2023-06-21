#![allow(dead_code)]

mod chunk;
mod chunk_type;
mod cli;
mod png;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use clap::Parser;
use cli::{Cli, Subcommand};

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.subcommand {
        Subcommand::Encode(args) => cli::encode(args),
        Subcommand::Decode(args) => cli::decode(args),
        Subcommand::Remove(args) => cli::remove(args),
        Subcommand::Print(args) => cli::print(args),
    }
}
