use serde;

pub mod read;
pub mod write;

#[derive(serde::Serialize)]
struct SocketMessage {
    // A standard socket message format
    action: Option<String>,
    key: Option<String>,
    secret: Option<String>,
    trades: Option<Vec<String>>,
    quotes: Option<Vec<String>>,
    bars: Option<Vec<String>>,
}

pub enum DataChannel {
    Quotes,
    Trades,
    Bars,
}
