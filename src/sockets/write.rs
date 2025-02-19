use futures_util::{Sink, SinkExt};
use tokio_tungstenite::tungstenite::{Message, Error};

use crate::sockets::DataChannel;
use crate::sockets::SocketMessage;

pub async fn send_action_message<S>(write: &mut S, action: String, symbols: Vec<String>, channel: &DataChannel) -> Result<(), Error>
where
    S: Sink<Message> + Unpin,
{
    let subscribe_message = match channel {
        DataChannel::Quotes => SocketMessage { action: Some(action), key: None, secret: None, trades: None, quotes: Some(symbols.to_vec()), bars: None },
        DataChannel::Trades => SocketMessage { action: Some(action), key: None, secret: None, trades: Some(symbols.to_vec()), quotes: None, bars: None },
        DataChannel::Bars => SocketMessage { action: Some(action), key: None, secret: None, trades: None, quotes: None, bars: Some(symbols.to_vec())},
    };

    let subscribe_message_json = serde_json::json!(subscribe_message).to_string();
    let _ = write.send(Message::Text(subscribe_message_json.into())).await;
    Ok(())
}
