use crate::model::Transfer;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PriceTracker {
    pub buys: HashMap<String, Vec<(f64, f64)>>,
    pub sells: HashMap<String, Vec<(f64, f64)>>,
}

impl PriceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, transfer: &Transfer) {
        self.buys
            .entry(transfer.to.clone())
            .or_default()
            .push((transfer.usd_price, transfer.amount));
        self.sells
            .entry(transfer.from.clone())
            .or_default()
            .push((transfer.usd_price, transfer.amount));
    }

    fn avg(data: &[(f64, f64)]) -> f64 {
        let (total_price, total_amt): (f64, f64) = data
            .iter()
            .copied()
            .fold((0.0, 0.0), |(sp, sa), (p, a)| (sp + p * a, sa + a));
        if total_amt > 0.0 {
            total_price / total_amt
        } else {
            0.0
        }
    }

    pub fn avg_buy_price(&self, addr: &str) -> f64 {
        Self::avg(
            self.buys
                .get(addr)
                .map(|v| v.as_slice())
                .unwrap_or_default(),
        )
    }

    pub fn avg_sell_price(&self, addr: &str) -> f64 {
        Self::avg(
            self.sells
                .get(addr)
                .map(|v| v.as_slice())
                .unwrap_or_default(),
        )
    }

    pub fn all_addresses(&self) -> impl Iterator<Item = &String> {
        self.buys.keys().chain(self.sells.keys())
    }
}
