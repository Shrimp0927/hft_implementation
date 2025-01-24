#![allow(dead_code)]
mod sockets;
mod data;
mod state;

use state::Subscriptions;

#[tokio::main]
async fn main() {
    let _subscriptions: Subscriptions<&str> = state::Subscriptions::new();
    //let (mut _write, mut _read) = sockets::connect_market_data_stream().await.unwrap();
    //tokio::spawn(async move {
    //    let _ = sockets::subscribe_to_quotes(_write, "subscribe", "FAKEPACA").await;
    //});
    //sockets::read_socket(_read).await;
}
