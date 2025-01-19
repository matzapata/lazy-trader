use async_trait::async_trait;
use clap::Args;
use console::style;
use lt::domain::market::market::Market;

use super::error::CliError;
use crate::cli::RunCommand;

#[derive(Args, Debug)]
pub struct Sentiment {
    token: Option<String>,
}

#[async_trait]
impl RunCommand for Sentiment {
    async fn run(self) -> Result<(), CliError> {

        let client = reqwest::Client::new();
        let market_repo = lt::infrastructure::domain::market::binance_repo::BinanceMarketKlineDataRepo::new(client);
        let sentiment_analyzer = lt::application::market_sentiment::MarketSentimentService::new(
            market_repo,
            vec![lt::domain::indicators::rsi::RsiIndicator::new(14)],
        );

        if let Some(token) = self.token {
            println!("Analyzing token: {}", token);
            
            let market = Market {
                id: "BTCUSDT".to_string(),
                interval: "1d".to_string(),
                limit: 100,
            };

            let result = sentiment_analyzer.analyze_market(&market).await;
            println!("Result: {:?}", result[0]);
        } else {
            println!("Analyzing all tokens...");
        }


        Ok(())
    }
}
