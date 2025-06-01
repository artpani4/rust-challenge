use crate::model::Transfer;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BalanceTracker {
    current: HashMap<String, f64>,
    max: HashMap<String, f64>,
}

impl BalanceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, transfer: &Transfer) {
        let from = &transfer.from;
        let to = &transfer.to;

        *self.current.entry(from.clone()).or_default() -= transfer.amount;
        *self.current.entry(to.clone()).or_default() += transfer.amount;

        let from_balance = self.current[from];
        let to_balance = self.current[to];

        self.max
            .entry(from.clone())
            .and_modify(|b| *b = b.max(from_balance))
            .or_insert(from_balance);
        self.max
            .entry(to.clone())
            .and_modify(|b| *b = b.max(to_balance))
            .or_insert(to_balance);
    }

    pub fn max_balances(&self) -> &HashMap<String, f64> {
        &self.max
    }
}
