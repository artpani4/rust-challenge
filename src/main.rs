mod generator;
mod model;
mod pipeline;
// mod storage;
use anyhow::Result;
use generator::generate_transfers;
use pipeline::calculate_user_stats;

fn main() -> Result<()> {
    let transfers = generate_transfers(10_000)?;

    let stats = calculate_user_stats(&transfers);

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }
    Ok(())
}
