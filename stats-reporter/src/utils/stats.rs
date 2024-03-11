use std::{fs, io};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Stats {
    pub tempreture: Option<i32>,
    pub used_mem: Option<i32>,
    pub total_mem: Option<i32>,
    pub cpu_usage: Option<f32>,
    system: System,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            tempreture: None,
            used_mem: None,
            total_mem: None,
            cpu_usage: None,
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            ),
        }
    }

    pub fn to_json(&self) -> String {
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
        self.system.refresh_memory();

        self.tempreture = Some(get_tempreture().unwrap());
        self.used_mem = Some(get_used_memory(&self.system));
        self.total_mem = Some(get_total_memory(&self.system));
        self.cpu_usage = Some(get_cpu_usage(&self.system));

        self
    }
}

fn get_tempreture() -> Result<i32, io::Error> {
    let output = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let tempreture = output.trim().parse::<f32>().unwrap() / 1000f32;

    Ok(tempreture.round() as i32)
}

fn get_used_memory(system: &System) -> i32 {
    (system.used_memory() as f32 / 1024_f32 / 1024_f32 / 1024_f32).round() as i32
}

fn get_total_memory(system: &System) -> i32 {
    (system.total_memory() as f32 / 1024_f32 / 1024_f32 / 1024_f32).round() as i32
}

fn get_cpu_usage(system: &System) -> f32 {
    system.global_cpu_info().cpu_usage()
}
