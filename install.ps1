# GitHub repo details
$repoOwner = "theprantadutta"
$repoName = "filefly"

# Get the latest release version from GitHub API
$latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$repoOwner/$repoName/releases/latest"
$version = $latestRelease.tag_name

# Find the download URL for the Windows version
$windowsAssetUrl = $latestRelease.assets | Where-Object { $_.name -like "*windows.exe" } | Select-Object -ExpandProperty browser_download_url

# Define installation path in the user's directory
$installDir = "$HOME\AppData\Local\Programs\filefly"

# Check if directory exists, create if not
if (-Not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir
}

# Download the binary for Windows
$exePath = "$installDir\filefly_v$version.exe"
Write-Host "Downloading Filefly version $version for Windows..."
Invoke-WebRequest -Uri $windowsAssetUrl -OutFile $exePath

# Rename the downloaded file to "filefly.exe" for consistency
$finalExePath = "$installDir\filefly.exe"
if (-Not (Test-Path $finalExePath)) {
    Rename-Item -Path $exePath -NewName $finalExePath
} else {
    Write-Host "filefly.exe already exists in $installDir. Skipping rename."
}

# Add to user's PATH permanently
$envPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if (-not $envPath.Split(";").Contains($installDir)) {
    [Environment]::SetEnvironmentVariable("Path", "$envPath;$installDir", [EnvironmentVariableTarget]::User)
    Write-Host "Added $installDir to the user's PATH."
} else {
    Write-Host "$installDir is already in the user's PATH."
}

Write-Host "Filefly v$version installed successfully in $installDir!"
