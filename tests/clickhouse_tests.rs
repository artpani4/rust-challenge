use dotenvy::dotenv;
use rust_challenge::config::ClickhouseConfig;
use rust_challenge::model::Transfer;
use rust_challenge::storage::clickhouse::ClickhouseStorage;
use std::env;
use uuid::Uuid;

fn load_test_config() -> ClickhouseConfig {
    dotenv().ok();

    ClickhouseConfig {
        url: env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL not set"),
        user: env::var("CLICKHOUSE_USER").expect("CLICKHOUSE_USER not set"),
        password: env::var("CLICKHOUSE_PASSWORD").expect("CLICKHOUSE_PASSWORD not set"),
        database: env::var("CLICKHOUSE_DATABASE").expect("CLICKHOUSE_DATABASE not set"),
    }
}

async fn clean_table(storage: &ClickhouseStorage) {
    storage.truncate_table().await.unwrap();
}

#[tokio::test]
async fn test_create_and_insert_and_read() {
    let config = load_test_config();
    let storage = ClickhouseStorage::new(&config);

    storage.create_table().await.unwrap();
    clean_table(&storage).await;

    let test_data = vec![
        Transfer {
            id: Uuid::new_v4(),
            ts: 1234567890,
            from: "addr_from".to_string(),
            to: "addr_to".to_string(),
            amount: 42.5,
            usd_price: 1.23,
        },
        Transfer {
            id: Uuid::new_v4(),
            ts: 1234567891,
            from: "addr_from2".to_string(),
            to: "addr_to2".to_string(),
            amount: 100.0,
            usd_price: 2.34,
        },
    ];

    storage.insert_transfers(test_data.clone()).await.unwrap();

    let rows = storage.read_all_transfers().await.unwrap();

    assert_eq!(rows.len(), test_data.len());
}
