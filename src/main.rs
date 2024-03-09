use tokio::time::{self, Duration};

mod utils;
use utils::stats::Stats;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(10));
    let client = reqwest::Client::new();
    let stat = Stats::new();

    loop {
        client
            .post("https://192.168.1.101:2784")
            .body(stat.to_json())
            .send()
            .await
            .unwrap();
        println!("Hello, world!");
        interval.tick().await;
    }
}
