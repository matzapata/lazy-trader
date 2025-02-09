use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub markets: Vec<Market>,
    pub strategy: Strategy
}

impl Default for Config {
    fn default() -> Self {
        Config {
            markets: vec![
                Market { id: "BTCUSDT".to_string() },
                Market { id: "ETHUSDT".to_string() },
                Market { id: "SOLUSDT".to_string() },
            ],
            strategy: Strategy {
                stop_loss: 0.97,
                take_profit: 1.05,
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Market {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    pub stop_loss: f64,
    pub take_profit: f64,
}

#[async_trait::async_trait]
pub trait TConfigRepository {
    async fn get_config(&self) -> Result<Config, Box<dyn std::error::Error>>;
    async fn save_config(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>>;
}