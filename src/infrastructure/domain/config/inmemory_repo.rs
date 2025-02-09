use crate::domain::config::{Config, TConfigRepository};
use std::path::PathBuf;

pub struct InmemoryConfigRepository {
    root: PathBuf,
}

impl InmemoryConfigRepository {
    pub fn new<T>(root: T) -> InmemoryConfigRepository
    where
        T: Into<PathBuf>,
    {
        InmemoryConfigRepository { root: root.into() }
    }
}

impl Default for InmemoryConfigRepository {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap();
        InmemoryConfigRepository {
            root: home_dir.join(".lt"),
        }
    }
}

#[async_trait::async_trait]
impl TConfigRepository for InmemoryConfigRepository {
    async fn get_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
        if self.root.exists() {
            let file_contents = std::fs::read_to_string(&self.root)?;
            return Ok(serde_json::from_str(&file_contents)?);
        }

        Ok(Config::default())
    }

    async fn save_config(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(&self.root, serde_json::to_string(config)?)?;
        Ok(())
    }
}
