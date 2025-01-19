// TODO: grab all indicators and compute

// TODO: given sentiments config calculate ranges

// TODO: calculate overlapping sentiments

// expected result:
// - RSI [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - EMA [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - MACD [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - BOLL [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - SMA [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - Overlapping [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]

use crate::domain::{indicators::indicator::{IndicatorResult, TIndicator}, market::{kline_data::TMarketKlineDataRepo, market::Market}};

pub struct MarketSentimentService<T, R>
where
    T: TIndicator,
    R: TMarketKlineDataRepo
{
    indicators: Vec<T>,
    market_data_repo: R
}

impl<T, R> MarketSentimentService<T, R>
where
    T: TIndicator, 
    R: TMarketKlineDataRepo
{
    pub fn new(market_data_repo: R, indicators: Vec<T>) -> Self {
        MarketSentimentService { indicators, market_data_repo }
    }

    pub async fn analyze_market(&self, market: &Market) -> Vec<Vec<IndicatorResult>> {
        let data = self.market_data_repo.get_klines(market).await.unwrap();
        println!("Data: {:?}", data);

        let mut result: Vec<Vec<IndicatorResult>> = Vec::new();
        for indicator in &self.indicators {
            let i_res  = indicator.compute(&data);
            result.push(i_res);
        }

        result
    }
}
