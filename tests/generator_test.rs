use anyhow::Result;
use rust_challenge::config::GlobalConfig;
use rust_challenge::generator::{DefaultTransferGenerator, TransferGenerator};

#[test]
fn generate_expected_count() -> Result<()> {
    let config = GlobalConfig::load();
    let generator = DefaultTransferGenerator {
        config: config?.generator,
    };
    let transfers = generator.generate(100).expect("wtf");
    assert_eq!(transfers.len(), 100);
    Ok(())
}

#[test]
fn generated_transfers_have_valid_data() -> Result<()> {
    let cfg = GlobalConfig::load();
    let generator = DefaultTransferGenerator {
        config: cfg?.generator,
    };
    let transfers = generator.generate(100).expect("wtf");
    for t in &transfers {
        assert_ne!(t.from, t.to, "should differ");
        assert!(t.amount >= generator.config.min_amount && t.amount <= generator.config.max_amount);
        assert!(
            t.usd_price >= generator.config.min_price && t.usd_price <= generator.config.max_price
        );
        assert!(t.ts > 0);
    }
    Ok(())
}
