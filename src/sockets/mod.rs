use futures_util::{Sink, Stream, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::{Message, Error}};
use serde;
use serde_json;
use dotenv;

#[derive(serde::Serialize)]
struct SocketMessage<'a> {
    action: Option<&'a str>,
    key: Option<&'a str>,
    secret: Option<&'a str>,
    trades: Option<Vec<&'a str>>,
    quotes: Option<Vec<&'a str>>,
    bars: Option<Vec<&'a str>>,
}

pub async fn connect_market_data_stream() -> Result<(impl Sink<Message> + Unpin, impl Stream<Item = Result<Message, Error>> + Unpin), Error> {
    // Connects to market data stream and returns (sink, stream)
    dotenv::dotenv().ok();
    let key = dotenv::var("KEY").unwrap();
    let secret = dotenv::var("SECRET").unwrap();
    let auth_message = SocketMessage {
        action: Some("auth"),
        key: Some(&key),
        secret: Some(&secret),
        trades: None,
        bars: None,
        quotes: None,
    };
    let (stream, _response) = connect_async(&dotenv::var("DATA_URL").unwrap()).await.unwrap();
    let (mut _write, mut _read) = stream.split();
    println!("Connected to market data stream");

    let auth_message_json = serde_json::json!(auth_message).to_string();
    let _ = _write.send(Message::Text(auth_message_json.into())).await;
    Ok((_write, _read))
}

pub async fn subscribe_to_quote<S>(mut write: S, symbol: &str) -> Result<(), Error>
where
    S: Sink<Message> + Unpin
{
    let symbols = vec![symbol];
    let subscribe_message = SocketMessage {
        action: Some("subscribe"),
        key: None,
        secret: None,
        trades: None,
        quotes: Some(symbols),
        bars: None,
    };
    let subscribe_message_json = serde_json::json!(subscribe_message).to_string();
    let _ = write.send(Message::Text(subscribe_message_json.into())).await;
    Ok(())
}

//pub async fn read_socket<S>(stream: S)
//where
//    S: Stream<Item = Result<Message, Error>>
//{
//    // Takes in a SplitStream and reads it
//}
