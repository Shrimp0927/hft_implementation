use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub trade_price: Decimal,
    #[serde(rename = "s")]
    pub trade_size: usize,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
}
