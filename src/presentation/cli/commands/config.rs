use async_trait::async_trait;
use clap::Args;
use console::style;
use lt::domain::config::Market;

use super::error::CliError;
use crate::cli::RunCommand;

#[derive(Args, Debug)]
pub struct ConfigCmd {
    subcommand: Option<String>,
    value: Option<String>,
}

#[async_trait]
impl RunCommand for ConfigCmd {
    async fn run(self) -> Result<(), CliError> {
        let config_repo = lt::infrastructure::domain::config::InmemoryConfigRepository::default();
        let config_service = lt::application::config::ConfigService::new(config_repo);

        let mut config = config_service.get().await?;

        match self.subcommand {
            Some(subcommand) => match subcommand.as_str() {
                "strategy.take_profit" => {
                    let take_profit = self.value.unwrap();
                    config.strategy.take_profit = take_profit.parse().unwrap();
                    config_service.save(&config).await?;
                    println!("{} {}", style("Updated take_profit").green(), take_profit);
                }
                "strategy.stop_loss" => {
                    let stop_loss = self.value.unwrap();
                    config.strategy.stop_loss = stop_loss.parse().unwrap();
                    config_service.save(&config).await?;
                    println!("{} {}", style("Updated stop loss").green(), stop_loss);
                }
                "markets.add" => {
                    let token = self.value.unwrap();
                    config.markets.push(Market {
                        id: token.clone(),
                        url: "".to_string(),
                    });
                    config_service.save(&config).await?;
                    println!("{} {}", style("Added market").green(), token);
                }
                "markets.remove" => {
                    let token = self.value.unwrap();
                    config.markets = config
                        .markets
                        .iter()
                        .filter(|f| f.id != token)
                        .cloned()
                        .collect();
                    config_service.save(&config).await?;
                    println!("{} {}", style("Removed market").red(), token);
                },
                "help" => println!("Available commands: strategy.take_profit, strategy.stop_loss, markets.add, markets.remove"),
                _ => println!("Available commands: strategy.take_profit, strategy.stop_loss, markets.add, markets.remove"),
            },
            None => println!("{:?}", &config),
        }

        Ok(())
    }
}
