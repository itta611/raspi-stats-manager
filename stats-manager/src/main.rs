// use std::sync::{Arc, Mutex};

use axum::{extract, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

async fn collect(payload: extract::Json<Stats>) -> Json<CollectResult> {
    println!("{:?}", payload.tempreture);
    Json(CollectResult { result: "OK" })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/collect", post(collect));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:2784")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct CollectResult {
    result: &'static str,
}

#[derive(Deserialize)]
struct Stats {
    tempreture: i32,
    used_mem: i32,
    total_mem: i32,
    cpu_usage: i32,
}
