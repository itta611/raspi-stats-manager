use reqwest::StatusCode;
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
    let master_port: &str = args.get(2).expect("2874");
    let host_name = match env::var("NODE_NAME") {
        Ok(val) => val,
        Err(_) => hostname::get().unwrap().into_string().unwrap(),
    };

    time::sleep(Duration::from_secs(10)).await;
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        stats_controller.update();

        let url = format!("http://{}:{}/report", master_ip, master_port);
        let payload = Report {
            host_name: host_name.clone(),
            stats: &stats_controller.stats,
        };
        let result = client.post(url).json(&payload).send().await;

        if result.is_err() {
            println!(
                "
Check host server is running correctly:
Failed to connect to master node ({})
",
                master_ip,
            );
        } else {
            let status_code = result.unwrap().status();
            if status_code != StatusCode::ACCEPTED {
                println!(
                    "
Server responsed status: {}
",
                    status_code
                );
            }
        }
    }
}
