use futures_util::{Sink, Stream, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::{Message, Error}};
use serde;
use serde_json;
use dotenv;

#[derive(serde::Serialize)]
struct SocketMessage<'a> {
    // A standard socket message format
    action: Option<&'a str>,
    key: Option<&'a str>,
    secret: Option<&'a str>,
    trades: Option<Vec<&'a str>>,
    quotes: Option<Vec<&'a str>>,
    bars: Option<Vec<&'a str>>,
}

pub enum DataChannel {
    Quotes,
    Trades,
    Bars,
}

pub async fn connect_market_data_stream() -> Result<(impl Sink<Message> + Unpin, impl Stream<Item = Result<Message, Error>> + Unpin), Error> {
    // Connects to market data stream
    // Returns: (sink, stream)
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

pub async fn send_action_message<S, T, U>(mut write: S, action: T, symbols: Vec<U>, channel: DataChannel) -> Result<(), Error>
where
    S: Sink<Message> + Unpin,
    T: AsRef<str>, for <'a> &'a str: From<T>,
    U: AsRef<str>, for<'a> &'a str: From<U>,
{
    let subscribe_message = match channel {
        DataChannel::Quotes => SocketMessage { action: Some(action.into()), key: None, secret: None, trades: None, quotes: Some(symbols.into_iter().map(Into::into).collect()), bars: None },
        DataChannel::Trades => SocketMessage { action: Some(action.into()), key: None, secret: None, trades: Some(symbols.into_iter().map(Into::into).collect()), quotes: None, bars: None },
        DataChannel::Bars => SocketMessage { action: Some(action.into()), key: None, secret: None, trades: None, quotes: None, bars: Some(symbols.into_iter().map(Into::into).collect())},
    };

    let subscribe_message_json = serde_json::json!(subscribe_message).to_string();
    let _ = write.send(Message::Text(subscribe_message_json.into())).await;
    Ok(())
}

pub async fn read_socket<S>(mut read: S)
where
    S: Stream<Item = Result<Message, Error>> + Unpin
{
    // Reads the socket stream to stdout
    // Note: At later date, could implement to write out to a file or excel or some other format
    loop {
        let Some(message) = read.next().await else { continue };
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => println!("Data stream message: {}", text),
                    Message::Binary(data) => println!("Data stream message(binary): {:?}", data),
                    _ => println!("Data stream message: {:?}", msg),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
