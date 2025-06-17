use anyhow::Result;
use env_logger;
use log::info;
use rust_challenge::config::GlobalConfig;
use rust_challenge::generator::{DefaultTransferGenerator, TransferGenerator};
use rust_challenge::storage::clickhouse::ClickhouseStorage;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    dotenvy::dotenv().ok();
    let config = GlobalConfig::load()?;
    println!("{:?}", &config.clickhouse);
    let generator = DefaultTransferGenerator {
        config: config.generator.clone(),
    };

    let transfers = generator.generate(10)?;
    let storage = ClickhouseStorage::new(&config.clickhouse);
    storage.create_table().await?;
    storage.insert_transfers(transfers).await?;

    Ok(())
}
