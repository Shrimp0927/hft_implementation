#![allow(dead_code)]
mod sockets;
mod data;

#[tokio::main]
async fn main() {
    let (mut _write, mut _read) = sockets::connect_market_data_stream().await.unwrap();
    tokio::spawn(async move {
        let action = String::from("subscribe");
        let symbols = vec![String::from("FAKEPACA")];
        let _ = sockets::send_action_message(&mut _write, action.clone(), symbols.clone(), &sockets::DataChannel::Quotes).await;
        let _ = sockets::send_action_message(&mut _write, action.clone(), symbols.clone(), &sockets::DataChannel::Trades).await;
    });
    let _ = sockets::read_socket(&mut _read).await;
}
