use serde;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::trading::asset::{AssetExchange, AssetClass};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum PositionSide {
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "long")]
    Long,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Position {
    #[serde(rename = "asset_id")]
    pub asset_id: Uuid,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "asset_exchange")]
    pub exchange: AssetExchange,
    #[serde(rename = "asset_class")]
    pub class: AssetClass,
    #[serde(rename = "asset_marginable")]
    pub marginable: bool,
    #[serde(rename = "avg_entry_price")]
    pub average_entry_price: Decimal,
    #[serde(rename = "qty")]
    pub quantity: Decimal,
    #[serde(rename = "side")]
    pub side: PositionSide,
    #[serde(rename = "market_value")]
    pub market_value: Option<Decimal>,
    #[serde(rename = "cost_basis")]
    pub cost_basis: Decimal,
    #[serde(rename = "unrealized_pl")]
    pub unrealized_gain: Option<Decimal>,
    #[serde(rename = "unrealized_plpc")]
    pub unrealized_gain_percent: Option<Decimal>,
    #[serde(rename = "unrealized_intraday_pl")]
    pub unrealized_gain_today: Option<Decimal>,
    #[serde(rename = "unrealized_intraday_plpc")]
    pub unrealized_gain_today_percent: Option<Decimal>,
    #[serde(rename = "current_price")]
    pub current_price: Option<Decimal>,
    #[serde(rename = "lastday_price")]
    pub lastday_price: Option<Decimal>,
    #[serde(rename = "change_today")]
    pub change_today: Option<Decimal>,
    #[serde(rename = "qty_available")]
    pub quantity_available: Decimal,
    #[serde(default)]
    #[serde(flatten)]
    extra: serde_json::Value,
}
