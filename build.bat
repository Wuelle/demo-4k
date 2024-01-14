:: cargo r -Z build-std=core -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --features=sanity --release
:: cargo r -Z build-std=core -Z unstable-options --target x86_64-pc-windows-msvc --features=sanity --release --bin demo
cargo rustc -Z build-std=core -Z unstable-options  --target i686-pc-windows-msvc --release -Z build-std-features=panic_immediate_abort --bin demo -- --emit obj="demo.o"
Crinkler.exe demo.o /OUT:final.exe /SUBSYSTEM:WINDOWS /ENTRY:WinMainCRTStartup "/LIBPATH:C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\um\x86" gdi32.lib user32.lib opengl32.lib kernel32.lib winmm.lib
final.exe
echo %ErrorLevel%

:: -Z build-std-features=panic_immediate_abort