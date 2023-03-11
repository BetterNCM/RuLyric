#[macro_use]
extern crate lazy_static;

static mut DATA_SENDER: Option<ExtEventSink> = None;
pub static mut WIN_HWND: Option<DWORD> = None;

use std::{
    collections::HashMap,
    fs,
    iter::{once, Map},
    os::windows::prelude::OsStrExt,
    ptr::{null, null_mut},
    sync::{Arc, Mutex},
};

use betterncm_macro::betterncm_native_call;
use betterncm_plugin_api::*;
use cef::CefV8Value;
use cef_sys::DWORD;
use druid::{
    AppDelegate, AppLauncher, Color, Command, DelegateCtx, Env, ExtEventSink, FontWeight, Handled,
    HasRawWindowHandle, RawWindowHandle, Selector, Target, WindowDesc, WindowHandle, WindowId,
};
use raw_window_handle_5::Win32WindowHandle;
use winapi::um::synchapi::Sleep;

use crate::{
    lyrics_app::{ui_builder, LyricAppData, LyricWinData},
    model::{
        font::FontConfig,
        lyrics::{LyricsData, LyricsWord},
    },
    win_helper::{embed_into_hwnd, get_desktop_hwnd},
};
mod lyrics_app;
mod model;
mod widgets;
mod win_helper;

struct Delegate {
    handles: HashMap<WindowId, WindowHandle>,
}

lazy_static! {
    pub static ref WIN_SIZE: (druid::Point,druid::Size) = {
        // attempt to read from %AppData%/.betterncm.rulyrics.lastpos.conf
        let path = format!(
            "{}\\.betterncm.rulyrics.lastpos.conf",
            std::env::var("APPDATA").unwrap()
        );
        if let Ok(content) = fs::read_to_string(path) {
            let mut iter = content.split_whitespace();
            let x = iter.next().unwrap().parse::<f64>().unwrap();
            let y = iter.next().unwrap().parse::<f64>().unwrap();
            let w = iter.next().unwrap().parse::<f64>().unwrap();
            let h = iter.next().unwrap().parse::<f64>().unwrap();
            (druid::Point::new(x, y), druid::Size::new(w, h))
        } else {
            (druid::Point::new(0.0, 0.0), druid::Size::new(400.0, 70.0))
        }
    };
}

impl AppDelegate<LyricAppData> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut LyricAppData,
        _env: &Env,
    ) -> Handled {
        if let Some(winid) = cmd.get(Selector::<usize>::new("CREATE_WINDOW")) {
            for (key, val) in &self.handles {
                val.close();
            }

            ctx.new_window(
                WindowDesc::new(ui_builder(*winid))
                    .show_titlebar(false)
                    .transparent(true)
                    .set_position(WIN_SIZE.0)
                    .window_size(WIN_SIZE.1),
            );
            Handled::Yes
        } else {
            Handled::No
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        handle: WindowHandle,
        data: &mut LyricAppData,
        env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        unsafe {
            if let RawWindowHandle::Win32(handle) = handle.raw_window_handle() {
                crate::WIN_HWND = Some(handle.hwnd as _);
            }
        }
        self.handles.insert(id, handle);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut LyricAppData,
        env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        self.handles.remove(&id);
    }
}

fn edit_data(callback: impl FnOnce(&mut LyricAppData) + Send + std::marker::Sync + 'static) {
    unsafe {
        if let Some(sink) = DATA_SENDER.as_ref() {
            sink.add_idle_callback(|data: &mut LyricAppData| callback(data));
        }
    }
}

#[betterncm_native_call]
fn init_lyrics_app(
    font_family: CefV8Value,
    font_size: CefV8Value,
    font_color: CefV8Value,
    font_weight: CefV8Value,
    font_background_color: CefV8Value,

    font_family_s: CefV8Value,
    font_size_s: CefV8Value,
    font_color_s: CefV8Value,
    font_weight_s: CefV8Value,
    font_background_color_s: CefV8Value,
) {
    fn get_font_conf_from_v8(
        font_family: CefV8Value,
        font_size: CefV8Value,
        font_color: CefV8Value,
        font_weight: CefV8Value,
        font_background_color: CefV8Value,
    ) -> FontConfig {
        FontConfig {
            font_family: font_family.get_string_value().to_string(),
            font_size: font_size.get_double_value(),
            font_color: druid::Color::from_hex_str(
                font_color.get_string_value().to_string().as_str(),
            )
            .unwrap(),
            font_weight: FontWeight::new(font_weight.get_uint_value() as u16),
            font_background_color: druid::Color::from_hex_str(
                font_background_color
                    .get_string_value()
                    .to_string()
                    .as_str(),
            )
            .unwrap(),
        }
    }

    let win_data = LyricWinData {
        font: get_font_conf_from_v8(
            font_family,
            font_size,
            font_color,
            font_weight,
            font_background_color,
        ),
        font_secondary: get_font_conf_from_v8(
            font_family_s,
            font_size_s,
            font_color_s,
            font_weight_s,
            font_background_color_s,
        ),
        with_words_lyrics: false,
    };

    if unsafe { DATA_SENDER.is_none() } {
        std::thread::spawn(|| {
            let main_window = WindowDesc::new(ui_builder(0))
                .show_titlebar(false)
                .transparent(true)
                .set_position(WIN_SIZE.0)
                .window_size(WIN_SIZE.1);

            let app = AppLauncher::with_window(main_window)
                .delegate(Delegate {
                    handles: HashMap::new(),
                })
                .log_to_console();
            unsafe {
                DATA_SENDER = Some(app.get_external_handle());
            }

            app.launch(LyricAppData {
                current_lyric: LyricsData::new_test("".to_string()),
                current_lyric_ext: LyricsData::new_test("".to_string()),
                win_data: vec![win_data],
            })
        });
    } else {
        edit_data(|data| {
            data.win_data[0] = win_data;
            // let _ = DATA_SENDER.as_ref().unwrap().submit_command(
            //     Selector::new("CREATE_WINDOW"),
            //     data.win_data.len() - 1,
            //     Target::Global,
            // );
        });
    }
}

