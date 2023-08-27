$env:LIBCLANG_PATH = 'J:\Program Files (x86)\LLVM\bin\'
Set-Location native;
Try {
    yarn run build;
    cargo build;
    taskkill.exe /f /im cloudmusic.exe
    Start-Sleep 1
    Remove-Item ../RuLyrics.dll
    Copy-Item -Force ./target/debug/RuLyrics.dll ../RuLyrics.dll
    Set-Location ..;
}
Catch {
    Set-Location ..;
}
