mod sockets;
mod data;
mod trading;
mod storage;
mod util;
mod algorithm;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    algorithm::main_algorithm().await;
}
