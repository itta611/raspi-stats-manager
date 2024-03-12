use std::env;

use tokio::time::{self, Duration};

mod utils;
use utils::stats::StatsController;

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

    // time::sleep(Duration::from_secs(10)).await;

    loop {
        interval.tick().await;

        stats_controller.update();

        let url = format!("http://{}:{}/collect", master_ip, master_port);
        // let s = serde_json::json(&stats_controller.stats).unwrap();
        let result = client.post(url).json(&stats_controller.stats).send().await;
        // println!("{}", s);
        println!("{:?}", result);

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
