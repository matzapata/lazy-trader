use crate::domain::market::kline_data::MarketKlineData;

#[derive(Debug)]
pub enum IndicatorSentiment {
    Bullish,
    Bearish,
    Neutral,
}

#[derive(Debug)]
pub struct IndicatorResult {
    pub value: Vec<f64>,
    pub sentiment: IndicatorSentiment,
}

pub trait TIndicator {
    fn info(&self) -> &'static str;
    fn compute(&self, data: &Vec<MarketKlineData>) -> Vec<IndicatorResult>;
}
