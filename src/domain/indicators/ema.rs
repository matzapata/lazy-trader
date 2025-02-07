use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;
use std::collections::HashMap;

pub struct EmaIndicator {
    fast_length: usize,
    slow_length: usize,
    values: HashMap<i64, IndicatorResult>,
}

impl EmaIndicator {
    pub fn new(fast_length: usize, slow_length: usize) -> Self {
        if fast_length > slow_length {
            panic!("Fast length should be less than slow length");
        }

        EmaIndicator {
            fast_length,
            slow_length,
            values: HashMap::new(),
        }
    }
}

impl TIndicator for EmaIndicator {
    fn name(&self) -> &'static str {
        "EMA"
    }

    fn info(&self) -> &'static str {
"
  EMA: Exponential Moving Average
  
  Bullish Signals:
    - Price crosses above the EMA → Uptrend signal.
    - Short-term EMA (e.g., 20 EMA) crosses above a longer-term EMA (e.g., 50 EMA) → Bullish momentum.
    - EMA is sloping upward and price remains above it → Strong bullish trend.

  Bearish Signals:
    - Price crosses below the EMA → Downtrend signal.
    - Short-term EMA crosses below a longer-term EMA → Bearish momentum.
    - EMA is sloping downward and price remains below it → Strong bearish trend.
"
    }

    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>> {
        let price_data: Vec<f64> = data.iter().map(|f| f.close).collect();
        if self.slow_length > price_data.len() {
            return Err("Invalid window size".into());
        }

        let slow_ema = ema(self.slow_length, &price_data);
        let fast_ema = ema(self.fast_length, &price_data);

        for i in 0..price_data.len() {
            let timestamp = data[i].close_time;
            let sentiment = if i < self.slow_length || fast_ema[i] == slow_ema[i] {
                IndicatorSentiment::Neutral
            } else if fast_ema[i] > slow_ema[i] {
                IndicatorSentiment::Bullish
            } else {
                IndicatorSentiment::Bearish
            };

            self.values.insert(
                timestamp,
                IndicatorResult {
                    value: vec![fast_ema[i], slow_ema[i]],
                    sentiment,
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

pub fn ema(window_size: usize, data_set: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    for _ in 0..(window_size - 1) {
        result.push(0.0);
    }

    let weighted_multiplier = 2.0 / (window_size as f64 + 1.0);
    let first_slice = &data_set[0..window_size];
    let first_sma: f64 = first_slice.iter().sum::<f64>() / window_size as f64;
    result.push(first_sma);
    for i in window_size..data_set.len() {
        let previous_ema = result[result.len() - 1];
        let ema: f64 =
            (data_set[i] * weighted_multiplier) + previous_ema * (1.0 - weighted_multiplier);
        result.push(ema);
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_moving_average() {
        let data_set = kline_data_from_close_price(&[5.0, 6.0, 4.0, 2.0]);

        // println!("{:?}", ema(2, &data_set.iter().map(|f| f.close).collect()));

        // ema 2 test

        let mut ema_2 = EmaIndicator::new(2, 4);
        ema_2.compute(&data_set).unwrap();

        for i in 0..data_set.len() {
            println!("{:?}", ema_2.get(data_set[i].close_time));
        }

        // assert!(ema_2.get(0).value[0] == 0.0);
        // assert!(ema_2.get(1).value[0] == 5.5);
        // assert!(ema_2.get(2).value[0] == 4.5);
        // assert!(ema_2.get(3).value[0] == 2.8333333333333335);

        // // ema 4 test

        // let mut ema_4 = EmaIndicator::new(4);
        // ema_4.compute(&data_set).unwrap();

        // println!("{}", ema_4.get(3).value[0]);
        // assert!(ema_4.get(3).value[0] == 4.24);
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


