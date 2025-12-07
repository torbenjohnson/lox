use std::env;

use lox::{Cli, CliError};

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum MainError {
    /// Cli error: {0}
    Cli(CliError),
}

fn main() -> Result<(), MainError> {
    Cli::parse(env::args()).map_err(MainError::Cli)?;

    Ok(())
}
