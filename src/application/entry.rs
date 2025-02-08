// calculate stop loss, take profit, and entry, expected profit, expected loss, etc

use crate::domain::{config::TConfigRepository, market::{kline_data::TMarketKlineDataRepo, market::Market}};

pub struct EntryService<R, C>
where
    C: TConfigRepository,
    R: TMarketKlineDataRepo,
{
    config_repo: C,
    market_data_repo: R,
}

pub struct  Entry {
    pub stop_loss: f64,
    pub take_profit: f64,
    pub expected_profit: f64,
    pub expected_loss: f64,
    pub entry_price: f64
}

impl<R, C> EntryService<R, C>
where
    C: TConfigRepository,
    R: TMarketKlineDataRepo,
{
    pub fn new(market_data_repo: R, config_repo: C) -> Self {
        EntryService { market_data_repo, config_repo }
    }

    pub async fn compute_entry(&self, market: &Market, amount: f64) -> Entry {
        let price = self.market_data_repo.get_price(market).await.unwrap();
        let config = self.config_repo.get_config().await.unwrap();

        // stop loss
        let stop_loss = config.strategy.stop_loss * price;

        // take profit
        let take_profit = config.strategy.risk * price;

        // expected profit
        let expected_profit = (take_profit - price) * amount;

        // expected loss
        let expected_loss = (price - stop_loss) * amount;

        Entry {
            stop_loss,
            take_profit,
            expected_profit,
            expected_loss,
            entry_price: price,
        }
    }
}
