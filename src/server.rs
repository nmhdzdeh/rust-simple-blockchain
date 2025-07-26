use crate::blockchain::Blockchain;
use axum::{Json, Router, routing::get};
use std::sync::{Arc, Mutex};

pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>) {
    let app = Router::new()
        .route("/blocks", get(get_blocks))
        .with_state(blockchain);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn get_blocks(
    state: axum::extract::State<Arc<Mutex<Blockchain>>>,
) -> Json<Vec<crate::block::Block>> {
    let chain = state.lock().unwrap();
    Json(chain.blocks.clone())
}

