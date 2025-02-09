use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;
use std::collections::HashMap;

pub struct SmaIndicator {
    fast_length: usize,
    slow_length: usize,
    values: HashMap<i64, IndicatorResult>,
}

impl SmaIndicator {
    pub fn new(fast_length: usize, slow_length: usize) -> Self {
        if fast_length > slow_length {
            panic!("Fast length should be less than slow length");
        }

        SmaIndicator {
            fast_length,
            slow_length,
            values: HashMap::new(),
        }
    }
}

impl TIndicator for SmaIndicator {
    fn name(&self) -> &'static str {
        "SMA"
    }

    fn info(&self) -> &'static str {
        "
  SMA: Simple Moving Average
  
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

        let slow_ema = sma(&price_data, self.slow_length).unwrap();
        let fast_ema = sma(&price_data, self.fast_length).unwrap();

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

pub fn sma(data: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data.len() {
        return None;
    }

    let mut window_start = 0;
    let mut result: Vec<f64> = Vec::new();
    while window_start + window_size <= data.len() {
        let window_end = window_start + window_size;
        let data_slice = &data[window_start..window_end];
        let sum: f64 = data_slice.iter().sum();
        let average = sum / window_size as f64;

        result.push(average);

        window_start += 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_average() {
        let data_set = vec![5.0, 6.0, 4.0, 2.0];

        // test with window size 2
        let result = sma(&data_set, 2).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(vec![5.5, 5.0, 3.0], result);

        // test with window size 3
        let result = sma(&data_set, 3).unwrap();
        assert_eq!(2, result.len());
        assert_eq!(vec![5.0, 4.0], result);

        // test with window size 4
        let result = sma(&data_set, 4).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(vec![4.25], result);

        // test with window size bigger than data size, should return None
        let result = sma(&data_set, 5);
        assert_eq!(None, result);
    }
}
