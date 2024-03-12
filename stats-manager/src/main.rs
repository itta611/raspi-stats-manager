use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

async fn collect() -> Json<CollectResult> {
    Json(CollectResult { result: "OK" })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/collect", get(collect));

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
