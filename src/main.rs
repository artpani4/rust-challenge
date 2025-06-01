use anyhow::Result;
use env_logger;
use log::{debug, info};
use rust_challenge::config::GlobalConfig;
use rust_challenge::generator::{DefaultTransferGenerator, TransferGenerator};
use rust_challenge::stats::calculate_user_stats;

fn main() -> Result<()> {
    env_logger::init();

    let config = GlobalConfig::load()?;
    info!("Loaded config: {:?}", config);

    let generator = DefaultTransferGenerator {
        config: config.generator.clone(),
    };
    let transfers = generator.generate(10)?;
    debug!("Generated {} transfers", transfers.len());

    let stats = calculate_user_stats(&transfers);

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }

    Ok(())
}
