use super::error::CliError;
use crate::{cli::RunCommand, console::print_divider};
use async_trait::async_trait;
use clap::Args;

#[derive(Args, Debug)]
pub struct IndicatorsCmd {}

#[async_trait]
impl RunCommand for IndicatorsCmd {
    async fn run(self) -> Result<(), CliError> {
        let indicators: Vec<Box<dyn lt::domain::indicators::indicator::TIndicator>> = vec![
            Box::new(lt::domain::indicators::rsi::RsiIndicator::new(14)),
            Box::new(lt::domain::indicators::ema::EmaIndicator::new(20, 50)),
            Box::new(lt::domain::indicators::macd::MacdIndicator::new(12, 26, 9)),
            Box::new(lt::domain::indicators::bb::BoilingBandsIndicator::new(
                20, 2.0, 50,
            )),
        ];

        print_divider();
        for indicator in &indicators {
            println!("{}", indicator.info());
            print_divider();
        }

        Ok(())
    }
}

