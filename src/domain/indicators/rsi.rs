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

trait TAsRsiIndicator {
    fn as_rsi_indicator(&self) -> IndicatorResult;
}

impl TAsRsiIndicator for f64 {
    fn as_rsi_indicator(&self) -> IndicatorResult {
        IndicatorResult {
            value: vec![*self],
            sentiment: if *self > 70.0 {
                IndicatorSentiment::Bullish
            } else if *self < 30.0 {
                IndicatorSentiment::Bearish
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
        "RSI: Relative Strength Index\n\
        Measures the speed and change of price movements.\n\
        Typically oscillates between 0 and 100.\n\
        Used to identify overbought or oversold conditions."
    }

    fn compute(&self, data: &Vec<MarketKlineData>) -> Option<Vec<IndicatorResult>> {
        let mut rsi: Vec<f64> = Vec::new();
        let price_data = data.iter().rev().map(|f| f.close).collect::<Vec<f64>>();

        if self.window_size > price_data.len() {
            return None;
        }

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

        rsi.push(rsi_a);

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

            rsi.push(100.0 - 100.0 / (1.0 + current_average_gain / current_average_loss));
        }

        Some(rsi.iter().map(|f| f.as_rsi_indicator()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_strength_index() {
        let rsi = RsiIndicator::new(8);
        let data = kline_data_from_close_price(&[
            5.0, 4.5, 4.0, 3.5, 3.5, 3.0, 2.0, 1.0, 1.5, 2.0, 4.0, 6.0, 5.0,
        ]);

        let result = rsi.compute(&data).unwrap();

        assert_eq!(5, result.len());
        assert_eq!(
            vec![
                56.852791878172596.as_rsi_indicator(),
                56.852791878172596.as_rsi_indicator(),
                59.17295654731064.as_rsi_indicator(),
                61.256328819550575.as_rsi_indicator(),
                63.16578540011347.as_rsi_indicator()
            ],
            result
        );
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
