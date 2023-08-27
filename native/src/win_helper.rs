extern crate winapi;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr::null_mut;

use winapi::shared::minwindef::{BOOL, FALSE, LPARAM, TRUE};
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, FindWindowExA, GetWindowLongA};

use crate::WIN_SIZE;

pub unsafe fn get_desktop_hwnd() -> *const i8 {
    let mut hDeskTop = FindWindowExA(0 as _, 0 as _, "WorkerW".as_ptr() as *const i8, 0 as _);
    let mut hShellDll;
    while hDeskTop as u32 != 0 {
        hShellDll = FindWindowExA(
            hDeskTop,
            0 as _,
            "SHELLDLL_DefView".as_ptr() as *const i8,
            0 as _,
        );
        hDeskTop = FindWindowExA(0 as _, hDeskTop, "WorkerW".as_ptr() as *const i8, 0 as _);
    }

    hDeskTop as _
}

pub unsafe fn embed_into_hwnd(traywin: *const i8) {
    let druidwin = crate::WIN_HWND.unwrap() as _;

    use winapi::um::winuser::*;

    let mut lStyle = GetWindowLongA(druidwin, GWL_STYLE);
    lStyle &= !(WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU) as i32;
    SetWindowLongA(druidwin, GWL_STYLE, lStyle);

    let mut lExStyle = GetWindowLongA(druidwin, GWL_EXSTYLE);
    lExStyle &= !(WS_EX_DLGMODALFRAME | WS_EX_CLIENTEDGE | WS_EX_WINDOWEDGE) as i32;
    SetWindowLongA(
        druidwin,
        GWL_EXSTYLE,
        (lExStyle as u32 | WS_EX_NOACTIVATE | WS_EX_LAYERED) as i32,
    );
    winapi::um::winuser::SetParent(druidwin, traywin as _);

    // get the position of traywin
    let mut rect = std::mem::zeroed();
    GetWindowRect(traywin as _, &mut rect);

    // print rect
    println!(
        "rect: {:?} {} {} {}",
        WIN_SIZE.0.x as i32 - rect.left,
        WIN_SIZE.0.y as i32 - rect.top,
        WIN_SIZE.1.width as i32,
        WIN_SIZE.1.height as i32
    );

    // if WIN_SIZE.0.x as i32 - rect.left < 0
    //     // || WIN_SIZE.0.y as i32 - rect.top < 0
    //     // || WIN_SIZE.0.y > rect.top as f64 + WIN_SIZE.1.height
    //     || WIN_SIZE.0.x > rect.left as f64 + WIN_SIZE.1.width
    // {
    //     SetWindowPos(
    //         druidwin,
    //         0 as _,
    //         0,
    //         0,
    //         0,
    //         0,
    //         SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
    //     );
    // } else {
        winapi::um::winuser::MoveWindow(
            druidwin,
            WIN_SIZE.0.x as i32 - rect.left,
            3,
            WIN_SIZE.1.width as _,
            WIN_SIZE.1.height as _,
            (SWP_FRAMECHANGED | SWP_NOOWNERZORDER | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE) as i32,
        );
    // }
}
