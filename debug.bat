REM filepath: /f:/rust/gbrust/debug.bat
@echo off
setlocal

if "%1"=="" (
    echo Usage: debug.bat ^<rom_file^>
    echo Example: debug.bat rom.gb
    exit /b 1
)

REM Check if ROM file exists
if not exist "%~f1" (
    echo Error: ROM file not found: %1
    echo Make sure the file exists and the path is correct
    pause
    exit /b 1
)

REM Get the directory where the batch file is located
set "SCRIPT_DIR=%~dp0"
cd /d "%SCRIPT_DIR%"

echo Building GBRust...
cargo build
if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    pause
    exit /b 1
)

echo.
echo Starting debugger with ROM: %~f1
echo.
cargo run -- "%~f1"

endlocal