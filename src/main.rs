mod core;

use clap::{Parser, Subcommand};

use crate::core::{ process, window, volume, sysinfo, fs };

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List isi direktori
    Ls { path: String, #[arg(short, long, default_value_t = 1)] depth: usize },
    /// Cari file berdasarkan nama
    Find { path: String, pattern: String },
    /// List proses jalan
    Ps,
    /// Kill proses berdasarkan PID
    Kill { pid: u32 },

    /// Info sistem: CPU, RAM, disk, battery
    Sys,

    /// Lihat volume saat ini
    VolGet,
    /// Set volume (0-100)
    VolSet { percent: f32 },
    Mute,
    Unmute,

    /// List semua window terbuka
    Win,
    Focus { hwnd: isize },
    Minimize { hwnd: isize },
    Maximize { hwnd: isize },
    Close { hwnd: isize },
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().cmd {
        Cmd::Ls { path, depth } => fs::list(&path, depth),
        Cmd::Find { path, pattern } => fs::find(&path, &pattern),
        Cmd::Ps => process::list_process(),
        Cmd::Kill { pid } => process::kill_process(pid),
        Cmd::Sys => sysinfo::overview(),
        Cmd::VolGet => { println!("{:.0}%", volume::get_volume()?); Ok(()) }
        Cmd::VolSet { percent } => volume::set_volume(percent),
        Cmd::Mute => volume::mute_volume(true),
        Cmd::Unmute => volume::mute_volume(false),
        Cmd::Win => {
            for w in window::list()? {
                println!("{:>12}  pid={:<7} {}", w.hwnd, w.pid, w.title);
            }
            Ok(())
        }
        Cmd::Focus { hwnd } => window::focus(hwnd),
        Cmd::Minimize { hwnd } => window::minimize(hwnd),
        Cmd::Maximize { hwnd } => window::maximize(hwnd),
        Cmd::Close { hwnd } => window::close(hwnd),
    }
}