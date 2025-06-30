@echo off
REM Echoes of the Forgotten Realm - Windows GUI Launcher
REM This batch file can be double-clicked to launch the game in a new window

setlocal enabledelayedexpansion

REM Set window title
title Echoes of the Forgotten Realm - Launcher

REM Change to the directory where this batch file is located
cd /d "%~dp0"

REM Check if we're already in a new console window
if "%ECHOES_LAUNCHED%"=="1" goto :skip_relaunch

REM Launch in a new console window with better settings
set ECHOES_LAUNCHED=1
start "Echoes of the Forgotten Realm" cmd /c ""%~f0" && pause"
exit

:skip_relaunch

REM Clear the screen
cls

REM Set console window properties for better experience
mode con: cols=120 lines=30

REM Enable ANSI colors (Windows 10+)
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>&1

REM Display banner
echo.
echo =====================================
echo  Echoes of the Forgotten Realm
echo  Cross-Platform RPG Adventure
echo =====================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Rust/Cargo not found in PATH
    echo.
    echo Please install Rust from: https://rustup.rs/
    echo.
    echo Installation steps:
    echo 1. Visit https://rustup.rs/
    echo 2. Download and run rustup-init.exe
    echo 3. Follow the installation prompts
    echo 4. Restart this launcher
    echo.
    echo Press any key to open the Rust installation page...
    pause >nul
    start https://rustup.rs/
    exit /b 1
)

REM Check if we're in the correct directory
if not exist Cargo.toml (
    echo [ERROR] Game files not found
    echo This launcher must be placed in the echoes_rpg directory
    echo alongside Cargo.toml and the src folder.
    echo.
    pause
    exit /b 1
)

REM Display system info
echo System Information:
echo   OS: %OS%
echo   Processor: %PROCESSOR_ARCHITECTURE%
echo   User: %USERNAME%
echo.

REM Check if this is first run (needs compilation)
if not exist "target\release\echoes_rpg.exe" (
    echo This appears to be your first time running the game.
    echo The game will be compiled for optimal performance.
    echo This may take a few minutes...
    echo.
)

REM Build the game in release mode
echo Building game...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Failed to build the game
    echo.
    echo Common solutions:
    echo - Update Rust: rustup update
    echo - Clean build cache: cargo clean
    echo - Check internet connection
    echo - Run as administrator
    echo.
    pause
    exit /b 1
)

echo.
echo [SUCCESS] Game built successfully!
echo.

REM Display game controls
echo Game Controls:
echo   Arrow Keys  - Move character
echo   G          - Get items / Loot chests
echo   I          - Open inventory
echo   C          - Character stats
echo   Q          - Quit game
echo.
echo Combat Controls:
echo   1          - Attack
echo   2          - Use ability
echo   3          - Use item
echo   4          - Flee
echo.

REM Final prompt before starting
echo Press any key to begin your adventure...
pause >nul

REM Clear screen and start the game
cls
echo Starting Echoes of the Forgotten Realm...
echo.

REM Run the game
cargo run --release

REM Game has ended - show exit message
echo.
if %ERRORLEVEL% EQU 0 (
    echo Thanks for playing Echoes of the Forgotten Realm!
) else (
    echo Game exited with error code: %ERRORLEVEL%
    echo.
    echo If you encounter issues:
    echo - Try running as administrator
    echo - Check Windows Defender exclusions
    echo - Update Windows and graphics drivers
)

echo.
echo Press any key to close this window...
pause >nul
