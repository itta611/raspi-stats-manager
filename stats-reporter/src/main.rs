use std::env;

use tokio::time::{self, Duration};

mod utils;
use utils::stats::Stats;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(10));
    let mut stats = Stats::new();
    let client = reqwest::Client::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!(
            "
Master node IP address not provided.
{} {{IP_ADDRESS}}
",
            args[0]
        )
    }

    let master_ip: &str = &args[1];
    let master_port = "2784";

    loop {
        stats.update();

        let url = format!("http://{}:{}", master_ip, master_port);
        let result = client.post(url).body(stats.to_json()).send().await;

        if result.is_err() {
            println!(
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
