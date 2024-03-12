use serde::{Deserialize, Serialize};
use std::{fs, io};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Serialize)]
pub struct Stats {
    pub hostname: String,
    pub temperature: Option<i32>,
    pub used_mem: Option<i32>,
    pub total_mem: Option<i32>,
    pub cpu_usage: Option<i32>,
}

pub struct StatsController {
    pub stats: Stats,
    system: System,
}

impl StatsController {
    pub fn new() -> StatsController {
        let stats = Stats {
            hostname: String::from("asdf"),
            temperature: None,
            used_mem: None,
            total_mem: None,
            cpu_usage: None,
        };
        StatsController {
            stats,
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            ),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_memory();

        self.stats.temperature = Some(get_temperature().unwrap());
        self.stats.used_mem = Some(get_used_memory(&self.system));
        self.stats.total_mem = Some(get_total_memory(&self.system));
        self.stats.cpu_usage = Some(get_cpu_usage(&self.system));
    }
}

fn get_temperature() -> Result<i32, io::Error> {
    let output = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let temperature = output.trim().parse::<f32>().unwrap() / 1000f32;

    Ok(temperature.round() as i32)
}

fn get_used_memory(system: &System) -> i32 {
    (system.used_memory() as f32 / 1024_f32 / 1024_f32 / 1024_f32).round() as i32
}

fn get_total_memory(system: &System) -> i32 {
    (system.total_memory() as f32 / 1024_f32 / 1024_f32 / 1024_f32).round() as i32
}

fn get_cpu_usage(system: &System) -> i32 {
    system.global_cpu_info().cpu_usage().round() as i32
}
