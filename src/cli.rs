use crate::{png::*, Result};
use clap::{Args, Parser, Subcommand as ClapSubcommand};
use std::{convert::TryFrom, fs, path::PathBuf, str::FromStr};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand)]
pub enum Subcommand {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

/// Encodes a secret message into the PNG file.
#[derive(Args)]
pub struct EncodeArgs {
    /// Path to PNG file
    png_path: PathBuf,

    /// PNG chunk type
    chunk_type: String,

    /// Message to encode
    message: String,

    /// Optional path to a file where the result will be outputted
    #[arg(short, long)]
    output: Option<PathBuf>,
}

/// Encodes a secret message into the PNG file.
#[derive(Args)]
pub struct DecodeArgs {
    /// Path to PNG file
    png_path: PathBuf,

    /// PNG chunk type
    chunk_type: String,
}

/// Removes a chunk from a PNG file.
#[derive(Args)]
pub struct RemoveArgs {
    /// Path to PNG file
    png_path: PathBuf,

    /// PNG chunk type
    chunk_type: String,
}

/// Print out all of the chunks in a PNG file.
#[derive(Args)]
pub struct PrintArgs {
    /// Path to PNG file
    png_path: PathBuf,
}

/// Encodes a secret message into the PNG file.
pub fn encode(args: EncodeArgs) -> Result<()> {
    let png_bytes = fs::read(args.png_path)?;
    let png_bytes = png_bytes.as_slice();
    let mut png = Png::try_from(png_bytes)?;

    let chunk = Chunk::new(
        ChunkType::from_str(args.chunk_type.as_str())?,
        args.message.as_bytes().to_vec(),
    );

    png.append_chunk(chunk);

    if let Some(out_path) = args.output.as_deref() {
        fs::write(out_path, png.as_bytes())?;
    }

    Ok(())
}

/// Searches a PNG file for a secret message and prints it out if found.
/// Encodes a secret message into the PNG file.
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png_bytes = fs::read(args.png_path)?;
    let png_bytes = png_bytes.as_slice();
    let png = Png::try_from(png_bytes)?;

    let message = png
        .chunks()
        .iter()
        .find(|c| c.chunk_type().to_string() == args.chunk_type)
        .map(|c| c.chunk_type().to_string())
        .ok_or("could not find a chunk with a matching message")?;

    println!("{message}");

    Ok(())
}

/// Removes a chunk from a PNG file.
pub fn remove(args: RemoveArgs) -> Result<()> {
    let png_bytes = fs::read(args.png_path)?;
    let png_bytes = png_bytes.as_slice();
    let mut png = Png::try_from(png_bytes)?;

    let chunk_type_to_remove = ChunkType::from_str(args.chunk_type.as_str())?;

    let to_remove = png
        .chunks()
        .iter()
        .find(|c| *c.chunk_type() == chunk_type_to_remove)
        .ok_or("could not find a chunk with a matching chunk type")?;

    png.remove_chunk(to_remove.chunk_type().to_string().as_str())
        .map(|_| ())
}

/// Print out all of the chunks in a PNG file.
pub fn print(args: PrintArgs) -> Result<()> {
    let png_bytes = fs::read(args.png_path)?;
    let png_bytes = png_bytes.as_slice();
    let png = Png::try_from(png_bytes)?;

    for chunk in png.chunks() {
        println!("{chunk}");
    }

    Ok(())
}
