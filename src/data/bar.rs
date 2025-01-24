use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
//use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Bar {
    #[serde(rename = "T")]
    pub bar_type: String, // b, d, u: minute, daily, update
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
}
