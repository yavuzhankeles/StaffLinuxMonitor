use crate::config::ApiConfig;
use crate::SystemInfo;
use anyhow::Result;
use reqwest::blocking::Client;
use std::time::Duration;

pub struct ApiClient {
    client: Client,
    config: ApiConfig,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()?;

        Ok(Self { client, config })
    }

    pub fn send_system_info(&self, info: &SystemInfo) -> Result<()> {
        let url = format!("{}/api/v1/system-info", self.config.base_url);
        
        let response = self.client
            .post(&url)
            .header("X-API-Key", &self.config.api_key)
            .json(info)
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("API request failed: {}", response.status());
        }

        Ok(())
    }

    pub fn get_system_info(&self) -> Result<SystemInfo> {
        let url = format!("{}/api/v1/system-info", self.config.base_url);
        
        let response = self.client
            .get(&url)
            .header("X-API-Key", &self.config.api_key)
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("API request failed: {}", response.status());
        }

        let info: SystemInfo = response.json()?;
        Ok(info)
    }
} 