use super::balances::BalanceTracker;
use super::price_avg::PriceTracker;
use crate::model::{Transfer, UserStats};
use std::collections::HashSet;

pub fn calculate_user_stats(transfers: &[Transfer]) -> Vec<UserStats> {
    let mut sorted: Vec<&Transfer> = transfers.iter().collect();
    sorted.sort_by_key(|t| t.ts);

    let mut balances = BalanceTracker::new();
    let mut prices = PriceTracker::new();

    for t in &sorted {
        balances.apply(t);
        prices.apply(t);
    }

    let max_balances = balances.max_balances();
    let mut all_addresses: HashSet<&String> = HashSet::new();
    all_addresses.extend(prices.all_addresses());

    all_addresses
        .into_iter()
        .map(|addr| UserStats {
            address: addr.to_string(),
            avg_buy_price: prices.avg_buy_price(addr),
            avg_sell_price: prices.avg_sell_price(addr),
            total_volume: {
                let empty = Vec::new();
                let buys = prices.buys.get(addr).unwrap_or(&empty);
                let sells = prices.sells.get(addr).unwrap_or(&empty);
                let buy_volume: f64 = buys.iter().map(|(_, amt)| amt).sum();
                let sell_volume: f64 = sells.iter().map(|(_, amt)| amt).sum();
                buy_volume + sell_volume
            },
            max_balance: *max_balances.get(addr).unwrap_or(&0.0),
        })
        .collect()
}