#[betterncm_native_call]
fn update_lyrics(line: CefV8Value, line_ext: CefV8Value, seek: CefV8Value) {
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_epoch_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    let seek: i128 = if seek.is_double() {
        unsafe { seek.get_double_value().to_int_unchecked() }
    } else {
        0
    };

    if line.is_string() {
        let line = line.get_string_value().to_string();
        edit_data(move |data: &mut LyricAppData| {
            data.current_lyric = LyricsData::new_test(line);
            data.current_lyric_ext = LyricsData::new_test("".to_string());
        });
    } else if line.is_object() {
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

        if line_ext.is_string() {
            let line_ext = line_ext.get_string_value().to_string();
            edit_data(move |data: &mut LyricAppData| {
                data.current_lyric = LyricsData::from_lyrics(lyrics, line_num.try_into().unwrap());

                if line_ext.len() > 0 {
                    data.current_lyric_ext = LyricsData::from_text_duration(
                        line_ext,
                        data.current_lyric.get_full_duration(),
                    );
                } else {
                    data.current_lyric_ext = LyricsData::new_test("".to_string());
                }

                data.current_lyric.start_time = seek as u64;
                data.current_lyric_ext.start_time = seek as u64;
            });
        } else {
            edit_data(move |data: &mut LyricAppData| {
                data.current_lyric = LyricsData::from_lyrics(lyrics, line_num.try_into().unwrap());

                data.current_lyric.start_time = seek as u64;
                data.current_lyric_ext = LyricsData::new_test("".to_string());
            });
        }
    }
}

#[betterncm_native_call]
fn embed_into_taskbar() {
    embed_into_with_classname(&"Shell_TrayWnd".to_string());
}

#[betterncm_native_call]
fn embed_into_desktop() {
    unsafe {
        embed_into_hwnd(get_desktop_hwnd());
    }
}

fn embed_into_with_classname(class_name: &String) {
    unsafe {
        use std::ptr::null_mut;
        let wide: Vec<u16> = std::ffi::OsStr::new(class_name)
            .encode_wide()
            .chain(once(0))
            .collect();
        let traywin = winapi::um::winuser::FindWindowW(wide.as_ptr(), null_mut());
        embed_into_hwnd(traywin as _);
    }
}

#[betterncm_native_call]
fn embed_into_any(class_name: CefV8Value) {
    embed_into_with_classname(&class_name.get_string_value().to_string());
}

#[betterncm_native_call]
fn seek(time: CefV8Value, paused: CefV8Value) {
    let time = time.get_uint_value() as u64;

    let paused = paused.get_bool_value();
    edit_data(move |data: &mut LyricAppData| {
        if time != 0 {
            data.current_lyric.start_time = time;
            data.current_lyric_ext.start_time = time;
        }
        data.current_lyric.paused = paused;
        data.current_lyric_ext.paused = paused;
    });
}

const FULL_V8VALUE_ARGS: [NativeAPIType; 100] = [NativeAPIType::V8Value; 100];

#[export_name = "BetterNCMPluginMain"]
extern "cdecl" fn betterncm_plugin_main(ctx: &mut PluginContext) -> ::core::ffi::c_int {
    unsafe {
        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            3,
            "rulyrics.update_lyrics\0".as_ptr() as _,
            update_lyrics,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            10,
            "rulyrics.init_lyrics_app\0".as_ptr() as _,
            init_lyrics_app,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            0,
            "rulyrics.embed_into_taskbar\0".as_ptr() as _,
            embed_into_taskbar,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            0,
            "rulyrics.embed_into_desktop\0".as_ptr() as _,
            embed_into_desktop,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            1,
            "rulyrics.embed_into_any\0".as_ptr() as _,
            embed_into_any,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            2,
            "rulyrics.seek\0".as_ptr() as _,
            seek,
        );
    }

    println!("BetterNCM Rust Plugin loaded!");

    1
}
