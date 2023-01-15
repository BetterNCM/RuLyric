$env:LIBCLANG_PATH = 'J:\Program Files (x86)\LLVM\bin\'

Set-Location native;
cargo build;
Copy-Item ./target/debug/RuLyrics.dll ../RuLyrics.dll
Set-Location ..;