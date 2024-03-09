// use std::net::TcpListener;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    // TcpListener::bind("127.0.0.1:2784").unwrap();
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        println!("Hello, world!");
    }
}
