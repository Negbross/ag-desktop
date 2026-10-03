use serde::{Deserialize, Serialize};
use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM, TRUE, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
    SetForegroundWindow, ShowWindow, PostMessageW,
    SW_MINIMIZE, SW_MAXIMIZE, SW_RESTORE, WM_CLOSE,
};

#[derive(Serialize, Deserialize)]
pub struct WinInfo {
    pub hwnd: isize,
    pub title: String,
    pub pid: u32,
}

/// List semua window yang visible & punya judul
pub fn list() -> anyhow::Result<Vec<WinInfo>> {
    let mut out: Vec<WinInfo> = Vec::new();

    unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            if !IsWindowVisible(hwnd).as_bool() {
                return TRUE;
            }
            let mut buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buf);
            if len == 0 {
                return TRUE;
            }
            let title = String::from_utf16_lossy(&buf[..len as usize]);

            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));

            let vec = &mut *(lparam.0 as *mut Vec<WinInfo>);
            vec.push(WinInfo { hwnd: hwnd.0 as isize, title, pid });
        }
        TRUE
    }

    unsafe {
        let _ = EnumWindows(Some(callback), LPARAM(&mut out as *mut _ as isize));
    }
    Ok(out)
}

pub fn focus(hwnd_raw: isize) -> anyhow::Result<()> {
    let hwnd = HWND(hwnd_raw as *mut _);
    unsafe {
        let _ = SetForegroundWindow(hwnd);
    }
    Ok(())
}

pub fn minimize(hwnd_raw: isize) -> anyhow::Result<()> {
    let hwnd = HWND(hwnd_raw as *mut _);
    unsafe {
        let _ = ShowWindow(hwnd, SW_MINIMIZE);
    }
    Ok(())
}

pub fn maximize(hwnd_raw: isize) -> anyhow::Result<()> {
    let hwnd = HWND(hwnd_raw as *mut _);
    unsafe {
        let _ = ShowWindow(hwnd, SW_MAXIMIZE);
    }
    Ok(())
}

pub fn restore(hwnd_raw: isize) -> anyhow::Result<()> {
    let hwnd = HWND(hwnd_raw as *mut _);
    unsafe {
        let _ = ShowWindow(hwnd, SW_RESTORE);
    }
    Ok(())
}

pub fn close(hwnd_raw: isize) -> anyhow::Result<()> {
    let hwnd = HWND(hwnd_raw as *mut _);
    unsafe {
        let _ = PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0));
    }
    Ok(())
}