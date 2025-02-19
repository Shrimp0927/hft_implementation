use futures_util::{Sink, Stream, SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::{Message, Error}};
use serde_json;
use serde;
use dotenv;

use crate::data::DataObject;
use crate::trading::TradeObject;
use crate::sockets::SocketMessage;
use crate::util::StreamObject;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum StreamData {
    DataObject(DataObject),
    TradeObject(TradeObject),
}

impl StreamObject<StreamData> for StreamData {
    fn print_from_vec(objects: Vec<StreamData>) {
        let mut data_objects = Vec::new();
        let mut trade_objects = Vec::new();

        for object in objects {
            match object {
                StreamData::DataObject(data) => data_objects.push(data),
                StreamData::TradeObject(trade) => trade_objects.push(trade),
            }
        }

        if !data_objects.is_empty() {
            DataObject::print_from_vec(data_objects);
        }

        if !trade_objects.is_empty() {
            TradeObject::print_from_vec(trade_objects);
        }
    }
}

pub async fn connect_stream(url: String) -> Result<(impl Sink<Message> + Unpin, impl Stream<Item = Result<Message, Error>> + Unpin), Error> {
    // Connects to market data stream
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
    let (stream, _response) = connect_async(dotenv::var(url.clone()).unwrap()).await.unwrap();
    let (mut _write, mut _read) = stream.split();
    println!("Connected to {} stream", url.clone());

    let auth_message_json = serde_json::json!(auth_message).to_string();
    let _ = _write.send(Message::Text(auth_message_json.into())).await;
    Ok((_write, _read))
}

pub async fn read_socket<S>(read: &mut S)
where
    S: Stream<Item = Result<Message, Error>> + Unpin,
{
    // Reads the socket stream to stdout and track the objects in GlobalData objects
    // Note: At later date, could implement to write out to a file or excel or some other format
    loop {
        let Some(message) = read.next().await else { continue };
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        let objects: Vec<StreamData> = serde_json::from_str(&text).unwrap();
                        StreamData::print_from_vec(objects);
                    },
                    Message::Binary(data) => {
                        println!("Stream message(binary): {:?}", data);
                    },
                    _ => println!("Sstream message: {:?}", msg),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
