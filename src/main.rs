mod sockets;
mod data;

#[tokio::main]
async fn main() {
    let (mut _write, mut _read) = sockets::connect_market_data_stream().await.unwrap();
}
