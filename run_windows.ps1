# Echoes of the Forgotten Realm - Windows PowerShell Launcher
# Cross-Platform RPG Adventure Game

param(
    [switch]$Debug,
    [switch]$Clean
)

# Set console title
$Host.UI.RawUI.WindowTitle = "Echoes of the Forgotten Realm - RPG Adventure"

# Function to display colored text
function Write-ColorText {
    param(
        [string]$Text,
        [ConsoleColor]$Color = [ConsoleColor]::White
    )
    Write-Host $Text -ForegroundColor $Color
}

# Function to check if command exists
function Test-Command {
    param([string]$Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

# Display welcome banner
Clear-Host
Write-ColorText "=====================================" -Color Cyan
Write-ColorText " Echoes of the Forgotten Realm" -Color Yellow
Write-ColorText " Cross-Platform RPG Adventure" -Color Yellow
Write-ColorText "=====================================" -Color Cyan
Write-Host ""

# Check if Rust is installed
Write-ColorText "Checking system requirements..." -Color Green

if (-not (Test-Command "cargo")) {
    Write-ColorText "ERROR: Rust/Cargo not found in PATH" -Color Red
    Write-ColorText "Please install Rust from https://rustup.rs/" -Color Yellow
    Write-Host ""
    Write-ColorText "Installation steps:" -Color White
    Write-ColorText "1. Visit https://rustup.rs/" -Color Gray
    Write-ColorText "2. Download and run rustup-init.exe" -Color Gray
    Write-ColorText "3. Follow the installation prompts" -Color Gray
    Write-ColorText "4. Restart PowerShell and try again" -Color Gray
    Write-Host ""
    Read-Host "Press Enter to exit"
    exit 1
}

# Check if we're in the correct directory
if (-not (Test-Path "Cargo.toml")) {
    Write-ColorText "ERROR: Cargo.toml not found" -Color Red
    Write-ColorText "Please run this script from the echoes_rpg directory" -Color Yellow
    Write-Host ""
    Read-Host "Press Enter to exit"
    exit 1
}

# Enable Windows 10+ console features
try {
    # Try to enable ANSI colors and UTF-8 support
    [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
    Write-ColorText "✓ Console features enabled" -Color Green
} catch {
    Write-ColorText "⚠ Could not enable all console features (continuing anyway)" -Color Yellow
}

# Clean build if requested
if ($Clean) {
    Write-ColorText "Cleaning build cache..." -Color Yellow
    cargo clean
    if ($LASTEXITCODE -ne 0) {
        Write-ColorText "Warning: Clean failed, continuing anyway" -Color Yellow
    }
}

# Determine build mode
$BuildMode = if ($Debug) { "" } else { "--release" }
$BuildModeText = if ($Debug) { "debug" } else { "release" }

Write-ColorText "Building game in $BuildModeText mode..." -Color Green
Write-ColorText "(This may take a moment on first run)" -Color Gray
Write-Host ""

# Build the game
$BuildArgs = @("build")
if (-not $Debug) {
    $BuildArgs += "--release"
}

& cargo @BuildArgs

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-ColorText "ERROR: Failed to build the game" -Color Red
    Write-ColorText "Please check the error messages above" -Color Yellow
    Write-Host ""
    Write-ColorText "Common solutions:" -Color White
    Write-ColorText "• Update Rust: rustup update" -Color Gray
    Write-ColorText "• Clean build cache: cargo clean" -Color Gray
    Write-ColorText "• Check internet connection for dependencies" -Color Gray
    Write-ColorText "• Run as administrator if permission issues occur" -Color Gray
    Write-Host ""
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host ""
Write-ColorText "✓ Build successful!" -Color Green
Write-Host ""

# Display game controls
Write-ColorText "Game Controls:" -Color Cyan
Write-ColorText "  Arrow Keys  - Move character" -Color Gray
Write-ColorText "  G          - Get items/loot chests" -Color Gray
Write-ColorText "  I          - Open inventory" -Color Gray
Write-ColorText "  C          - View character stats" -Color Gray
Write-ColorText "  Q          - Quit game" -Color Gray
Write-Host ""
Write-ColorText "Combat Controls:" -Color Cyan
Write-ColorText "  1          - Attack" -Color Gray
Write-ColorText "  2          - Use ability" -Color Gray
Write-ColorText "  3          - Use item" -Color Gray
Write-ColorText "  4          - Flee from combat" -Color Gray
Write-Host ""

# Display system information
Write-ColorText "System Information:" -Color Magenta
Write-ColorText "  OS: $($PSVersionTable.OS)" -Color Gray
Write-ColorText "  PowerShell: $($PSVersionTable.PSVersion)" -Color Gray
Write-ColorText "  Terminal: $($env:TERM)" -Color Gray
Write-Host ""

Write-ColorText "Press any key to start the adventure..." -Color Yellow
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Clear screen and start the game
Clear-Host

# Run the game
Write-ColorText "Starting Echoes of the Forgotten Realm..." -Color Green
Write-Host ""

$RunArgs = @("run")
if (-not $Debug) {
    $RunArgs += "--release"
}

& cargo @RunArgs

# Game has ended
Write-Host ""
if ($LASTEXITCODE -eq 0) {
    Write-ColorText "Thanks for playing Echoes of the Forgotten Realm!" -Color Green
} else {
    Write-ColorText "Game exited with an error (code: $LASTEXITCODE)" -Color Red
}

Write-Host ""
Write-ColorText "Game session ended. Press Enter to close..." -Color Gray
Read-Host
