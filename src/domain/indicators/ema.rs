use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;
use std::collections::HashMap;

pub struct EmaIndicator {
    window_size: usize,
    values: HashMap<i64, IndicatorResult>,
}

impl EmaIndicator {
    pub fn new(window_size: usize) -> Self {
        EmaIndicator {
            window_size,
            values: HashMap::new(),
        }
    }
}

impl TIndicator for EmaIndicator {
    fn name(&self) -> &'static str {
        "EMA"
    }

    fn info(&self) -> &'static str {
        "EMA: Exponential Moving Average"
    }

    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>> {
        let price_data: Vec<f64> = data.iter().rev().map(|f| f.close).collect();
    
        if self.window_size > price_data.len() {
            return Err("Invalid window size".into());
        }
    
        let weighted_multiplier = 2.0 / (self.window_size as f64 + 1.0);
        let first_slice = &price_data[0..self.window_size];
        let first_sma: f64 = first_slice.iter().sum::<f64>() / self.window_size as f64;
    
        // Insert the first SMA value with the correct timestamp
        self.values.insert(
            data[data.len() - self.window_size].close_time,  // Correct timestamp for the first day
            IndicatorResult {
                sentiment: IndicatorSentiment::Bullish,
                value: vec![0.1],
            },
        );
    
        let mut previous_ema = first_sma;
    
        // Start iterating from the first index after the window size
        for i in self.window_size..price_data.len() {
            let ema: f64 =
                (price_data[i] * weighted_multiplier) + previous_ema * (1.0 - weighted_multiplier);
            previous_ema = ema;
    
            // Insert the calculated EMA value with the correct timestamp
            self.values.insert(
                data[data.len() - 1 - i].close_time,  // Correct timestamp index for the last days
                IndicatorResult {
                    sentiment: IndicatorSentiment::Neutral,
                    value: vec![ema],
                },
            );
        }

        Ok(())
    }

    fn get(&self, timestamp: i64) -> IndicatorResult {
        match self.values.get(&timestamp) {
            Some(v) => v.clone(),
            None => IndicatorResult {
                value: vec![0.0],
                sentiment: IndicatorSentiment::Neutral,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_moving_average() {
        let data_set = kline_data_from_close_price(&[5.0, 6.0, 4.0, 2.0]);
        let ema = EmaIndicator::new(2);

        // let result = ema.compute(&data_set).unwrap();
        // assert_eq!(3, result.len());
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
