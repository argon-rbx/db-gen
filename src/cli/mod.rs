use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod generate;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn main(self) -> Result<()> {
        match self.command {
            Command::Generate(generate) => generate.main(),
        }
    }
}

#[derive(Subcommand)]
pub enum Command {
    Generate(generate::Generate),
}
