use super::indicator::{IndicatorResult, IndicatorSentiment, TIndicator};
use crate::domain::market::kline_data::MarketKlineData;

pub struct SmaIndicator {
    window_size: usize,
}

impl SmaIndicator {
    pub fn new(window_size: usize) -> Self {
        SmaIndicator { window_size }
    }
}

impl TIndicator for SmaIndicator {
    fn name(&self) -> &'static str {
        "SMA"
    }

    fn info(&self) -> &'static str {
        "SMA: Simple Moving Average"
    }

    fn compute(&mut self, data: &Vec<MarketKlineData>) -> Result<(), Box<dyn std::error::Error>> {
        // let close_price: Vec<f64> = data.iter().rev().map(|f| f.close).collect();

        // if self.window_size > close_price.len() {
        //     return None;
        // }

        // let mut window_start = 0;
        // let mut result: Vec<IndicatorResult> = Vec::new();
        // while window_start + self.window_size <= close_price.len() {
        //     let window_end = window_start + self.window_size;
        //     let data_slice = &close_price[window_start..window_end];
        //     let sum: f64 = data_slice.iter().sum();
        //     let average = sum / self.window_size as f64;

        //     let timestamp = data.iter().rev().nth(window_start + self.window_size - 1).unwrap().close_time;

        //     result.push(IndicatorResult {
        //         sentiment: IndicatorSentiment::Neutral,
        //         value: vec![average],
        //         timestamp
        //     });

        //     window_start += 1;
        // }

        // Some(result)

        Ok(())
    }

    fn get(&self, timestamp: i64) -> IndicatorResult {
        IndicatorResult {
            sentiment: IndicatorSentiment::Neutral,
            value: vec![0.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_moving_average() {
        let data_set = kline_data_from_close_price(&[5.0, 6.0, 4.0, 2.0]);

        // // test with window size 2
        // let sma2 = SmaIndicator::new(2);
        // let result = sma2.compute(&data_set).unwrap();
        // assert_eq!(3, result.len());
        // // assert_eq!(vec![5.5, 5.0, 3.0], result);

        // // test with window size 3
        // let sma3 = SmaIndicator::new(3);
        // let result = sma3.compute(&data_set).unwrap();
        // assert_eq!(2, result.len());
        // // assert_eq!(vec![5.0, 4.0], result);

        // // test with window size 4
        // let sma4 = SmaIndicator::new(4);
        // let result = sma4.compute(&data_set).unwrap();
        // assert_eq!(1, result.len());
        // // assert_eq!(vec![4.25], result);

        // // test with window size bigger than data size, should return None
        // let sma5 = SmaIndicator::new(5);
        // let result = sma5.compute(&data_set);
        // assert_eq!(None, result);
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
