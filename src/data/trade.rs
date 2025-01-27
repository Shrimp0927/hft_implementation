use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
//use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub trade_price: Decimal,
    #[serde(rename = "s")]
    pub trade_size: usize,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(flatten)]
    extra: serde_json::Value,
}
