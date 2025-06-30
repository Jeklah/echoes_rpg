# Echoes of the Forgotten Realm - Advanced Windows PowerShell Launcher
# This script provides a GUI-like experience for launching the game

param(
    [switch]$Debug,
    [switch]$SkipBuild,
    [switch]$ForceRebuild
)

# Set strict mode for better error handling
Set-StrictMode -Version 3.0

# Import required assemblies for GUI elements
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

# Console window configuration
$Host.UI.RawUI.WindowTitle = "Echoes of the Forgotten Realm - Launcher"

# Function to display colored text
function Write-ColorOutput {
    param(
        [string]$Text,
        [ConsoleColor]$ForegroundColor = [ConsoleColor]::White,
        [ConsoleColor]$BackgroundColor = [ConsoleColor]::Black,
        [switch]$NoNewline
    )

    $currentFg = $Host.UI.RawUI.ForegroundColor
    $currentBg = $Host.UI.RawUI.BackgroundColor

    $Host.UI.RawUI.ForegroundColor = $ForegroundColor
    $Host.UI.RawUI.BackgroundColor = $BackgroundColor

    if ($NoNewline) {
        Write-Host $Text -NoNewline
    } else {
        Write-Host $Text
    }

    $Host.UI.RawUI.ForegroundColor = $currentFg
    $Host.UI.RawUI.BackgroundColor = $currentBg
}

# Function to show GUI message box
function Show-MessageBox {
    param(
        [string]$Message,
        [string]$Title = "Echoes RPG",
        [System.Windows.Forms.MessageBoxButtons]$Buttons = [System.Windows.Forms.MessageBoxButtons]::OK,
        [System.Windows.Forms.MessageBoxIcon]$Icon = [System.Windows.Forms.MessageBoxIcon]::Information
    )

    return [System.Windows.Forms.MessageBox]::Show($Message, $Title, $Buttons, $Icon)
}

# Function to show progress dialog
function Show-ProgressDialog {
    param([string]$Title, [string]$Message)

    $form = New-Object System.Windows.Forms.Form
    $form.Text = $Title
    $form.Size = New-Object System.Drawing.Size(400, 150)
    $form.StartPosition = "CenterScreen"
    $form.FormBorderStyle = "FixedDialog"
    $form.MaximizeBox = $false
    $form.MinimizeBox = $false

    $label = New-Object System.Windows.Forms.Label
    $label.Location = New-Object System.Drawing.Point(20, 20)
    $label.Size = New-Object System.Drawing.Size(360, 40)
    $label.Text = $Message
    $label.Font = New-Object System.Drawing.Font("Segoe UI", 10)

    $progressBar = New-Object System.Windows.Forms.ProgressBar
    $progressBar.Location = New-Object System.Drawing.Point(20, 70)
    $progressBar.Size = New-Object System.Drawing.Size(340, 20)
    $progressBar.Style = "Marquee"
    $progressBar.MarqueeAnimationSpeed = 30

    $form.Controls.Add($label)
    $form.Controls.Add($progressBar)

    $form.Show()
    $form.Refresh()

    return $form
}

