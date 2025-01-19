use async_trait::async_trait;
use clap::Args;
use console::style;

use crate::cli::RunCommand;
use super::error::CliError;

#[derive(Args, Debug)]
pub struct Sentiment {
    token: Option<String>,
}

#[async_trait]
impl RunCommand for Sentiment {
    async fn run(self) -> Result<(), CliError> {
        println!("{}", style("Successfully signed in").yellow());

        Ok(())
    }
}