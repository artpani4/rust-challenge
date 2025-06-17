use crate::config::ClickhouseConfig;
use crate::model::Transfer;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Row, Debug, Serialize, Deserialize)]
pub struct TransferRow {
    #[serde(with = "clickhouse::serde::uuid")]
    pub id: Uuid,
    pub ts: u64,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub usd_price: f64,
}

impl From<Transfer> for TransferRow {
    fn from(t: Transfer) -> Self {
        Self {
            id: t.id,
            ts: t.ts,
            from: t.from,
            to: t.to,
            amount: t.amount,
            usd_price: t.usd_price,
        }
    }
}

pub struct ClickhouseStorage {
    client: Client,
}

impl ClickhouseStorage {
    pub fn new(cfg: &ClickhouseConfig) -> Self {
        let client = Client::default()
            .with_url(&cfg.url)
            .with_user(&cfg.user)
            .with_password(&cfg.password)
            .with_database(&cfg.database);
        Self { client }
    }

    pub async fn read_all_transfers(&self) -> clickhouse::error::Result<Vec<Transfer>> {
        self.client
            .query("SELECT id, ts, from, to, amount, usd_price FROM transfers")
            .fetch_all::<Transfer>()
            .await
    }

    pub async fn truncate_table(&self) -> clickhouse::error::Result<()> {
        self.client
            .query("TRUNCATE TABLE IF EXISTS transfers")
            .execute()
            .await
    }

    pub async fn create_table(&self) -> clickhouse::error::Result<()> {
        self.client
            .query(
                r"
                CREATE TABLE IF NOT EXISTS transfers (
                    id UUID,
                    ts UInt64,
                    from String,
                    to String,
                    amount Float64,
                    usd_price Float64
                ) ENGINE = MergeTree
                ORDER BY ts
                ",
            )
            .execute()
            .await
    }

    pub async fn insert_transfers(
        &self,
        transfers: Vec<Transfer>,
    ) -> clickhouse::error::Result<()> {
        let mut insert = self.client.insert("transfers")?;
        for t in transfers {
            let row: TransferRow = t.into();
            insert.write(&row).await?;
        }
        insert.end().await
    }
}
