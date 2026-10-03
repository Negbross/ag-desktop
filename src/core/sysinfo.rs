use sysinfo::{Disks, System};

#[derive(serde::Serialize, Debug)]
pub struct SysOverview {
    pub cpu_percent: f32,
    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub disks: Vec<DiskInfo>,
}

#[derive(serde::Serialize, Debug)]
pub struct DiskInfo {
    pub mount: String,
    pub free_gb: f64,
    pub total_gb: f64,
}

pub fn overview_collect() -> anyhow::Result<SysOverview> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list().list().iter().map(|d| DiskInfo {
        mount: d.mount_point().display().to_string(),
        free_gb: d.available_space() as f64 / 1_073_741_824.0,
        total_gb: d.total_space() as f64 / 1_073_741_824.0,
    }).collect();
    Ok(SysOverview {
        cpu_percent: sys.global_cpu_usage(),
        ram_used_gb: sys.used_memory() as f64 / 1_073_741_824.0,
        ram_total_gb: sys.total_memory() as f64 / 1_073_741_824.0,
        disks,
    })
}