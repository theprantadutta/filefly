# GitHub repo details
$repoOwner = "theprantadutta"
$repoName = "filefly"

# Headers for GitHub API
$headers = @{
    "User-Agent" = "Mozilla/5.0"
}

# Get the latest release version from GitHub API
$latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$repoOwner/$repoName/releases/latest" -Headers $headers
$version = $latestRelease.tag_name

# Check if assets exist
if ($latestRelease.assets -eq $null) {
    Write-Host "No assets found in the latest release."
    exit
}

# Find the download URL for the Windows version
$windowsAssetUrl = $latestRelease.assets | Where-Object { $_.name -like "*windows.exe" } | Select-Object -ExpandProperty browser_download_url

# Check if the Windows asset URL is found
if (-Not $windowsAssetUrl) {
    Write-Host "No Windows executable found in the release assets."
    exit
}

# Define installation path in the user's directory
$installDir = "$HOME\AppData\Local\Programs\filefly"

# Check if directory exists, create if not
if (-Not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir
}

# Download the binary for Windows
$exePath = "$installDir\filefly_v$version.exe"
Write-Host "Downloading Filefly version $version for Windows..."
try {
    Invoke-WebRequest -Uri $windowsAssetUrl -OutFile $exePath -ErrorAction Stop
} catch {
    Write-Host "Error downloading the executable: $_"
    exit
}

# Rename the downloaded file to "filefly.exe" for consistency
$finalExePath = "$installDir\filefly.exe"
if (Test-Path $exePath) {
    # Rename the downloaded file to "filefly.exe" for consistency
    $finalExePath = "$installDir\filefly.exe"
    if (Test-Path $finalExePath) {
        Write-Host "Existing filefly.exe found. Deleting..."
        Remove-Item -Path $finalExePath -Force
    }
    Rename-Item -Path $exePath -NewName $finalExePath
} else {
    Write-Host "Downloaded file not found. Something went wrong."
    exit
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
