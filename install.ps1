# PMSynapse CLI Installer for Windows
# https://github.com/HelixoidLLC/pmsynapse

$ErrorActionPreference = "Stop"

$Repo = "HelixoidLLC/pmsynapse"
$BinaryName = "snps.exe"
$InstallDir = "$env:USERPROFILE\bin"

Write-Host "╔═══════════════════════════════════════╗" -ForegroundColor Blue
Write-Host "║   PMSynapse CLI Installer             ║" -ForegroundColor Blue
Write-Host "╚═══════════════════════════════════════╝" -ForegroundColor Blue
Write-Host ""

# Detect architecture
$Arch = "amd64"  # Windows installers typically x64
if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") {
    Write-Host "Error: ARM64 Windows not supported yet" -ForegroundColor Red
    exit 1
}

$AssetName = "snps-windows-$Arch.exe"
$Archive = "$AssetName.zip"
$DownloadUrl = "https://github.com/$Repo/releases/latest/download/$Archive"
$ChecksumUrl = "$DownloadUrl.sha256"

Write-Host "Detected: windows-$Arch" -ForegroundColor Green
Write-Host "Installing $BinaryName to $InstallDir...`n"

# Create temp directory
$TempDir = New-Item -ItemType Directory -Path "$env:TEMP\pmsynapse-install-$(Get-Random)" -Force

try {
    Set-Location $TempDir

    # Download archive
    Write-Host "[1/4] Downloading $Archive..." -ForegroundColor Blue
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $Archive -UseBasicParsing

    # Download checksum
    Write-Host "[2/4] Verifying checksum..." -ForegroundColor Blue
    Invoke-WebRequest -Uri $ChecksumUrl -OutFile "$Archive.sha256" -UseBasicParsing

    # Verify checksum
    $ExpectedHash = (Get-Content "$Archive.sha256" -Raw).Split()[0]
    $ActualHash = (Get-FileHash -Algorithm SHA256 $Archive).Hash

    if ($ExpectedHash -ne $ActualHash) {
        Write-Host "Error: Checksum verification failed" -ForegroundColor Red
        Write-Host "Expected: $ExpectedHash" -ForegroundColor Red
        Write-Host "Got:      $ActualHash" -ForegroundColor Red
        exit 1
    }

    # Extract
    Write-Host "[3/4] Extracting..." -ForegroundColor Blue
    Expand-Archive -Path $Archive -DestinationPath . -Force

    # Create install directory if needed
    if (-not (Test-Path $InstallDir)) {
        Write-Host "Creating directory: $InstallDir" -ForegroundColor Yellow
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    # Install
    Write-Host "[4/4] Installing to $InstallDir..." -ForegroundColor Blue
    Move-Item -Path $BinaryName -Destination "$InstallDir\$BinaryName" -Force

    # Add to PATH if not already there
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        Write-Host "Adding $InstallDir to PATH..." -ForegroundColor Yellow
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$UserPath;$InstallDir",
            "User"
        )
        $env:Path = "$env:Path;$InstallDir"
        Write-Host "Note: Restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    }

    Write-Host ""
    Write-Host "✓ Installation successful!" -ForegroundColor Green
    Write-Host ""

    # Verify installation
    $SnpsPath = "$InstallDir\$BinaryName"
    if (Test-Path $SnpsPath) {
        & $SnpsPath --version
        Write-Host ""
        Write-Host "Get started:" -ForegroundColor Green
        Write-Host "  snps --help"
        Write-Host "  snps thoughts init"
        Write-Host "  snps daemon start"
    }
}
finally {
    # Cleanup
    Set-Location $env:TEMP
    Remove-Item -Recurse -Force $TempDir -ErrorAction SilentlyContinue
}
