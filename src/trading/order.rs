use serde;
use crate::util::vec_deserializer;

use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::DateTime;
use chrono::Utc;

use crate::trading::asset::AssetClass;
use crate::trading::TimeInForce;
use crate::trading::Amount;

pub fn create_new_order() {
    // TODO
}

#[derive(Debug, serde::Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "done_for_day")]
    DoneForDay,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "replaced")]
    Replaced,
    #[serde(rename = "pending_cancel")]
    PendingCancel,
    #[serde(rename = "pending_repalce")]
    PendingReplace,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending_new")]
    PendingNew,
    #[serde(rename = "accepted_for_bidding")]
    AcceptedForBidding,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "calculated")]
    Calculated,
}

#[derive(Debug, serde::Deserialize)]
pub enum OrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug)]
pub enum OrderClass {
    Simple, // market, limit, stop loss, orders
    Bracket,
    OTO,
    OCO,
    Mleg,
    Other, // Error! should not be able to get this
}

impl<'de> serde::Deserialize<'de> for OrderClass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let order_class = String::deserialize(deserializer)?;
        match order_class.as_str() {
            "" => Ok(OrderClass::Simple),
            "bracket" => Ok(OrderClass::Bracket),
            "oto" => Ok(OrderClass::OTO),
            "OCO" => Ok(OrderClass::OCO),
            "mleg" => Ok(OrderClass::Mleg),
            _other => Ok(OrderClass::Other),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub enum OrderType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

#[derive(Debug, serde::Deserialize)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "status")]
    pub status: OrderStatus,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "submitted_at")]
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(rename = "filled_at")]
    pub filled_at: Option<DateTime<Utc>>,
    #[serde(rename = "expired_at")]
    pub expired_at: Option<DateTime<Utc>>,
    #[serde(rename = "canceled_at")]
    pub canceled_at: Option<DateTime<Utc>>,
    #[serde(rename = "asset_class")]
    pub asset_class: AssetClass,
    #[serde(rename = "asset_id")]
    pub asset_id: Uuid,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub amount: Amount,
    #[serde(rename = "filled_qty")]
    pub filled_quantity: Decimal,
    #[serde(rename = "type")]
    pub type_: OrderType,
    #[serde(rename = "order_class")]
    pub class: OrderClass,
    #[serde(rename = "side")]
    pub side: OrderSide,
    #[serde(rename = "time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "limit_price")]
    pub limit_price: Option<Decimal>,
    #[serde(rename = "stop_price")]
    pub stop_price: Option<Decimal>,
    #[serde(rename = "trail_price")]
    pub trail_price: Option<Decimal>,
    #[serde(rename = "trail_percent")]
    pub trail_percent: Option<Decimal>,
    #[serde(rename = "filled_avg_price")]
    pub average_fill_price: Option<Decimal>,
    #[serde(rename = "extended_hours")]
    pub extended_hours: bool,
    #[serde(rename = "legs", deserialize_with = "vec_deserializer")]
    pub legs: Vec<Order>,
    #[serde(default)]
    #[serde(flatten)]
    extra: serde_json::Value,
}
