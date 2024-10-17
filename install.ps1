# GitHub repo details
$repoOwner = "theprantadutta"
$repoName = "filefly"

# Get the latest release version from GitHub API
$latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$repoOwner/$repoName/releases/latest"
$version = $latestRelease.tag_name

# Find the download URL for the Windows version
$windowsAssetUrl = $latestRelease.assets | Where-Object { $_.name -like "*windows.exe" } | Select-Object -ExpandProperty browser_download_url

# Define installation path in user directory
$installDir = "$HOME\AppData\Local\filefly"

# Check if directory exists, create if not
if (-Not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir
}

# Download the binary for Windows
$exePath = "$installDir\filefly_$version.exe"
Write-Host "Downloading Filefly version $version for Windows..."
Invoke-WebRequest -Uri $windowsAssetUrl -OutFile $exePath

# Add to PATH for current session
$env:Path += ";$installDir"

Write-Host "Filefly v$version installed successfully in $installDir!"
