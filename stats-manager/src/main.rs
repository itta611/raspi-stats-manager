// use std::sync::{Arc, Mutex};

use std::sync::{Arc, Mutex};

use axum::{extract, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

async fn collect(
    Json(payload): Json<Stats>,
    stats_list: Arc<Mutex<Vec<Stats>>>,
) -> Json<CollectResult> {
    stats_list.lock().unwrap().push(payload);
    Json(CollectResult { result: "OK" })
}

#[tokio::main]
async fn main() {
    let stats_list: Vec<Stats> = vec![];
    let stats_list = Arc::new(Mutex::new(stats_list));

    tracing_subscriber::fmt::init();

    let app = Router::new().route(
        "/collect",
        post(move |payload| {
            let stats_list = stats_list.clone();
            collect(payload, stats_list)
        }),
    );

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
    temperature: i32,
    used_mem: i32,
    total_mem: i32,
    cpu_usage: i32,
}
