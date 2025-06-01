use anyhow::Result;
use rust_challenge::config::GlobalConfig;
use rust_challenge::generator::{DefaultTransferGenerator, TransferGenerator};
use rust_challenge::stats::calculate_user_stats;

fn main() -> Result<()> {
    let config = GlobalConfig::load();

    let generator = DefaultTransferGenerator {
        config: config.generator.clone(),
    };
    let transfers = generator.generate(10)?;
    // for t in transfers.iter() {
    //     println!("{:?}", t);
    // }
    let stats = calculate_user_stats(&transfers);

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }

    Ok(())
}
