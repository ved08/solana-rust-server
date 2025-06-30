use axum::{debug_handler, extract::Query, routing::post, Router};
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/fetch-balance", post(fetch_balance));
    println!("Hello, world!");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    address: String,
}

#[debug_handler]
async fn fetch_balance(Query(address): Query<Address>) -> String {
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(rpc_url);
    let pubkey = Pubkey::from_str_const(&address.address);
    let balance = client.get_balance(&pubkey.to_bytes().into()).unwrap();
    format!("{}SOL", balance as f64 / LAMPORTS_PER_SOL as f64)
}
