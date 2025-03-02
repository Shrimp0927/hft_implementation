mod sockets;
mod data;
mod trading;
mod storage;
mod util;
mod algorithm;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let (mut _write, mut _read) = sockets::read::connect_stream(String::from("DATA_URL")).await.unwrap();
    let (mut trade_write, mut trade_read) = sockets::read::connect_stream(String::from("TRADE_URL")).await.unwrap();
    let task1 = tokio::task::spawn(async move {
        let action = String::from("subscribe");
        let symbols = vec![String::from("FAKEPACA")];
        let _ = sockets::write::send_action_message(&mut _write, action.clone(), symbols.clone(), &sockets::DataChannel::Quotes).await;
        let _ = sockets::write::send_action_message(&mut _write, action.clone(), symbols.clone(), &sockets::DataChannel::Trades).await;
    });
    let task2 = tokio::task::spawn(async move {
        let _ = sockets::read::read_socket(&mut trade_read).await;
    });

    let task3 = tokio::task::spawn(async move {
        let _ = sockets::read::read_socket(&mut _read).await;
    });

    let _ = tokio::join!(task1, task2, task3);
}
