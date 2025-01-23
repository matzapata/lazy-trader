// TODO: calculate overlapping sentiments

// expected result:
// - RSI [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - EMA [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - MACD [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - BOLL [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - SMA [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - Overlapping [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]

use crate::domain::{
    indicators::indicator::{IndicatorResult, TIndicator},
    market::{kline_data::TMarketKlineDataRepo, market::Market},
};

pub struct MarketSentimentService<R>
where
    R: TMarketKlineDataRepo,
{
    indicators: Vec<Box<dyn TIndicator>>,
    market_data_repo: R,
}

pub struct MarketIndicator {
    pub label: String,
    pub data: Vec<IndicatorResult>,
}

impl<R> MarketSentimentService<R>
where
    R: TMarketKlineDataRepo,
{
    pub fn new(market_data_repo: R, indicators: Vec<Box<dyn TIndicator>>) -> Self {
        MarketSentimentService {
            indicators,
            market_data_repo,
        }
    }

    pub async fn analyze_market(&self, market: &Market) -> Vec<MarketIndicator> {
        let data = self.market_data_repo.get_klines(market).await.unwrap();

        let mut result: Vec<MarketIndicator> = Vec::new();
        for indicator in &self.indicators {
            let i_res = indicator.compute(&data).unwrap();
            result.push(MarketIndicator {
                label: indicator.name().to_string(),
                data: i_res,
            });
        }

        // TODO: joint analysis, overlapping, golder cross, death cross, etc

        result
    }
}
