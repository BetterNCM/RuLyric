use betterncm_macro::betterncm_native_call;
use betterncm_plugin_api::*;
use cef::CefV8Value;

#[betterncm_native_call]
fn test_func(arg0: usize, arg1: CefV8Value) {
    println!("BetterNCM ❤ Rust!");
    println!("{} {:?}!", arg0, arg1);
    unsafe {
        dbg!(cef_sys::cef_v8context_get_current_context());
    }
    if arg1.is_function() {
        let arg1 = arg1.into_v8function();
        std::thread::spawn(move || {
            println!("Delay Executing function");
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("Executing!");
            arg1.execute_function(&[arg0.into()]);
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("Executing!");
            arg1.execute_function(&[(arg0 * 2).into()]);
        });
    }
}

const FULL_V8VALUE_ARGS: [NativeAPIType; 100] = [NativeAPIType::V8Value; 100];

#[export_name = "BetterNCMPluginMain"]
extern "cdecl" fn betterncm_plugin_main(ctx: &mut PluginContext) -> ::core::ffi::c_int {
    unsafe {
        ctx.add_native_api_raw(
            FULL_V8VALUE_ARGS.as_ptr(),
            2,
            "betterncm-native-plugin-rs-test_func\0".as_ptr() as _,
            test_func,
        );
    }

    println!("BetterNCM Rust Plugin loaded!");

    1
}