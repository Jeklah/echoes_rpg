@echo off
title Echoes of the Forgotten Realm - RPG Adventure

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Rust/Cargo not found in PATH
    echo Please install Rust from https://rustup.rs/
    echo.
    pause
    exit /b 1
)

REM Display welcome message
echo =====================================
echo  Echoes of the Forgotten Realm
echo  Cross-Platform RPG Adventure
echo =====================================
echo.
echo Initializing game...
echo.

REM Enable ANSI colors for Windows 10+
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>nul

REM Check if we're in the correct directory
if not exist Cargo.toml (
    echo ERROR: Cargo.toml not found
    echo Please run this script from the echoes_rpg directory
    echo.
    pause
    exit /b 1
)

REM Build and run the game in release mode for better performance
echo Building game (this may take a moment on first run)...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Failed to build the game
    echo Please check the error messages above
    echo.
    echo Common solutions:
    echo - Update Rust: rustup update
    echo - Clean build cache: cargo clean
    echo - Check internet connection for dependencies
    echo.
    pause
    exit /b 1
)

echo.
echo Starting game...
echo.
echo Controls:
echo   Arrow Keys - Move
echo   G - Get items/loot chests
echo   I - Inventory
echo   C - Character stats
echo   Q - Quit
echo.
echo Press any key to start...
pause >nul

REM Clear screen and run the game
cls
cargo run --release

REM Game has ended
echo.
echo Thanks for playing Echoes of the Forgotten Realm!
echo.
pause
