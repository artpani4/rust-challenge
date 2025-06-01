use anyhow::{Context, Result};
use rust_challenge::model::Transfer;
use rust_challenge::stats::calculate_user_stats;
use uuid::Uuid;

#[test]
fn calculates_correct_max_balance() -> Result<()> {
    let transfers = vec![
        Transfer {
            id: Uuid::new_v4(),
            ts: 100,
            from: "A".to_string(),
            to: "B".to_string(),
            amount: 10.0,
            usd_price: 1.0,
        },
        Transfer {
            id: Uuid::new_v4(),
            ts: 200,
            from: "B".to_string(),
            to: "C".to_string(),
            amount: 5.0,
            usd_price: 2.0,
        },
        Transfer {
            id: Uuid::new_v4(),
            ts: 300,
            from: "A".to_string(),
            to: "B".to_string(),
            amount: 20.0,
            usd_price: 1.5,
        },
    ];

    let stats = calculate_user_stats(&transfers);

    let b_stats = stats
        .iter()
        .find(|s| s.address == "B")
        .context("Stats for address B not found")?;
    assert_eq!(b_stats.max_balance, 25.0);
    Ok(())
}

#[test]
fn calculates_avg_prices_and_volume() {
    let transfers = vec![
        Transfer {
            id: Uuid::new_v4(),
            ts: 100,
            from: "X".to_string(),
            to: "Y".to_string(),
            amount: 5.0,
            usd_price: 1.0,
        },
        Transfer {
            id: Uuid::new_v4(),
            ts: 200,
            from: "Z".to_string(),
            to: "Y".to_string(),
            amount: 15.0,
            usd_price: 2.0,
        },
        Transfer {
            id: Uuid::new_v4(),
            ts: 300,
            from: "Y".to_string(),
            to: "X".to_string(),
            amount: 10.0,
            usd_price: 3.0,
        },
    ];

    let stats = calculate_user_stats(&transfers);
    let y_stats = stats.iter().find(|s| s.address == "Y").unwrap();

    assert!((y_stats.avg_buy_price - 1.75).abs() < 1e-6);

    assert!((y_stats.avg_sell_price - 3.0).abs() < 1e-6);

    assert!((y_stats.total_volume - 30.0).abs() < 1e-6);
}

#[test]
fn handles_missing_addresses_gracefully() {
    let transfers: Vec<Transfer> = vec![];
    let stats = calculate_user_stats(&transfers);
    assert!(stats.is_empty());
}
