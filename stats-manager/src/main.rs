use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};

async fn report(
    Extension(stats_list): Extension<Arc<Mutex<HashMap<String, Stats>>>>,
    Json(payload): Json<Report>,
) -> Json<ReportResult> {
    stats_list
        .lock()
        .unwrap()
        .insert(payload.host_name, payload.stats);
    println!("{:?}", stats_list);
    Json(ReportResult { result: "OK" })
}

async fn get_list(
    Extension(stats_list): Extension<Arc<Mutex<HashMap<String, Stats>>>>,
) -> Json<UserLists> {
    println!("{:?}", stats_list);
    Json(UserLists {
        data: stats_list.lock().unwrap().clone(),
    })
}

#[tokio::main]
async fn main() {
    // let stats_list = Arc::new(Mutex::new(HashMap::new()));
    let stats_list: Arc<Mutex<HashMap<String, Stats>>> = Arc::new(Mutex::new(HashMap::new()));

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/report", post(report))
        .route("/getlist", get(get_list))
        .layer(Extension(stats_list));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:2784")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct UserLists {
    data: HashMap<String, Stats>,
}

#[derive(Serialize)]
struct ReportResult {
    result: &'static str,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Stats {
    temperature: i32,
    used_mem: i32,
    total_mem: i32,
    cpu_usage: i32,
}

#[derive(Deserialize)]
struct Report {
    host_name: String,
    stats: Stats,
}
