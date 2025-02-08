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
                "strategy.risk" => {
                    let risk = self.value.unwrap();
                    config.strategy.risk = risk.parse().unwrap();
                    config_service.save(&config).await?;
                    println!("{} {}", style("Updated risk").green(), risk);
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
                _ => println!("unknown command"),
            },
            None => println!("{:?}", &config),
        }

        Ok(())
    }
}
