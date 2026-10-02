use sysinfo::{System, Pid};

pub fn list_process() -> anyhow::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut procs: Vec<_> = sys.processes().values().collect();
    procs.sort_by_key(|p| std::cmp::Reverse(p.memory()));
    for p in procs.iter().take(30) {
        println!(
            "{:>7}  {:>8} MB  {}",
            p.pid(),
            p.memory() / 1_048_576,
            p.name().to_string_lossy()
        );
    }
    Ok(())
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