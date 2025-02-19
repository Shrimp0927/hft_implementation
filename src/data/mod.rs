use serde;
use serde_json;

pub mod quote;
pub mod trade;
pub mod bar;
pub mod subscription;

use quote::Quote;
use trade::Trade;
use bar::Bar;
use subscription::Subscription;

use crate::util::StreamObject;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "T")]
pub enum DataObject {
    #[serde(rename = "q")]
    Quote(Quote),
    #[serde(rename = "t")]
    Trade(Trade),
    #[serde(rename = "b")] // b, d, u: minute, daily, update
    Bar(Bar),
    #[serde(rename = "subscription")]
    Subscription(Subscription),
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error(DataStreamError),
}

impl StreamObject<DataObject> for DataObject {
    fn print_from_vec(objects: Vec<DataObject>) {
        for object in &objects {
            match object {
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
    }
} 

#[derive(Debug, serde::Deserialize)]
pub struct DataStreamError {
    #[serde(rename = "code")]
    pub error_code: u64,
    #[serde(rename = "msg")]
    pub message: String,
}

pub struct GlobalData {
    pub quotes: Vec<Quote>,
    pub trades: Vec<Trade>,
    pub bars: Vec<Bar>,
}

impl GlobalData {
    pub fn new() -> Self {
        Self {
            quotes: Vec::new(),
            trades: Vec::new(),
            bars: Vec::new(),
        }
    }
}
