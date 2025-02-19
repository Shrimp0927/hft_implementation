mod account;
mod order;
mod asset;
mod position;

use rust_decimal::Decimal;

use asset::Asset;
use order::Order;
use position::Position;
use account::Account;

use crate::util::StreamObject;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum TradeObject {
    Asset(Asset),
    Order(Order),
    Position(Position),
    Account(Account),
}

impl StreamObject<TradeObject> for TradeObject {
    fn print_from_vec(objects: Vec<TradeObject>) {
        for object in &objects {
            match object {
                TradeObject::Asset(a) => {
                    println!("{:?}", a);
                },
                TradeObject::Order(o) => {
                    println!("{:?}", o);
                },
                TradeObject::Position(p) => {
                    println!("{:?}", p);
                },
                TradeObject::Account(a) => {
                    println!("{:?}", a);
                },
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "gtc")]
    Gtc,
    #[serde(rename = "opg")]
    Opg,
    #[serde(rename = "cls")]
    Cls,
    #[serde(rename = "ioc")]
    Ioc,
    #[serde(rename = "fok")]
    Fok,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Amount {
    Quantity {
        #[serde(rename = "qty")]
        quantity: Decimal,
    },
    Notional {
        #[serde(rename = "notional")]
        notional: Decimal,
    },
}
