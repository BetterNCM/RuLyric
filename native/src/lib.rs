#[macro_use]
extern crate lazy_static;

static mut DATA_SENDER: Option<ExtEventSink> = None;
pub static mut WIN_HWND: Option<DWORD> = None;

use std::{iter::once, os::windows::prelude::OsStrExt};

use betterncm_macro::betterncm_native_call;
use betterncm_plugin_api::*;
use cef::CefV8Value;
use cef_sys::DWORD;
use druid::{AppLauncher, ExtEventSink, WindowDesc};

use crate::{
    lyrics_app::{ui_builder, LyricAppData},
    model::lyrics::{LyricsData, LyricsWord},
};
mod lyrics_app;
mod model;
mod widgets;

#[betterncm_native_call]
fn init_lyrics_app() {
    std::thread::spawn(|| {
        let main_window = WindowDesc::new(ui_builder())
            .show_titlebar(false)
            .transparent(true)
            .window_size((400.0, 70.0));

        let app = AppLauncher::with_window(main_window).log_to_console();
        unsafe {
            DATA_SENDER = Some(app.get_external_handle());
        }

        app.launch(LyricAppData {
            current_lyric: LyricsData::new_test("".to_string()),
        })
    });
}

#[betterncm_native_call]
fn update_lyrics(line: CefV8Value, _line_ext: CefV8Value) {
    if line.is_string() {
        let line = line.get_string_value().to_string();
        unsafe {
            DATA_SENDER
                .clone()
                .unwrap()
                .add_idle_callback(|data: &mut LyricAppData| {
                    data.current_lyric = LyricsData::new_test(line);
                });
        }
    } else if line.is_object() {
        unsafe {
            let line_num = line.get_value_byindex(1).get_uint_value();
            let words = line.get_value_byindex(0);
            let mut lyrics = vec![];
            for i in 0..words.get_array_length() {
                let val = words.get_value_byindex(i as isize);
                lyrics.push(LyricsWord {
                    lyric_word: val.get_value_byindex(0).get_string_value().to_string(),
                    lyric_duration: val.get_value_byindex(1).get_uint_value() as u64,
                });
            }

            DATA_SENDER
                .clone()
                .unwrap()
                .add_idle_callback(move |data: &mut LyricAppData| {
                    data.current_lyric =
                        LyricsData::from_lyrics(lyrics, line_num.try_into().unwrap());
                });
        }
    }
}

#[betterncm_native_call]
fn embed_into_taskbar() {
    unsafe {
        use std::ptr::null_mut;
        let wide: Vec<u16> = std::ffi::OsStr::new("Shell_TrayWnd")
            .encode_wide()
            .chain(once(0))
            .collect();
        let traywin = winapi::um::winuser::FindWindowW(wide.as_ptr(), null_mut());

        let wide: Vec<u16> = std::ffi::OsStr::new("druid")
            .encode_wide()
            .chain(once(0))
            .collect();
        let druidwin = winapi::um::winuser::FindWindowW(wide.as_ptr(), null_mut());

        use winapi::um::winuser::*;

        let mut lStyle = GetWindowLongA(druidwin, GWL_STYLE);
        lStyle &=
            !(WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU) as i32;
        SetWindowLongA(druidwin, GWL_STYLE, lStyle);

        let mut lExStyle = GetWindowLongA(druidwin, GWL_EXSTYLE);
        lExStyle &= !(WS_EX_DLGMODALFRAME | WS_EX_CLIENTEDGE | WS_EX_WINDOWEDGE) as i32;
        SetWindowLongA(druidwin, GWL_EXSTYLE, lExStyle | WS_EX_NOACTIVATE as i32);
        winapi::um::winuser::MoveWindow(druidwin, 20, 10, 400, 70, 0);

        winapi::um::winuser::SetParent(druidwin, traywin);
    }
}

const FULL_V8VALUE_ARGS: [NativeAPIType; 100] = [NativeAPIType::V8Value; 100];

#[export_name = "BetterNCMPluginMain"]
extern "cdecl" fn betterncm_plugin_main(ctx: &mut PluginContext) -> ::core::ffi::c_int {
    unsafe {
        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            2,
            "rulyrics.update_lyrics\0".as_ptr() as _,
            update_lyrics,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            0,
            "rulyrics.init_lyrics_app\0".as_ptr() as _,
            init_lyrics_app,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            0,
            "rulyrics.embed_into_taskbar\0".as_ptr() as _,
            embed_into_taskbar,
        );
    }

    println!("BetterNCM Rust Plugin loaded!");

    1
}
