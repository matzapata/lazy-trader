use crate::domain::config::{Config, TConfigRepository};

pub struct ConfigService<T>
where
    T: TConfigRepository,
{
    repo: T,
}

impl<T> ConfigService<T>
where
    T: TConfigRepository,
{
    pub fn new(repo: T) -> Self {
        ConfigService { repo }
    }

    pub async fn get(&self) -> Result<Config, Box<dyn std::error::Error>> {
        self.repo.get_config().await
    }

    pub async fn save(&self, cfg: &Config) -> Result<(), Box<dyn std::error::Error>> {
        self.repo.save_config(cfg).await
    }
}
