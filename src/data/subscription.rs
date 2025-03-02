use serde;

// Subscribe struct tracks subscriptions for channels of the data stream
#[derive(Debug, serde::Deserialize)]
pub struct Subscription {
    #[serde(default)]
    quotes: Vec<String>,
    #[serde(default)]
    trades: Vec<String>,
    #[serde(default)]
    bars: Vec<String>,
}
