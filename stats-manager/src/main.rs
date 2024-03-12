use std::sync::{Arc, Mutex};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

async fn collect(payload: Json<Stats>, stats_list: Arc<Mutex<Vec<i32>>>) -> Json<CollectResult> {
    println!("{:?}", payload.tempreture);
    Json(CollectResult { result: "OK" })
}

#[tokio::main]
async fn main() {
    let stats_list: Vec<i32> = vec![1, 2, 3];
    let stats_list = Arc::new(Mutex::new(stats_list));

    tracing_subscriber::fmt::init();

    let app = Router::new().route(
        "/collect",
        post(move |payload| {
            let stats_list = stats_list.clone();
            collect(payload, stats_list)
        }),
    );

    // run our app with hyper, listening globally on port 3000
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
    pub tempreture: i32,
    pub used_mem: i32,
    pub total_mem: i32,
    pub cpu_usage: i32,
}
