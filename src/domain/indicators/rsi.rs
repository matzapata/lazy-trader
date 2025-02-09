use std::collections::HashMap;

use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;

pub struct RsiIndicator {
    window_size: usize,
    values: HashMap<i64, IndicatorResult>,
}

impl RsiIndicator {
    pub fn new(window_size: usize) -> Self {
        RsiIndicator {
            window_size,
            values: HashMap::new(),
        }
    }
}

trait TAsRsiIndicator {
    fn as_rsi_indicator(&self) -> IndicatorResult;
}

impl TAsRsiIndicator for f64 {
    fn as_rsi_indicator(&self) -> IndicatorResult {
        IndicatorResult {
            value: vec![*self],
            sentiment: if *self > 70.0 {
                IndicatorSentiment::Bearish
            } else if *self < 30.0 {
                IndicatorSentiment::Bullish
            } else {
                IndicatorSentiment::Neutral
            },
        }
    }
}

impl TIndicator for RsiIndicator {
    fn name(&self) -> &'static str {
        "RSI"
    }

    fn info(&self) -> &'static str {
        "
  RSI: Relative Strength Index
  
  Bullish Signals:
    - RSI crosses above 30 → Oversold bounce, potential bullish reversal.
    - RSI stays above 50 → Indicates bullish momentum.
    - RSI crosses above 70 during an uptrend → Strong bullish continuation (but watch for overbought conditions).

  Bearish Signals:
    - RSI crosses below 70 → Overbought, potential bearish reversal.
    - RSI stays below 50 → Indicates bearish momentum.
    - RSI crosses below 30 during a downtrend → Strong bearish continuation (but watch for oversold conditions).
"
    }

    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>> {
        if self.window_size > data.len() {
            return Err(format!("Invalid window size: {}", self.window_size).into());
        }

        let price_data = data.iter().map(|f| f.close).collect::<Vec<f64>>();
        let res = rsi(&price_data, self.window_size);

        // fill with 0
        for i in 0..self.window_size {
            let timestamp = data.iter().nth(i).unwrap().close_time;
            self.values.insert(
                timestamp,
                IndicatorResult {
                    value: vec![0.0],
                    sentiment: IndicatorSentiment::Neutral,
                },
            );
        }

        // fill with res
        for i in self.window_size..res.len() {
            let timestamp = data.iter().nth(i + self.window_size).unwrap().close_time;
            self.values.insert(timestamp, res[i].as_rsi_indicator());
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

fn rsi(data_set: &Vec<f64>, window_size: usize) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    let mut previous_average_gain;
    let mut previous_average_loss;

    // RSI Step one
    let mut gains_sum = 0.0;
    let mut loss_sum = 0.0;
    for i in 0..(window_size + 1) {
        let gain = if i == 0 {
            0.0
        } else {
            (100.0 / data_set[i - 1]) * data_set[i] - 100.0
        };
        if gain >= 0.0 {
            gains_sum += gain;
        } else {
            loss_sum += gain.abs();
        }
    }
    let current_average_gain = gains_sum / window_size as f64;
    let current_average_loss = loss_sum / window_size as f64;
    let rsi_a = 100.0 - 100.0 / (1.0 + (current_average_gain / current_average_loss));

    previous_average_gain = current_average_gain;
    previous_average_loss = current_average_loss;
    result.push(rsi_a);

    // RSI Step two
    for i in (window_size + 1)..data_set.len() {
        let gain = (100.0 / data_set[i - 1]) * data_set[i] - 100.0;
        let (current_gain, current_loss) = if gain > 0.0 {
            (gain, 0.0)
        } else {
            (0.0, gain.abs())
        };
        let current_average_gain = (previous_average_gain * (window_size as f64 - 1.0)
            + current_gain)
            / window_size as f64;
        let current_average_loss = (previous_average_loss * (window_size as f64 - 1.0)
            + current_loss)
            / window_size as f64;
        previous_average_gain = current_average_gain;
        previous_average_loss = current_average_loss;
        let rsi = 100.0 - 100.0 / (1.0 + current_average_gain / current_average_loss);
        result.push(rsi);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_strength_index() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = rsi(&data_set, 8);

        assert_eq!(5, result.len());
        assert_eq!(
            vec![
                56.852791878172596,
                56.852791878172596,
                59.17295654731064,
                61.256328819550575,
                63.16578540011347
            ],
            result
        );

        let data_set = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03,
        ];

        let result = rsi(&data_set, 14);

        assert_eq!(3, result.len());
        assert_eq!(
            vec![70.53539393736207, 66.436571546019, 66.66146763681454],
            result
        );
    }
}
