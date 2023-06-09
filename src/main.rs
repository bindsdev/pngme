#![allow(dead_code)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    Ok(())
}