# Function to check prerequisites
function Test-Prerequisites {
    Write-ColorOutput "Checking prerequisites..." -ForegroundColor Yellow

    # Check if Rust is installed
    try {
        $cargoVersion = & cargo --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✓ Rust/Cargo found: $cargoVersion" -ForegroundColor Green
        } else {
            throw "Cargo not found"
        }
    } catch {
        Write-ColorOutput "✗ Rust/Cargo not found" -ForegroundColor Red

        $result = Show-MessageBox -Message "Rust is required to run this game but was not found on your system.`n`nWould you like to open the Rust installation page?" -Title "Rust Not Found" -Buttons YesNo -Icon Warning

        if ($result -eq [System.Windows.Forms.DialogResult]::Yes) {
            Start-Process "https://rustup.rs/"
        }

        return $false
    }

    # Check if we're in the correct directory
    if (-not (Test-Path "Cargo.toml")) {
        Write-ColorOutput "✗ Game files not found" -ForegroundColor Red
        Show-MessageBox -Message "This launcher must be placed in the echoes_rpg directory alongside Cargo.toml and the src folder." -Title "Game Files Not Found" -Icon Error
        return $false
    }

    Write-ColorOutput "✓ Game files found" -ForegroundColor Green

    # Check terminal capabilities
    try {
        # Test if we can resize console window
        $currentSize = $Host.UI.RawUI.WindowSize
        if ($currentSize.Width -lt 100 -or $currentSize.Height -lt 25) {
            try {
                $newSize = $currentSize
                $newSize.Width = [Math]::Max(120, $currentSize.Width)
                $newSize.Height = [Math]::Max(30, $currentSize.Height)
                $Host.UI.RawUI.WindowSize = $newSize
                Write-ColorOutput "✓ Console window resized for optimal experience" -ForegroundColor Green
            } catch {
                Write-ColorOutput "⚠ Could not resize console window" -ForegroundColor Yellow
            }
        }
    } catch {
        Write-ColorOutput "⚠ Limited terminal control available" -ForegroundColor Yellow
    }

    return $true
}

