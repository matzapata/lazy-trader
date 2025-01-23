use async_trait::async_trait;
use clap::Args;
use lt::domain::{indicators::indicator::IndicatorSentiment, market::market::Market};
use prettytable::{color, Attr, Cell, Row, Table};

use super::error::CliError;
use crate::cli::RunCommand;

#[derive(Args, Debug)]
pub struct Sentiment {
    market: Option<String>,
    interval: Option<String>,
    limit: Option<u32>,
}

#[async_trait]
impl RunCommand for Sentiment {
    async fn run(self) -> Result<(), CliError> {
        let client = reqwest::Client::new();
        let market_repo =
            lt::infrastructure::domain::market::binance_repo::BinanceMarketKlineDataRepo::new(
                client,
            );
        let sentiment_analyzer = lt::application::market_sentiment::MarketSentimentService::new(
            market_repo,
            vec![
                Box::new(lt::domain::indicators::rsi::RsiIndicator::new(14)),
                Box::new(lt::domain::indicators::sma::SmaIndicator::new(14)),
            ],
        );

        let mut markets: Vec<Market> = vec![];
        if let Some(mkt) = self.market {
            println!("Analyzing market: {}", mkt);

            markets.push(Market {
                id: "BTCUSDT".to_string(),
                interval: "1d".to_string(),
                limit: 100,
            });
        } else {
            println!("Analyzing all tokens...");
        }

        for market in markets {
            let result = sentiment_analyzer.analyze_market(&market).await;

            let mut table = Table::new();

            // Build the header row
            let header: Row = Row::new(
                result
                    .iter()
                    .map(|indicator| Cell::new(&indicator.label))
                    .collect(),
            );
            table.add_row(header);

            // Find the maximum number of rows
            let max_rows = result
                .iter()
                .map(|indicator| indicator.data.len())
                .max()
                .unwrap_or(0);

            // Add rows of data
            for i in 0..max_rows {
                let row: Row = Row::new(
                    result
                        .iter()
                        .map(|indicator| {
                            if let Some(data) = indicator.data.get(i) {
                                Cell::new(&format!("{}", data)).with_style(Attr::ForegroundColor(
                                    match data.sentiment {
                                        IndicatorSentiment::Bullish => color::GREEN,
                                        IndicatorSentiment::Bearish => color::RED,
                                        IndicatorSentiment::Neutral => color::YELLOW,
                                    },
                                ))
                            } else {
                                Cell::new("") // Empty cell for missing data
                            }
                        })
                        .collect(),
                );
                table.add_row(row);
            }

            table.printstd();
        }

        Ok(())
    }
}
