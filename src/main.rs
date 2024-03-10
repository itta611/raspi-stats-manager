use tokio::time::{self, Duration};

mod utils;
use utils::stats::Stats;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(10));
    let client = reqwest::Client::new();
    let mut stats = Stats::new();
    let master_ip = "192.168.1.101";
    let master_port = "2784";

    loop {
        stats.update();

        let url = format!("http://{}:{}", master_ip, master_port);
        let result = client.post(url).body(stats.to_json()).send().await;

        if result.is_err() {
            panic!(
                "
Failed to connect to master node ({}):
Check host server is running correctly.

",
                master_ip
            );
        }

        interval.tick().await;
    }
}
