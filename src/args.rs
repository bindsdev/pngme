use clap::{Args, Parser, Subcommand as ClapSubcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(ClapSubcommand)]
pub enum Subcommand {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    png_path: PathBuf,
    chunk_type: String,
    message: String,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    png_path: PathBuf,
    chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    png_path: PathBuf,
    chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    png_path: PathBuf,
}
