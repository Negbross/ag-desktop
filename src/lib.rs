pub mod core;

use crate::core::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AgentAction {
    Done,
    // --- Filesystem ---
    ListDir { path: String, depth: usize },
    FindFile { path: String, pattern: String },

    // --- Process ---
    ListProcesses,
    KillProcess { pid: u32 },
    LaunchProcess { name: String },

    // --- Window ---
    ListWindows,
    FocusWindow { hwnd: isize },
    MinimizeWindow { hwnd: isize },
    MaximizeWindow { hwnd: isize },
    RestoreWindow { hwnd: isize },
    CloseWindow { hwnd: isize },

    // --- System ---
    SystemOverview,
    GetVolume,
    SetVolume { percent: f32 },
    Mute,
    Unmute,

    // --- Input ---
    MoveMouse { x: i32, y: i32 },
    Click { button: String },
    Scroll { amount: i32 },
    TypeText { text: String },
    KeyPress { key: String },
}

#[derive(Serialize, Debug)]
pub struct ActionResult {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl ActionResult {
    fn ok(value: impl Serialize) -> Self {
        Self { ok: true, result: serde_json::to_value(value).ok(), error: None }
    }
    fn ok_empty() -> Self {
        Self { ok: true, result: None, error: None }
    }
    fn err(e: impl std::fmt::Display) -> Self {
        Self { ok: false, result: None, error: Some(e.to_string()) }
    }
}

/// Satu pintu masuk eksekusi — dipanggil dari CLI, server, atau langsung dari agent loop nanti.
pub fn execute(actions: Vec<AgentAction>) -> Vec<ActionResult> {
    let mut results = Vec::new();
    for action in actions {
        let result = run(action).unwrap_or_else(|e| ActionResult::err(e));
        let failed = !result.ok;
        results.push(result);
        if failed {
            break;
        }
    }
    results
}

fn run(action: AgentAction) -> anyhow::Result<ActionResult> {
    // ini tetep sama persis, nerima SATU AgentAction — gak berubah
    use AgentAction::*;
    Ok(match action {
        Done => ActionResult::ok_empty(),

        ListDir { path, depth } => ActionResult::ok(fs::list(&path, depth)?),
        FindFile { path, pattern } => ActionResult::ok(fs::find(&path, &pattern)?),

        ListProcesses => ActionResult::ok(process::list_process()?),
        KillProcess { pid } => { process::kill_process(pid)?; ActionResult::ok_empty() }
        LaunchProcess { name } => {
            process::launch_process(&name)?;
            ActionResult::ok_empty()
        }

        ListWindows => ActionResult::ok(window::list()?),
        FocusWindow { hwnd } => { window::focus(hwnd)?; ActionResult::ok_empty() }
        MinimizeWindow { hwnd } => { window::minimize(hwnd)?; ActionResult::ok_empty() }
        MaximizeWindow { hwnd } => { window::maximize(hwnd)?; ActionResult::ok_empty() }
        RestoreWindow { hwnd } => { window::restore(hwnd)?; ActionResult::ok_empty() }
        CloseWindow { hwnd } => { window::close(hwnd)?; ActionResult::ok_empty() }

        SystemOverview => ActionResult::ok(sysinfo::overview_collect()?),
        GetVolume => ActionResult::ok(volume::get_volume()?),
        SetVolume { percent } => { volume::set_volume(percent)?; ActionResult::ok_empty() }
        Mute => { volume::mute_volume(true)?; ActionResult::ok_empty() }
        Unmute => { volume::mute_volume(false)?; ActionResult::ok_empty() }

        MoveMouse { x, y } => { input::move_mouse(x, y)?; ActionResult::ok_empty() }
        Click { button } => { input::click(&button)?; ActionResult::ok_empty() }
        Scroll { amount } => { input::scroll(amount)?; ActionResult::ok_empty() }
        TypeText { text } => { input::type_text(&text)?; ActionResult::ok_empty() }
        KeyPress { key } => { input::key_press(&key)?; ActionResult::ok_empty() }
    })
}