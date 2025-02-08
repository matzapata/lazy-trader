use crate::domain::market::kline_data::MarketKlineData;

#[derive(Debug, PartialEq, Clone)]
pub enum IndicatorSentiment {
    Bullish,
    Bearish,
    Neutral,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndicatorResult {
    pub value: Vec<f64>,
    pub sentiment: IndicatorSentiment,
}

impl std::fmt::Display for IndicatorResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_values: Vec<String> =
            self.value.iter().map(|&v| format!("{:.2}", v)).collect();
        let formatted_values_str = formatted_values.join(", ");

        // let styled_output = match self.sentiment {
        //     IndicatorSentiment::Bullish => format!("Bullish({})", formatted_values_str).to_string(),
        //     IndicatorSentiment::Bearish => format!("Bearish({})", formatted_values_str).to_string(),
        //     IndicatorSentiment::Neutral => format!("Neutral({})", formatted_values_str).to_string(),
        // };

        let styled_output = match self.sentiment {
            IndicatorSentiment::Bullish => format!("Bullish").to_string(),
            IndicatorSentiment::Bearish => format!("Bearish").to_string(),
            IndicatorSentiment::Neutral => format!("Neutral").to_string(),
        };

        write!(f, "{}", styled_output)
    }
}

pub trait TIndicator: Send + Sync {
    fn name(&self) -> &'static str;
    fn info(&self) -> &'static str;
    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>>;
    fn get(&self, timestamp: i64) -> IndicatorResult;
}
