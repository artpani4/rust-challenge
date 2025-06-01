use log::debug;
use rand::{distributions::Alphanumeric, Rng};

pub fn rand_address(rng: &mut impl Rng) -> String {
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let address = format!("0x{}", suffix);
    debug!("Generated random address: {}", address);
    address
}

pub fn generate_address_pool(rng: &mut impl Rng, size: usize) -> Vec<String> {
    let pool = (0..size).map(|_| rand_address(rng)).collect();
    debug!("Generated address pool with {} addresses", size);
    pool
}
