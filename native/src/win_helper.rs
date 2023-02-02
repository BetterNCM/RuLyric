extern crate winapi;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr::null_mut;

use winapi::shared::minwindef::{BOOL, FALSE, LPARAM, TRUE};
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, FindWindowExA, GetWindowLongA};

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
    let wide: Vec<u16> = std::ffi::OsStr::new("druid")
        .encode_wide()
        .chain(once(0))
        .collect();
    let druidwin = winapi::um::winuser::FindWindowW(wide.as_ptr(), null_mut());

    use winapi::um::winuser::*;

    let mut lStyle = GetWindowLongA(druidwin, GWL_STYLE);
    lStyle &= !(WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU) as i32;
    SetWindowLongA(druidwin, GWL_STYLE, lStyle);

    let mut lExStyle = GetWindowLongA(druidwin, GWL_EXSTYLE);
    lExStyle &= !(WS_EX_DLGMODALFRAME | WS_EX_CLIENTEDGE | WS_EX_WINDOWEDGE) as i32;
    SetWindowLongA(
        druidwin,
        GWL_EXSTYLE,
        lExStyle | WS_EX_NOACTIVATE | WS_EX_LAYERED as i32,
    );
    winapi::um::winuser::SetParent(druidwin, traywin as _);

    winapi::um::winuser::MoveWindow(
        druidwin,
        20,
        3,
        400,
        70,
        (SWP_FRAMECHANGED | SWP_NOSIZE | SWP_NOZORDER | SWP_NOOWNERZORDER) as i32,
    );
}
