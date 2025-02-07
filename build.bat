
@echo off
echo Building GBRust...
cargo build
if %ERRORLEVEL% EQU 0 (
    echo Build successful!
) else (
    echo Build failed!
)
pause