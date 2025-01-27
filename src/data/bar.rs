use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
//use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bar {
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "o")]
    pub open_price: Decimal,
    #[serde(rename = "h")]
    pub high_price: Decimal,
    #[serde(rename = "l")]
    pub low_price: Decimal,
    #[serde(rename = "c")]
    pub close_price: Decimal,
    #[serde(rename = "v")]
    pub volume: usize,
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(flatten)]
    extra: serde_json::Value,
}
