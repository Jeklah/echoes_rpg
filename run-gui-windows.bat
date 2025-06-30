@echo off
title Echoes of the Forgotten Realm - GUI Edition
echo.
echo ===============================================
echo  Echoes of the Forgotten Realm - GUI Version
echo ===============================================
echo.
echo Starting native Windows GUI application...
echo.

REM Check if GUI executable exists
if not exist "echoes_rpg_gui.exe" (
    echo ERROR: echoes_rpg_gui.exe not found!
    echo Please ensure the GUI executable is in the same directory as this batch file.
    echo.
    echo To build the GUI version, run:
    echo cargo build --release --target x86_64-pc-windows-gnu --features gui
    echo.
    pause
    exit /b 1
)

echo Game found! Starting Echoes RPG GUI...
echo.
echo Features:
echo - Native Windows application with graphical interface
echo - Text-based gameplay with superior performance
echo - No terminal compatibility issues
echo - Smooth 60+ FPS rendering
echo - Professional Windows UI
echo.
echo Starting GUI application...

REM Start the GUI version
start "" "echoes_rpg_gui.exe"

REM Optional: Wait a moment to see if it starts successfully
timeout /t 2 /nobreak >nul

echo.
echo GUI application launched successfully!
echo If the window doesn't appear, check Windows Defender or antivirus settings.
echo.
pause
