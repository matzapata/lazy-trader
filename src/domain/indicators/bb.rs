use super::{
    indicator::{IndicatorResult, IndicatorSentiment, TIndicator},
    sma::sma,
};
use crate::domain::market::kline_data::MarketKlineData;
use std::collections::HashMap;

pub struct BoilingBandsIndicator {
    window_size: usize,
    multiplier: f64,
    slow_length: usize,
    values: HashMap<i64, IndicatorResult>,
}

impl BoilingBandsIndicator {
    pub fn new(window_size: usize, multiplier: f64, slow_length: usize) -> Self {
        BoilingBandsIndicator {
            window_size,
            multiplier,
            slow_length,
            values: HashMap::new(),
        }
    }
}

impl TIndicator for BoilingBandsIndicator {
    fn name(&self) -> &'static str {
        "BB"
    }

    fn info(&self) -> &'static str {
        "
  BB: Bollinger Bands

  Bullish Signals:
    - Price closes above the upper band → Strong bullish momentum, possible continuation.
    - Price crosses above the middle band (SMA) → Bullish reversal signal.
    - Bands expand while price stays near the upper band → Trend continuation with increasing volatility.

  Bearish Signals:
    - Price closes below the lower band → Strong bearish momentum, possible continuation.
    - Price crosses below the middle band (SMA) → Bearish reversal signal.
    - Bands expand while price stays near the lower band → Trend continuation with increasing volatility.
"
    }

    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>> {
        let price_data: Vec<f64> = data.iter().map(|f| f.close).collect();
        if self.slow_length > price_data.len() {
            return Err("Invalid window size".into());
        }

        let res = bb(&price_data, self.window_size, self.multiplier);

        for i in 0..res.len() {
            let timestamp = data[i + self.window_size - 1].close_time;
            let price = data[i + self.window_size - 1].close;
            self.values.insert(
                timestamp,
                IndicatorResult {
                    value: vec![res[i][0], res[i][1], res[i][2]],
                    sentiment: if price > res[i][2] {
                        IndicatorSentiment::Bullish
                    } else if price < res[i][0] {
                        IndicatorSentiment::Bearish
                    } else {
                        IndicatorSentiment::Neutral
                    },
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

pub fn bb(data_set: &Vec<f64>, window_size: usize, multiplier: f64) -> Vec<[f64; 3]> {
    //  [lower, middle, upper]
    let middle_bound = sma(data_set, window_size).unwrap();

    let mut res: Vec<[f64; 3]> = Vec::new();

    for i in 0..middle_bound.len() {
        let slice = &data_set[i..window_size + i];
        let variance = slice
            .iter()
            .map(|value| {
                let diff = middle_bound[i] - (*value as f64);
                diff * diff
            })
            .sum::<f64>()
            / window_size as f64;

        let standard_deviation = variance.sqrt();

        res.push([
            middle_bound[i] - multiplier * standard_deviation,
            middle_bound[i],
            middle_bound[i] + multiplier * standard_deviation,
        ]);
    }

    res
}

#[test]
fn test_bollinger_bands() {
    let data_set = vec![
        5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
    ];

    let result = bb(&data_set, 8, 2.0);
    let middle_bound = result.iter().map(|x| x[1]).collect::<Vec<f64>>();
    let upper_bound = result.iter().map(|x| x[2]).collect::<Vec<f64>>();
    let lower_bound = result.iter().map(|x| x[0]).collect::<Vec<f64>>();

    assert_eq!(6, middle_bound.len());
    assert_eq!(
        vec![3.0625, 2.875, 2.5625, 2.5625, 2.875, 3.3125],
        middle_bound
    );

    assert_eq!(6, upper_bound.len());
    assert_eq!(
        vec![
            6.395572906493346,
            5.906088913245535,
            4.589659342528357,
            4.589659342528357,
            5.206844763272204,
            5.758798223847616
        ],
        upper_bound
    );

    assert_eq!(6, lower_bound.len());
    assert_eq!(
        vec![
            -0.27057290649334576,
            -0.1560889132455352,
            0.535340657471643,
            0.535340657471643,
            0.5431552367277961,
            0.8662017761523844
        ],
        lower_bound
    );
}
