use serde::Serialize;
use std::env;
use tokio::time::{self, Duration};

mod utils;
use utils::stats::{Stats, StatsController};

#[derive(Serialize)]
struct Report<'a> {
    host_name: String,
    stats: &'a Stats,
}

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(10));
    let mut stats_controller = StatsController::new();
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

    time::sleep(Duration::from_secs(10)).await;

    loop {
        interval.tick().await;

        stats_controller.update();

        let url = format!("http://{}:{}/report", master_ip, master_port);
        let hostname = hostname::get().unwrap();
        let payload = Report {
            host_name: hostname.into_string().unwrap(),
            stats: &stats_controller.stats,
        };
        let result = client.post(url).json(&payload).send().await;

        if result.is_err() {
            println!(
                "
        Failed to connect to master node ({}):
        Check host server is running correctly.

        ",
                master_ip
            );
        }
    }
}
