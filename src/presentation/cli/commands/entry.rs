use super::error::CliError;
use crate::cli::RunCommand;
use async_trait::async_trait;
use clap::Args;

#[derive(Args, Debug)]
pub struct EntryCmd {
    market: String,
    stop_loss: Option<f64>,
    risk: Option<f64>,
    amount: Option<f64>,
}

#[async_trait]
impl RunCommand for EntryCmd {
    async fn run(self) -> Result<(), CliError> {
        let entry_service = lt::application::entry::EntryService::new(
            lt::infrastructure::domain::market::binance_repo::BinanceMarketKlineDataRepo::new(
                reqwest::Client::new(),
            ),
            lt::infrastructure::domain::config::InmemoryConfigRepository::default(),
        );

        let market = lt::domain::market::market::Market::new(
            self.market.clone(),
            lt::domain::market::market::Interval::D1,
            100,
        );

        let entry = entry_service
            .compute_entry(&market, self.amount.unwrap_or(1.0))
            .await;

        println!("With an stop loss at {}, take profit at {} and an entry price of {}, expected profit is {} and expected loss is {}", entry.stop_loss, entry.take_profit, entry.entry_price, entry.expected_profit, entry.expected_loss);

        Ok(())
    }
}
