use rust_challenge::generator::generate_transfers;

#[test]
fn generate_expected_count() {
    let transfers = generate_transfers(100).expect("wtf");
    assert_eq!(transfers.len(), 100);
}

#[test]
fn generated_transefers_have_valid_data() {
    let transfers = generate_transfers(100).expect("wtf");
    for t in &transfers {
        assert_ne!(t.from, t.to, "should differ");
        assert!(t.amount >= 1.0 && t.amount <= 1000.0);
        assert!(t.usd_price >= 0.1 && t.usd_price <= 2.0);
        assert!(t.ts > 0);
    }
}

pub mod ololo {
    fn ololo() -> () {}
}
