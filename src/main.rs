mod block;
mod blockchain;
mod cli;
mod storage;
mod server;

use std::sync::{Arc, Mutex};
use blockchain::Blockchain;

#[tokio::main]
async fn main() {
    //cli::run();
    let blockchain=Arc::new(Mutex::new(Blockchain::new()));
    server::start_server(blockchain).await;
}
