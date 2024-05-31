use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// Generate reflection database and completion schema
#[derive(Parser)]
pub struct Generate {
    /// Where to output reflection database (.msgpack) and completion schema (.json) files
    #[arg()]
    output: Vec<PathBuf>,

    /// Previous database version to compare with
    #[arg(short, long)]
    ver: Option<String>,
}

impl Generate {
    pub fn main(self) -> Result<()> {
        println!("test");
        Ok(())
    }
}
