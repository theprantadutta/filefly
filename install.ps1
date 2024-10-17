# GitHub repo details
$repoOwner = "theprantadutta"
$repoName = "filefly"

# Check if running as Administrator
If (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Warning "You do not have Administrator rights to run this script! Please re-run this script as an Administrator."
    Start-Process powershell "-NoProfile -ExecutionPolicy Bypass -File $PSCommandPath" -Verb RunAs
    Exit
}

# Get the latest release version from GitHub API
$latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$repoOwner/$repoName/releases/latest"
$version = $latestRelease.tag_name

# Find the download URL for the Windows version
$windowsAssetUrl = $latestRelease.assets | Where-Object { $_.name -like "*windows.exe" } | Select-Object -ExpandProperty browser_download_url

# Define installation path (use admin install directory if possible)
$installDir = "C:\Program Files\filefly"

# If admin install directory fails, fall back to user directory
if (-Not (Test-Path $installDir)) {
    $installDir = "$HOME\AppData\Local\filefly"
    if (-Not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir
    }
}

# Download the binary for Windows
$exePath = "$installDir\filefly_$version.exe"
Write-Host "Downloading Filefly version $version for Windows..."
Invoke-WebRequest -Uri $windowsAssetUrl -OutFile $exePath

# Optionally add to PATH for current session
$env:Path += ";$installDir"

Write-Host "Filefly v$version installed successfully in $installDir!"
