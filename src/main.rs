mod core;
mod llm_bridge;

use clap::{Parser, Subcommand};
use ag_desktop::execute;
use crate::core::{process, window, volume, sysinfo, fs, input };

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

    /// Gerakin mouse ke koordinat absolut
    MoveMouse { x: i32, y: i32 },
    /// Klik mouse
    Click { #[arg(default_value = "left")] button: String },
    /// Scroll vertikal (+ ke atas, - ke bawah)
    Scroll { amount: i32 },
    /// Ketik teks
    Type { text: String },
    /// Tekan key khusus (enter/tab/escape/backspace/space)
    Key { name: String },

    /// LLM Agent
    Ask { instruction: String }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Cli::parse().cmd {
        Cmd::Ls { path, depth } => fs::list(&path, depth),
        Cmd::Find { path, pattern } => fs::find(&path, &pattern),
        Cmd::Ps => { for p in process::list_process()? { println!("{:?}", p); } Ok(()) },
        Cmd::Kill { pid } => process::kill_process(pid),
        Cmd::Sys => {
            let overview = sysinfo::overview_collect()?;
            println!("{:?}", overview);
            Ok(())
        },
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

        Cmd::MoveMouse { x, y } => input::move_mouse(x, y),
        Cmd::Click { button } => input::click(&button),
        Cmd::Scroll { amount } => input::scroll(amount),
        Cmd::Type { text } => input::type_text(&text),
        Cmd::Key { name } => input::key_press(&name),

        Cmd::Ask { instruction } => {
            llm_bridge::agent_loop(&instruction, 10).await?;
            Ok(())
        }
    }
}