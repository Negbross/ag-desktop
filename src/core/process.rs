use sysinfo::{System, Pid};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ProcInfo {
    pub pid: u32,
    pub name: String,
    pub memory_mb: u64,
}

pub fn list_process() -> anyhow::Result<Vec<ProcInfo>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut procs: Vec<_> = sys.processes().values().collect();
    procs.sort_by_key(|p| std::cmp::Reverse(p.memory()));
    Ok(procs.iter().take(30).map(|p| ProcInfo {
        pid: p.pid().as_u32(),
        name: p.name().to_string_lossy().into_owned(),
        memory_mb: p.memory() / 1_048_576,
    }).collect())
}

pub fn kill_process(pid: u32) -> anyhow::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    match sys.process(Pid::from_u32(pid)) {
        Some(p) => println!("{}", if p.kill() { "killed" } else { "gagal (butuh admin?)" }),
        None => println!("PID {pid} gak ketemu"),
    }
    Ok(())
}

pub fn launch_process(name: &str) -> anyhow::Result<()> {
    std::process::Command::new(name).spawn()?;
    Ok(())
}