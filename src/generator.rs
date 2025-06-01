use crate::address::generate_address_pool;
use crate::config::GeneratorConfig;
use crate::model::Transfer;
use anyhow::Context;
use rand::{seq::SliceRandom, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub trait TransferGenerator {
    fn generate(&self, count: usize) -> anyhow::Result<Vec<Transfer>>;
}

pub struct DefaultTransferGenerator {
    pub config: GeneratorConfig,
}

impl TransferGenerator for DefaultTransferGenerator {
    fn generate(&self, count: usize) -> anyhow::Result<Vec<Transfer>> {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("sth wrong with system time (< UNIX_EPOCH)")?
            .as_secs();

        let address_pool = generate_address_pool(&mut rng, self.config.address_pool_amount);

        let mut transfers = Vec::with_capacity(count);
        while transfers.len() < count {
            let from = address_pool
                .choose(&mut rng)
                .cloned()
                .context("No address found for 'from'")?;
            let to = address_pool
                .choose(&mut rng)
                .cloned()
                .context("No address found for 'to'")?;
            if from == to {
                continue;
            }
            let amount = rng.gen_range(self.config.min_amount..self.config.max_amount);
            let usd_price = rng.gen_range(self.config.min_price..self.config.max_price);
            let ts = now - rng.gen_range(0..self.config.max_age_secs);
            transfers.push(Transfer {
                id: Uuid::new_v4(),
                ts,
                from,
                to,
                amount,
                usd_price,
            });
        }

        Ok(transfers)
    }
}
