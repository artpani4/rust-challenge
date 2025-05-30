use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub generator: GeneratorConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            min_amount: 1.0,
            max_amount: 1000.0,
            min_price: 0.1,
            max_price: 2.0,
            max_age_secs: 86_400 * 30,
        }
    }
}

impl GlobalConfig {
    pub fn load() -> Self {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("Failed to load config!!")
    }
}
