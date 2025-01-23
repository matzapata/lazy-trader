use async_trait::async_trait;
use clap::Args;
use console::style;
use lt::domain::market::market::Market;

use super::error::CliError;
use crate::cli::RunCommand;

#[derive(Args, Debug)]
pub struct Position {
    variant: String, // Short or long
    market: String,
    stop_loss: Option<f64>,
    risk: Option<f64>,
    amount: Option<f64>,
}

#[async_trait]
impl RunCommand for Position {
    async fn run(self) -> Result<(), CliError> {
        // stop loss from param of config
        // risk from param of config

        // retrieve current price

        // calculate stop loss, take profit, and entry
        // if amount calculate potential profit and loss

        Ok(())
    }
}
