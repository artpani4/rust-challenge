use rand::{distributions::Alphanumeric, Rng};

pub fn rand_address(rng: &mut impl Rng) -> String {
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("0x{}", suffix)
}

pub fn generate_address_pool(rng: &mut impl Rng, size: usize) -> Vec<String> {
    (0..size).map(|_| rand_address(rng)).collect()
}
