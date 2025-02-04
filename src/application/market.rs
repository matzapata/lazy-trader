// TODO: calculate overlapping sentiments

// expected result:
// - RSI [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - EMA [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - MACD [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - BOLL [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - SMA [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]
// - Overlapping [ { "sentiment": "sell", "from": "Timestamp", "to": "Timestamp" }, { "sentiment": "buy", "from": "Timestamp", "to": "Timestamp"} ]

use std::collections::HashMap;
use std::sync::Arc;

use crate::domain::{
    indicators::indicator::{IndicatorResult, TIndicator},
    market::{
        kline_data::{MarketKlineData, TMarketKlineDataRepo},
        market::Market,
    },
};

pub struct MarketSentimentService<R>
where
    R: TMarketKlineDataRepo,
{
    market_data_repo: R,
}

impl<R> MarketSentimentService<R>
where
    R: TMarketKlineDataRepo,
{
    pub fn new(market_data_repo: R) -> Self {
        MarketSentimentService { market_data_repo }
    }

    pub async fn fetch_market_data(&self, market: &Market) -> Vec<MarketKlineData> {
        self.market_data_repo
            .get_klines(market)
            .await
            .unwrap_or(vec![])
    }
}
