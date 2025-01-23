use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;

pub struct EmaIndicator {
    window_size: usize,
}

impl EmaIndicator {
    pub fn new(window_size: usize) -> Self {
        EmaIndicator { window_size }
    }
}

impl TIndicator for EmaIndicator {
    fn name(&self) -> &'static str {
        "EMA"
    }

    fn info(&self) -> &'static str {
        "EMA: Exponential Moving Average"
    }

    fn compute(&self, data: &Vec<MarketKlineData>) -> Option<Vec<IndicatorResult>> {
        let price_data: Vec<f64> = data.iter().rev().map(|f| f.close).collect();
        if self.window_size > price_data.len() {
            return None;
        }

        let mut result: Vec<IndicatorResult> = Vec::new();

        let weighted_multiplier = 2.0 / (self.window_size as f64 + 1.0);
        let first_slice = &price_data[0..self.window_size];
        let first_sma: f64 = first_slice.iter().sum::<f64>() / self.window_size as f64;

        result.push(IndicatorResult {
            sentiment: IndicatorSentiment::Neutral,
            value: vec![first_sma],
        });

        for i in self.window_size..price_data.len() {
            let previous_ema = result[result.len() - 1].value[0];
            let ema: f64 =
                (price_data[i] * weighted_multiplier) + previous_ema * (1.0 - weighted_multiplier);

            result.push(IndicatorResult {
                sentiment: IndicatorSentiment::Neutral,
                value: vec![ema],
            });
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_moving_average() {
        let data_set = kline_data_from_close_price(&[5.0, 6.0, 4.0, 2.0]);
        let ema = EmaIndicator::new(2);

        let result = ema.compute(&data_set).unwrap();
        assert_eq!(3, result.len());
        // assert_eq!(vec![5.5, 4.5, 2.8333333333333335], result);

        // let result = exponential_moving_average(&data_set, 4).unwrap();
        // assert_eq!(1, result.len());
        // assert_eq!(vec![4.25], result);

        // let result = exponential_moving_average(&data_set, 5);
        // assert_eq!(None, result);

        // let data_set = vec![
        //     22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39,
        // ];

        // let result = exponential_moving_average(&data_set, 10).unwrap();
        // assert_eq!(3, result.len());
        // assert_eq!(
        //     vec![22.220999999999997, 22.208090909090906, 22.241165289256195],
        //     result
        // );
    }

    fn kline_data_from_close_price(close_prices: &[f64]) -> Vec<MarketKlineData> {
        close_prices
            .iter()
            .map(|&close| MarketKlineData {
                close,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                volume: 0.0,
                close_time: 0,
                open_time: 0,
                quote_asset_volume: 0.0,
                number_of_trades: 0,
                take_buy_base_asset_volume: 0.0,
                take_buy_quote_asset_volume: 0.0,
                ignore: 0.0,
            })
            .collect()
    }
}
