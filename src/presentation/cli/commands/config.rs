use async_trait::async_trait;
use clap::{Args, Subcommand};
use console::style;
use lt::domain::config::{Config, Market};

use super::error::CliError;
use crate::cli::RunCommand;

#[derive(Args, Debug)]
pub struct ConfigCmd {
    #[command(subcommand)]
    pub action: Option<ConfigAction>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set a configuration value
    Set {
        /// Key of the configuration (e.g., market.limit)
        key: String,
        /// Value to set for the key
        value: String,
    },
    Add {
        /// Key of the configuration (e.g., market.limit)
        key: String,
        /// Value to set for the key
        value: String,
    },
    /// Delete a configuration value
    Remove {
        /// Key of the configuration (e.g., market id)
        key: String,
        /// Value to delete
        value: String,
    },
}

#[async_trait]
impl RunCommand for ConfigCmd {
    async fn run(self) -> Result<(), CliError> {
        let config_repo = lt::infrastructure::domain::config::InmemoryConfigRepository::default();
        let config_service = lt::application::config::ConfigService::new(config_repo);

        let mut config = config_service.get().await?;

        match self.action {
            Some(action) => match action {
                ConfigAction::Set { key, value } => match key.as_str() {
                    "strategy.take_profit" => {
                        config.strategy.take_profit = value.parse().unwrap();
                        config_service.save(&config).await?;
                        println!("{} {}", style("Updated take_profit").green(), value);
                    }
                    "strategy.stop_loss" => {
                        config.strategy.stop_loss = value.parse().unwrap();
                        config_service.save(&config).await?;
                        println!("{} {}", style("Updated stop loss").green(), value);
                    }
                    _ => print_help(&config),
                },
                ConfigAction::Add { key, value } => match key.as_str() {
                    "markets" => {
                        config.markets.push(Market { id: value.clone() });
                        config_service.save(&config).await?;
                        println!("{} {}", style("Added market").green(), value);
                    }
                    _ => print_help(&config),
                },
                ConfigAction::Remove { key, value } => match key.as_str() {
                    "markets" => {
                        config.markets = config
                            .markets
                            .iter()
                            .filter(|f| f.id != value)
                            .cloned()
                            .collect();
                        config_service.save(&config).await?;
                        println!("{} {}", style("Removed market").red(), value);
                    }
                    _ => print_help(&config),
                },
            },
            None => print_help(&config),
        }

        Ok(())
    }
}

fn print_help(cfg: &Config) {
    println!("{}: \n\n{:#?}\n\n", style("Current config").bold(), cfg);
    println!("Available commands:");
    println!(
        " - {}",
        style("config set <key: strategy.take_profit, strategy.stop_loss> <value>").blue()
    );
    println!(" - {}", style("config add <key: markets> <value>").blue());
    println!(
        " - {}",
        style("config remove <key: markets> <value>").blue()
    );
}
