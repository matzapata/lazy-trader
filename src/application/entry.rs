// calculate stop loss, take profit, and entry, expected profit, expected loss, etc

use crate::domain::{
    config::TConfigRepository,
    market::{kline_data::TMarketKlineDataRepo, market::Market},
};

pub struct EntryService<R, C>
where
    C: TConfigRepository,
    R: TMarketKlineDataRepo,
{
    config_repo: C,
    market_data_repo: R,
}

pub struct Entry {
    pub stop_loss: f64,
    pub take_profit: f64,
    pub expected_profit: f64,
    pub potential_loss: f64,
    pub entry_price: f64,
}

impl<R, C> EntryService<R, C>
where
    C: TConfigRepository,
    R: TMarketKlineDataRepo,
{
    pub fn new(market_data_repo: R, config_repo: C) -> Self {
        EntryService {
            market_data_repo,
            config_repo,
        }
    }

    pub async fn compute_entry(
        &self,
        market: &Market,
        amount: Option<f64>,
        stop_loss: Option<f64>,
        take_profit: Option<f64>,
    ) -> Entry {
        let price = self.market_data_repo.get_price(market).await.unwrap();
        let config = self.config_repo.get_config().await.unwrap();

        let stop_loss_price = stop_loss.unwrap_or(config.strategy.stop_loss) * price;
        let take_profit_price = take_profit.unwrap_or(config.strategy.take_profit) * price;

        let expected_profit = (take_profit_price - price) * amount.unwrap_or(1.0);
        let potential_loss = (price - stop_loss_price) * amount.unwrap_or(1.0);

        Entry {
            stop_loss: stop_loss_price,
            take_profit: take_profit_price,
            expected_profit,
            potential_loss,
            entry_price: price,
        }
    }
}
