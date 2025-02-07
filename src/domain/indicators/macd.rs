use super::{
    ema::ema,
    indicator::{IndicatorResult, IndicatorSentiment, TIndicator},
};
use crate::domain::market::kline_data::MarketKlineData;
use std::collections::HashMap;

pub struct MacdIndicator {
    fast_length: usize,
    slow_length: usize,
    signal_length: usize,
    values: HashMap<i64, IndicatorResult>,
}

impl MacdIndicator {
    pub fn new(fast_length: usize, slow_length: usize, signal_length: usize) -> Self {
        MacdIndicator {
            fast_length,
            slow_length,
            signal_length,
            values: HashMap::new(),
        }
    }
}

impl TIndicator for MacdIndicator {
    fn name(&self) -> &'static str {
        "MACD"
    }

    fn info(&self) -> &'static str {
        "
  MACD: Moving Average Convergence/Divergence

  Bullish Signals:        
    - MACD crosses above the Signal Line → Buy signal (momentum is increasing).
    - MACD and Signal Line both above the zero line → Strong bullish trend.
    - Rising Histogram → Bullish momentum is strengthening.
  Bearish Signals:
    - MACD crosses below the Signal Line → Sell signal (momentum is decreasing).
    - MACD and Signal Line both below the zero line → Strong bearish trend.
    - Falling Histogram → Bearish momentum is strengthening.
        "
    }

    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>> {
        let price_data: Vec<f64> = data.iter().map(|f| f.close).collect();
        if self.slow_length > price_data.len() {
            return Err("Invalid window size".into());
        }

        let (macd, signal) = moving_average_convergence_divergence(
            &price_data,
            self.fast_length,
            self.slow_length,
            self.signal_length,
        );

        let mut macd_above = macd[0] > signal[0];
        for i in 0..price_data.len() {
            let timestamp = data[i].close_time;

            let sentiment = if macd[i] < signal[i] && macd_above {
                // we're going down
                macd_above = false;
                IndicatorSentiment::Bearish
            } else if macd[i] > signal[i] && !macd_above {
                // we're going up
                macd_above = true;
                IndicatorSentiment::Bullish
            } else {
                IndicatorSentiment::Neutral
            };

            self.values.insert(
                timestamp,
                IndicatorResult {
                    value: vec![macd[i], signal[i]],
                    sentiment: sentiment,
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

pub fn moving_average_convergence_divergence(
    data_set: &Vec<f64>,
    fast_length: usize,
    slow_length: usize,
    signal_length: usize,
) -> (Vec<f64>, Vec<f64>) {
    let fast_ema = ema(fast_length, data_set);
    let slow_ema = ema(slow_length, data_set);

    let mut macd: Vec<f64> = Vec::new();
    for i in 0..slow_ema.len() {
        let macd_val = fast_ema[(fast_ema.len() - slow_ema.len()) + i] - slow_ema[i];
        macd.push(macd_val);
    }

    let signal = ema(signal_length, &macd);

    (macd, signal)
}

// #[test]
// fn test_moving_average_convergence_divergence() {
//     let data_set = vec![
//         5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
//     ];

//     let result = moving_average_convergence_divergence(&data_set, 12, 26, 9);
//     assert_eq!(None, result);

//     let result = moving_average_convergence_divergence(&data_set, 3, 6, 2).unwrap();
//     assert_eq!(8, result.macd.len());
//     assert_eq!(
//         vec![
//             -1.5,
//             -1.0178571428571432,
//             -0.48596938775510257,
//             -0.1194424198250732,
//             0.02852327155351908,
//             0.18443626539537084,
//             0.32091429671097904,
//             0.4309544083649852
//         ],
//         result.macd
//     );
//     assert_eq!(7, result.signal.len());
//     assert_eq!(
//         vec![
//             -1.2589285714285716,
//             -0.7436224489795923,
//             -0.32750242954324627,
//             -0.09015196214540272,
//             0.09290685621511298,
//             0.24491181654569036,
//             0.36894021109188696
//         ],
//         result.signal
//     );
// }
