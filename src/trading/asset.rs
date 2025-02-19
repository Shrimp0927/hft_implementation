use serde;

use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub enum AssetClass {
    #[serde(rename = "us_equity")]
    UsEquity,
    #[serde(rename = "us_option")]
    UsOption,
    #[serde(rename = "crypto")]
    Crypto,
}

#[derive(Debug, serde::Deserialize)]
pub enum AssetStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

#[derive(Debug, serde::Deserialize)]
pub enum AssetExchange {
    #[serde(rename = "AMEX")]
    AMEX,
    #[serde(rename = "ARCA")]
    ARCA,
    #[serde(rename = "BATS")]
    BATS,
    #[serde(rename = "NYSE")]
    NYSE,
    #[serde(rename = "NASDAQ")]
    NASDAQ,
    #[serde(rename = "NYSEARCA")]
    NYSEARCA,
    #[serde(rename = "FTXU")]
    FTXU,
    #[serde(rename = "CBSE")]
    CBSE,
    #[serde(rename = "GNSS")]
    GNSS,
    #[serde(rename = "ERSX")]
    ERSX,
    #[serde(rename = "OTC")]
    OTC,
    #[serde(rename = "CRYPTO")]
    CRYPTO,
    #[serde(other, rename(serialize = "Other"))]
    Other,
}

#[derive(Debug, serde::Deserialize)]
pub struct Asset {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "exchange")]
    pub exchange: AssetExchange,
    #[serde(rename = "asset_class")]
    pub class: AssetClass,
    #[serde(rename = "")]
    pub tradable: bool,
    #[serde(rename = "marginable")]
    pub marginable: bool,
    #[serde(rename = "shortable")]
    pub shortable: bool,
    #[serde(rename = "fractionable")]
    pub fractionable: bool,
    #[serde(rename = "easy_to_borrow")]
    pub easy_to_borrow: bool,
    #[serde(default)]
    #[serde(flatten)]
    extra: serde_json::Value,
}
