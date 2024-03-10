// use async_process::Command;
use std::{fs, io};
// use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

pub struct Stats {
    pub tempreture: Option<i32>,
    pub used_mem: Option<i32>,
    pub total_mem: Option<i32>,
    pub cpu_usage: Option<f32>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            tempreture: None,
            used_mem: None,
            total_mem: None,
            cpu_usage: None,
        }
    }

    pub fn to_json(&self) -> String {
        // &self.tempreture
        format!(
            "{{
                \"tempreture\": {},
                \"usedMemory\": {},
                \"totalMemory\": {},
                \"cpuUsage\": {},
            }}",
            &self.tempreture.unwrap(),
            &self.used_mem.unwrap(),
            &self.total_mem.unwrap(),
            &self.cpu_usage.unwrap(),
        )
    }

    pub fn update(&mut self) -> &Stats {
        self.tempreture = Some(get_tempreture().unwrap());
        self
    }
}

fn get_tempreture() -> Result<i32, io::Error> {
    let output = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let tempreture = output.parse::<f32>().unwrap() / 1000f32;
    Ok(tempreture.round() as i32)
}

// pub async fn get() -> Stats {
//     let mut system =
//         System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
//     system.refresh_memory();

//     let used_mem =
//         (system.used_memory() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
//     let total_mem =
//         (system.total_memory() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
//     let cpu_usage = system.global_cpu_info().cpu_usage();
//     let cpu_tempreture = get_cpu_tempreture().await.unwrap().parse::<f32>().unwrap() / 1000.0;

//     Stats {
//         tempreture,
//         used_mem,
//         total_mem,
//         cpu_usage,
//     }
// }
