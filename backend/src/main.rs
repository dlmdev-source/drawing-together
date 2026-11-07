use axum::{
    routing::{get, post},
    Json, Router,
};
use rand::Rng;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello)).route("/api/rooms", post(create_room));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async  fn hello() -> &'static str {
    "Hello World !"
}

async fn create_room() -> Json<Value> {
    let code = random_code();
    Json(json!({"code": code}))
}

fn random_code() -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();

    (0..6).map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char).collect()

}