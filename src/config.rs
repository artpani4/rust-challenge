use anyhow::{Context, Result};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub generator: GeneratorConfig,
    pub clickhouse: ClickhouseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
    pub address_pool_amount: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClickhouseConfig {
    pub url: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl GlobalConfig {
    pub fn load() -> Result<Self> {
        info!("Loading configuration");

        let base = Figment::new().merge(Toml::file("config.toml"));

        let clickhouse: ClickhouseConfig = Figment::new()
            .merge(Env::prefixed("CLICKHOUSE_"))
            .extract()
            .context("Failed to load clickhouse config")?;

        let mut config: GlobalConfig = base.extract().context("Failed to load config.toml")?;

        config.clickhouse = clickhouse;

        Ok(config)
    }
}