# Function to build the game
function Build-Game {
    param([bool]$IsDebug, [bool]$ForceClean)

    if ($ForceClean) {
        Write-ColorOutput "Cleaning build cache..." -ForegroundColor Yellow
        & cargo clean
    }

    $buildMode = if ($IsDebug) { "debug" } else { "release" }
    $buildArgs = @("build")
    if (-not $IsDebug) {
        $buildArgs += "--release"
    }

    Write-ColorOutput "Building game in $buildMode mode..." -ForegroundColor Yellow
    Write-ColorOutput "(This may take a few minutes on first run)" -ForegroundColor Gray

    # Show progress dialog for long-running build
    $progressForm = Show-ProgressDialog -Title "Building Game" -Message "Compiling Echoes of the Forgotten Realm...`nThis may take a few minutes on first run."

    try {
        $buildOutput = & cargo @buildArgs 2>&1
        $buildSuccess = $LASTEXITCODE -eq 0

        $progressForm.Close()
        $progressForm.Dispose()

        if ($buildSuccess) {
            Write-ColorOutput "✓ Build completed successfully!" -ForegroundColor Green
            return $true
        } else {
            Write-ColorOutput "✗ Build failed" -ForegroundColor Red
            Write-ColorOutput "Build output:" -ForegroundColor Yellow
            $buildOutput | ForEach-Object { Write-ColorOutput $_ -ForegroundColor Gray }

            Show-MessageBox -Message "Failed to build the game. Please check the console output for details.`n`nCommon solutions:`n• Update Rust: rustup update`n• Clean build cache: cargo clean`n• Check internet connection`n• Run as administrator" -Title "Build Failed" -Icon Error
            return $false
        }
    } catch {
        $progressForm.Close()
        $progressForm.Dispose()
        Write-ColorOutput "✗ Build process failed: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

# Function to display game information
function Show-GameInfo {
    Clear-Host

    # ASCII Art Banner
    Write-ColorOutput @"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║    ███████╗ ██████╗██╗  ██╗ ██████╗ ███████╗███████╗        ║
    ║    ██╔════╝██╔════╝██║  ██║██╔═══██╗██╔════╝██╔════╝        ║
    ║    █████╗  ██║     ███████║██║   ██║█████╗  ███████╗        ║
    ║    ██╔══╝  ██║     ██╔══██║██║   ██║██╔══╝  ╚════██║        ║
    ║    ███████╗╚██████╗██║  ██║╚██████╔╝███████╗███████║        ║
    ║    ╚══════╝ ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝        ║
    ║                                                              ║
    ║                 OF THE FORGOTTEN REALM                       ║
    ║              Cross-Platform RPG Adventure                    ║
    ╚══════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

    Write-Host ""
    Write-ColorOutput "Welcome to Echoes of the Forgotten Realm!" -ForegroundColor Yellow
    Write-Host ""

    # System Information
    Write-ColorOutput "System Information:" -ForegroundColor Magenta
    Write-ColorOutput "  OS: $($PSVersionTable.OS)" -ForegroundColor Gray
    Write-ColorOutput "  PowerShell: $($PSVersionTable.PSVersion)" -ForegroundColor Gray
    Write-ColorOutput "  Architecture: $($env:PROCESSOR_ARCHITECTURE)" -ForegroundColor Gray
    Write-ColorOutput "  User: $($env:USERNAME)" -ForegroundColor Gray
    Write-Host ""

    # Game Controls
    Write-ColorOutput "Game Controls:" -ForegroundColor Cyan
    Write-ColorOutput "  Arrow Keys  - Move character" -ForegroundColor Gray
    Write-ColorOutput "  G          - Get items / Loot chests" -ForegroundColor Gray
    Write-ColorOutput "  I          - Open inventory" -ForegroundColor Gray
    Write-ColorOutput "  C          - Character stats" -ForegroundColor Gray
    Write-ColorOutput "  Q          - Quit game" -ForegroundColor Gray
    Write-Host ""

    Write-ColorOutput "Combat Controls:" -ForegroundColor Cyan
    Write-ColorOutput "  1          - Attack" -ForegroundColor Gray
    Write-ColorOutput "  2          - Use ability" -ForegroundColor Gray
    Write-ColorOutput "  3          - Use item" -ForegroundColor Gray
    Write-ColorOutput "  4          - Flee from combat" -ForegroundColor Gray
    Write-Host ""
}

# Function to run the game
function Start-Game {
    param([bool]$IsDebug)

    Write-ColorOutput "Starting Echoes of the Forgotten Realm..." -ForegroundColor Green
    Write-Host ""

    $runArgs = @("run")
    if (-not $IsDebug) {
        $runArgs += "--release"
    }

    try {
        & cargo @runArgs
        $gameExitCode = $LASTEXITCODE

        Write-Host ""
        if ($gameExitCode -eq 0) {
            Write-ColorOutput "Thanks for playing Echoes of the Forgotten Realm!" -ForegroundColor Green
        } else {
            Write-ColorOutput "Game exited with code: $gameExitCode" -ForegroundColor Yellow
            if ($gameExitCode -eq 130) {
                Write-ColorOutput "Game was interrupted (Ctrl+C)" -ForegroundColor Gray
            }
        }
    } catch {
        Write-ColorOutput "Error starting game: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Main execution
try {
    # Show initial information
    Show-GameInfo

    # Check prerequisites
    if (-not (Test-Prerequisites)) {
        Write-Host ""
        Write-ColorOutput "Press any key to exit..." -ForegroundColor Gray
        $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
        exit 1
    }

    Write-Host ""

    # Check if we need to build
    $needsBuild = $true
    $exePath = if ($Debug) { "target\debug\echoes_rpg.exe" } else { "target\release\echoes_rpg.exe" }

    if ($SkipBuild) {
        if (Test-Path $exePath) {
            $needsBuild = $false
            Write-ColorOutput "Skipping build (using existing executable)" -ForegroundColor Yellow
        } else {
            Write-ColorOutput "Executable not found, build required despite --SkipBuild flag" -ForegroundColor Yellow
        }
    }

    # Build the game if needed
    if ($needsBuild -or $ForceRebuild) {
        if (-not (Build-Game -IsDebug $Debug -ForceClean $ForceRebuild)) {
            Write-Host ""
            Write-ColorOutput "Press any key to exit..." -ForegroundColor Gray
            $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
            exit 1
        }
    }

    Write-Host ""
    Write-ColorOutput "Press any key to begin your adventure..." -ForegroundColor Yellow
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

    # Clear screen and start game
    Clear-Host
    Start-Game -IsDebug $Debug

} catch {
    Write-ColorOutput "Unexpected error: $($_.Exception.Message)" -ForegroundColor Red
    Write-ColorOutput "Stack trace: $($_.ScriptStackTrace)" -ForegroundColor Gray
} finally {
    Write-Host ""
    Write-ColorOutput "Launcher session ended. Press any key to close..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}
