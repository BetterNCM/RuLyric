$env:LIBCLANG_PATH = 'J:\Program Files\LLVM\bin\'

Set-Location native;
cargo +nightly build --target i686-pc-windows-msvc -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release;
Copy-Item -Force ./target/i686-pc-windows-msvc/release/RuLyrics.dll ../RuLyrics.dll
Set-Location ..;