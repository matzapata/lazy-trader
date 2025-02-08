use super::error::CliError;
use crate::cli::RunCommand;
use async_trait::async_trait;
use chrono::DateTime;
use clap::Args;
use clap::ValueEnum;
use lt::domain::{
    indicators::indicator::IndicatorSentiment,
    market::market::{Interval, Market},
};
use prettytable::format;
use prettytable::{color, Attr, Cell, Row, Table};
use terminal_size::{terminal_size, Width};

#[derive(Args, Debug)]
pub struct SentimentCmd {
    market: Option<String>,

    #[arg(long, default_value_t = 200)]
    limit: u32,

    #[arg(long)]
    interval: SentimentCmdInterval,

    #[arg(long, default_value_t = false)]
    neutral: bool,

    #[arg(long, default_value_t = false)]
    info: bool,

}

#[derive(Debug, Clone, ValueEnum)]
pub enum SentimentCmdInterval {
    D1,
    H1,
}
impl Into<Interval> for SentimentCmdInterval {
    fn into(self) -> Interval {
        match self {
            SentimentCmdInterval::D1 => Interval::D1,
            SentimentCmdInterval::H1 => Interval::H1,
        }
    }
}

#[async_trait]
impl RunCommand for SentimentCmd {
    async fn run(self) -> Result<(), CliError> {
        let client = reqwest::Client::new();
        let market_repo =
            lt::infrastructure::domain::market::binance_repo::BinanceMarketKlineDataRepo::new(
                client,
            );
        let market_data_service = lt::application::market::MarketSentimentService::new(market_repo);
        let config_repo = lt::infrastructure::domain::config::InmemoryConfigRepository::default();
        let config_service = lt::application::config::ConfigService::new(config_repo);
        let mut indicators: Vec<Box<dyn lt::domain::indicators::indicator::TIndicator>> = vec![
            Box::new(lt::domain::indicators::rsi::RsiIndicator::new(14)),
            Box::new(lt::domain::indicators::ema::EmaIndicator::new(20, 50)),
            Box::new(lt::domain::indicators::macd::MacdIndicator::new(12, 26, 9)),
            Box::new(lt::domain::indicators::bb::BoilingBandsIndicator::new(20, 2.0, 50)),
        ];

        // select markets to analyze
        let mut markets: Vec<Market> = vec![];
        if let Some(mkt) = self.market {
            markets.push(Market::new(mkt, self.interval.clone().into(), self.limit));
        } else {
            markets = config_service
                .get()
                .await?
                .markets
                .iter()
                .map(|mkt| Market::new(mkt.id.clone(), self.interval.clone().into(), self.limit))
                .collect();
        }

        let interval = match self.interval {
            SentimentCmdInterval::D1 => 24 * 60 * 60 * 1000,
            SentimentCmdInterval::H1 => 60 * 60 * 1000,
        };

        // analyze each market
        for market in markets {
            // fetch market data and compute
            let kline_data = market_data_service.fetch_market_data(&market).await;
            for indicator in &mut indicators {
                indicator.compute(&kline_data).unwrap();
            }

            let start_date = kline_data.first().unwrap().close_time;
            let end_date = kline_data.last().unwrap().close_time;

            // display results ================================================================

            let mut table = Table::new();

            let format = format::FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separators(
                    &[format::LinePosition::Top, format::LinePosition::Bottom],
                    format::LineSeparator::new('-', '+', '+', '+'),
                )
                .padding(1, 1)
                .build();
            table.set_format(format);

            let mut current_time = start_date;
            while current_time < end_date {
                let datetime = DateTime::from_timestamp(current_time / 1000, 0).unwrap();
                let formatted_date = datetime.format("%d-%m-%Y %Hhs").to_string();

                // if show-all is false, skip indicators that are not bullish nor bearish
                if !self.neutral {
                    // skip row if no indicator is bullish n
                    if !indicators.iter().any(|indicator| {
                        indicator.get(current_time).sentiment != IndicatorSentiment::Neutral
                    }) {
                        current_time += interval;
                        continue;
                    }
                }

                let cells: Vec<Cell> = std::iter::once(Cell::new(formatted_date.as_str()))
                    .chain(indicators.iter().map(|indicator| {
                        let data = indicator.get(current_time);

                        Cell::new(&format!("{}", data)).with_style(Attr::ForegroundColor(
                            match data.sentiment {
                                IndicatorSentiment::Bullish => color::GREEN,
                                IndicatorSentiment::Bearish => color::RED,
                                IndicatorSentiment::Neutral => color::YELLOW,
                            },
                        ))
                    }))
                    .collect();

                table.add_row(Row::new(cells));

                current_time += interval;
            }

            // Build the header row
            let header: Row = Row::new(
                std::iter::once(Cell::new("Date"))
                    .chain(
                        indicators
                            .iter()
                            .map(|indicator| Cell::new(&indicator.name())),
                    )
                    .collect(),
            );
            table.add_row(header);

            // Print the table
            table.printstd();

            // print descriptions
            if self.info {
                println!();
                print_divider();
                for indicator in &indicators {
                    println!("{}", indicator.info());
                    print_divider();
                }
            }
        }

        Ok(())
    }
}

fn print_divider() {
    let width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80, // Default to 80 if size can't be determined
    };
    println!("{}", "-".repeat(width));
}
