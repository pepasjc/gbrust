@echo off
echo Running GBRust tests...
cargo test
if %ERRORLEVEL% EQU 0 (
    echo All tests passed!
) else (
    echo Tests failed!
)
pause
