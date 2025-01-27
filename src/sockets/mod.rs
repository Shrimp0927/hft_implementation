use futures_util::{Sink, Stream, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::{Message, Error}};
use serde;
use serde_json;
use dotenv;

use crate::data::DataObject;

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

pub async fn connect_market_data_stream() -> Result<(impl Sink<Message> + Unpin, impl Stream<Item = Result<Message, Error>> + Unpin), Error> {
    // Connects to market data stream
    // Returns: (sink, stream)
    dotenv::dotenv().ok();
    let key = dotenv::var("KEY").unwrap();
    let secret = dotenv::var("SECRET").unwrap();
    let auth_message = SocketMessage {
        action: Some(String::from("auth")),
        key: Some(key),
        secret: Some(secret),
        trades: None,
        bars: None,
        quotes: None,
    };
    let (stream, _response) = connect_async(dotenv::var("DATA_URL").unwrap()).await.unwrap();
    let (mut _write, mut _read) = stream.split();
    println!("Connected to market data stream");

    let auth_message_json = serde_json::json!(auth_message).to_string();
    let _ = _write.send(Message::Text(auth_message_json.into())).await;
    Ok((_write, _read))
}

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

pub async fn read_socket<S>(read: &mut S)
where
    S: Stream<Item = Result<Message, Error>> + Unpin
{
    // Reads the socket stream to stdout and track the objects in GlobalData objects
    // Note: At later date, could implement to write out to a file or excel or some other format
    loop {
        let Some(message) = read.next().await else { continue };
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        let entities: Vec<DataObject> = serde_json::from_str(&text).unwrap();
                        for entity in &entities {
                            match entity {
                                DataObject::Quote(q) => {
                                    println!("{:?}", q);
                                },
                                DataObject::Trade(t) => {
                                    println!("{:?}", t);
                                },
                                DataObject::Bar(b) => {
                                    println!("{:?}", b);
                                },
                                DataObject::Subscription(data) => {
                                    println!("{:?}", data);
                                },
                                DataObject::Success => {
                                    println!("succesful operation");
                                },
                                DataObject::Error(e) => {
                                    println!("{:?}", e);
                                },
                            }
                        }
                    },
                    Message::Binary(data) => {
                        println!("Data stream message(binary): {:?}", data);
                    },
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
