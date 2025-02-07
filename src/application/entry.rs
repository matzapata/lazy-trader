
// calculate stop loss, take profit, and entry, expected profit, expected loss, etc

use crate::domain::market::{
    kline_data::{MarketKlineData, TMarketKlineDataRepo},
    market::Market,
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
