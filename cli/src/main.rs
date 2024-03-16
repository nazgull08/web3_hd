pub mod cli;
pub mod commands;
pub mod error;
pub mod settings;

use cli::{handle_command, Cli};
use clap::Parser;
use error::Error;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let settings = Settings::new()?;
    handle_command(args, settings).await?;
    Ok(())
}
