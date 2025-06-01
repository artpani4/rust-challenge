use super::balances::BalanceTracker;
use super::price_avg::PriceTracker;
use crate::model::{Transfer, UserStats};
use log::{debug, info};
use std::collections::HashSet;

pub fn calculate_user_stats(transfers: &[Transfer]) -> Vec<UserStats> {
    info!("Calculating user stats for {} transfers", transfers.len());

    let mut sorted: Vec<&Transfer> = transfers.iter().collect();
    sorted.sort_by_key(|t| t.ts);
    debug!("Sorted transfers by timestamp");

    let mut balances = BalanceTracker::new();
    let mut prices = PriceTracker::new();

    for t in &sorted {
        debug!(
            "Applying transfer {}: from={} to={} amount={}",
            t.id, t.from, t.to, t.amount
        );
        balances.apply(t);
        prices.apply(t);
    }

    let max_balances = balances.max_balances();
    let mut all_addresses: HashSet<&String> = HashSet::new();
    all_addresses.extend(prices.all_addresses());
    debug!("Collected {} unique addresses", all_addresses.len());

    let result = all_addresses
        .into_iter()
        .map(|addr| {
            let empty = Vec::new();
            let buys = prices.buys.get(addr).unwrap_or(&empty);
            let sells = prices.sells.get(addr).unwrap_or(&empty);
            let buy_volume: f64 = buys.iter().map(|(_, amt)| amt).sum();
            let sell_volume: f64 = sells.iter().map(|(_, amt)| amt).sum();
            let total_volume = buy_volume + sell_volume;

            let avg_buy = prices.avg_buy_price(addr);
            let avg_sell = prices.avg_sell_price(addr);
            let max_balance = *max_balances.get(addr).unwrap_or(&0.0);

            debug!(
                "Stats for {}: avg_buy={}, avg_sell={}, total_volume={}, max_balance={}",
                addr, avg_buy, avg_sell, total_volume, max_balance
            );

            UserStats {
                address: addr.to_string(),
                avg_buy_price: avg_buy,
                avg_sell_price: avg_sell,
                total_volume,
                max_balance,
            }
        })
        .collect();

    info!("Finished calculating stats");
    result
}
