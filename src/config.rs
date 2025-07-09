use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout_seconds: u64,
    pub retry_count: u32,
    pub rate_limit: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            api_key: "".to_string(),
            timeout_seconds: 30,
            retry_count: 3,
            rate_limit: 100,
        }
    }
}

impl ApiConfig {
    pub fn load() -> Result<Self> {
        // Önce .env dosyasını yükle
        dotenv::dotenv().ok();

        // Yapılandırma dosyasını yükle
        let config_path = Path::new("config.toml");
        let config = if config_path.exists() {
            let settings = config::Config::builder()
                .add_source(config::File::from(config_path))
                .add_source(config::Environment::with_prefix("STAFFMON"))
                .build()?;

            settings.try_deserialize()?
        } else {
            Self::default()
        };

        Ok(config)
    }
} 