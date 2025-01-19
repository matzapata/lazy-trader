use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;

pub struct RsiIndicator {
    window_size: usize,
}

impl RsiIndicator {
    pub fn new(window_size: usize) -> Self {
        RsiIndicator { window_size }
    }
}

impl TIndicator for RsiIndicator {
    fn info(&self) -> &'static str {
        "RSI: Relative Strength Index\n\
        Measures the speed and change of price movements.\n\
        Typically oscillates between 0 and 100.\n\
        Used to identify overbought or oversold conditions."
    }

    fn compute(&self, data: &Vec<MarketKlineData>) -> Vec<IndicatorResult> {
        let price_data: Vec<f64> = data.iter().rev().take(100).map(|f| f.close).collect();
        let mut result: Vec<IndicatorResult> = Vec::new();

        let mut previous_average_gain;
        let mut previous_average_loss;

        // RSI Step one
        let mut gains_sum = 0.0;
        let mut loss_sum = 0.0;
        for i in 0..(self.window_size + 1) {
            let gain = if i == 0 {
                0.0
            } else {
                (100.0 / price_data[i - 1]) * price_data[i] - 100.0
            };

            if gain >= 0.0 {
                gains_sum += gain;
            } else {
                loss_sum += gain.abs();
            }
        }
        let current_average_gain = gains_sum / self.window_size as f64;
        let current_average_loss = loss_sum / self.window_size as f64;

        let rsi_a = 100.0 - 100.0 / (1.0 + (current_average_gain / current_average_loss));
        previous_average_gain = current_average_gain;
        previous_average_loss = current_average_loss;

        result.push(to_indicator_result(rsi_a));

        // RSI Step two
        for i in (self.window_size + 1)..price_data.len() {
            let gain = (100.0 / price_data[i - 1]) * price_data[i] - 100.0;
            let (current_gain, current_loss) = if gain > 0.0 {
                (gain, 0.0)
            } else {
                (0.0, gain.abs())
            };

            let current_average_gain = (previous_average_gain * (self.window_size as f64 - 1.0)
                + current_gain)
                / self.window_size as f64;
            let current_average_loss = (previous_average_loss * (self.window_size as f64 - 1.0)
                + current_loss)
                / self.window_size as f64;

            previous_average_gain = current_average_gain;
            previous_average_loss = current_average_loss;

            let rsi = 100.0 - 100.0 / (1.0 + current_average_gain / current_average_loss);
            result.push(to_indicator_result(rsi));
        }

        result
    }
}

fn to_indicator_result(data: f64) -> IndicatorResult {
    IndicatorResult {
        value: vec![data],
        sentiment: if data > 70.0 {
            IndicatorSentiment::Bullish
        } else if data < 30.0 {
            IndicatorSentiment::Bearish
        } else {
            IndicatorSentiment::Neutral
        },
    }
}

// #[test]
// fn test_relative_strength_index() {
//     let price_data = vec![
//         5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
//     ];

//     let result = relative_strength_index(&price_data, 14);
//     assert_eq!(None, result);

//     let result = relative_strength_index(&price_data, 8).unwrap();

//     assert_eq!(5, result.len());
//     assert_eq!(
//         vec![
//             56.852791878172596,
//             56.852791878172596,
//             59.17295654731064,
//             61.256328819550575,
//             63.16578540011347
//         ],
//         result
//     );

//     let price_data = vec![
//         44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
//         45.61, 46.28, 46.28, 46.00, 46.03,
//     ];

//     let result = relative_strength_index(&price_data, 14).unwrap();

//     assert_eq!(3, result.len());
//     assert_eq!(
//         vec![70.53539393736207, 66.436571546019, 66.66146763681454],
//         result
//     );
// }
