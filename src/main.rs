use anyhow::Result;
use clap::Parser as _;

use crate::cli::Cli;

mod cli;
mod commands;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let cli = Cli::parse();
    cli.execute().await?;
    Ok(())
}
