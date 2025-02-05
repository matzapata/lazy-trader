use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;
use std::collections::HashMap;

// TODO: add slow, fast ema

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
        let price_data: Vec<f64> = data.iter().map(|f| f.close).collect();
    
        if self.window_size > price_data.len() {
            return Err("Invalid window size".into());
        }
    
        let weighted_multiplier = 2.0 / (self.window_size as f64 + 1.0);
        let first_slice = &price_data[0..self.window_size];
        let first_sma: f64 = first_slice.iter().sum::<f64>() / self.window_size as f64;
    
        // Insert the first SMA value with the correct timestamp
        self.values.insert(
            data[self.window_size - 1].close_time,  // Correct timestamp for the first day
            IndicatorResult {
                sentiment: IndicatorSentiment::Bullish,
                value: vec![first_sma],
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
                data[i].close_time,  // Correct timestamp index for the last days
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
        
        // ema 2 test

        let mut ema_2 = EmaIndicator::new(2);
        ema_2.compute(&data_set).unwrap();

        assert!(ema_2.get(0).value[0] == 0.0);
        assert!(ema_2.get(1).value[0] == 5.5);
        assert!(ema_2.get(2).value[0] == 4.5);
        assert!(ema_2.get(3).value[0] == 2.8333333333333335);

        // ema 4 test

        let mut ema_4 = EmaIndicator::new(4);
        ema_4.compute(&data_set).unwrap();

        println!("{}", ema_4.get(3).value[0]);
        assert!(ema_4.get(3).value[0] == 4.24);
    }

    fn kline_data_from_close_price(close_prices: &[f64]) -> Vec<MarketKlineData> {
        close_prices
            .iter()
            .enumerate()
            .map(|(i, &close)| MarketKlineData {
                close,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                volume: 0.0,
                close_time: i as i64,
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
