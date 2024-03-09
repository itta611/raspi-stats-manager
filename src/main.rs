use reqwest;
use tokio::time::{self, Duration};

mod types;
use types::Stat;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(10));
    let client = reqwest::Client::new();

    loop {
        let stat = Stat {
            tempreture: 40,
            memory: 4,
            cpu: 0.2,
        };
        client.post("https://192.168.1.101:2784").body(stat);
        println!("Hello, world!");
        interval.tick().await;
    }
}
