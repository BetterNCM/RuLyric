#[macro_use]
extern crate lazy_static;

static mut DataSender: Option<ExtEventSink> = None;

use betterncm_macro::betterncm_native_call;
use betterncm_plugin_api::*;
use cef::{CefString, CefV8Value};
use druid::{AppLauncher, ExtEventSink, WindowDesc};

use crate::lyrics_app::{ui_builder, LyricAppData};
mod lyrics_app;
mod model;
mod widgets;

#[betterncm_native_call]
fn init_lyrics_app() {
    std::thread::spawn(|| {
        let main_window = WindowDesc::new(ui_builder())
            .show_titlebar(false)
            .transparent(true)
            .resizable(false)
            .window_size((400.0, 70.0));

        let app = AppLauncher::with_window(main_window).log_to_console();
        unsafe {
            DataSender = Some(app.get_external_handle());
        }

        app.launch(LyricAppData {
            current_lyric: "".to_string(),
        })
    });
}

#[betterncm_native_call]
fn update_lyrics(line: CefV8Value) {
    if line.is_string() {
        let line = line.get_string_value().to_string();
        unsafe {
            DataSender.clone().unwrap().add_idle_callback(|data:&mut LyricAppData|{
                data.current_lyric=line;
            });
        }
    }
}

const FULL_V8VALUE_ARGS: [NativeAPIType; 100] = [NativeAPIType::V8Value; 100];

#[export_name = "BetterNCMPluginMain"]
extern "cdecl" fn betterncm_plugin_main(ctx: &mut PluginContext) -> ::core::ffi::c_int {
    unsafe {
        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            1,
            "rulyrics.update_lyrics\0".as_ptr() as _,
            update_lyrics,
        );

        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            0,
            "rulyrics.init_lyrics_app\0".as_ptr() as _,
            init_lyrics_app,
        );
    }

    println!("BetterNCM Rust Plugin loaded!");

    1
}
