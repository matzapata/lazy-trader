use super::market::Market;

#[derive(Debug, Clone)]
pub struct MarketKlineData {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: usize,
    pub take_buy_base_asset_volume: f64,
    pub take_buy_quote_asset_volume: f64,
    pub ignore: f64,
}

#[async_trait::async_trait]
pub trait TMarketKlineDataRepo {
    async fn get_klines(&self, market: &Market) -> Option<Vec<MarketKlineData>>;
}
